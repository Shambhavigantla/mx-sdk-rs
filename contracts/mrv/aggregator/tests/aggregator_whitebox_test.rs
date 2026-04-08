use mrv_aggregator::MrvAggregator;
use multiversx_sc::types::ManagedBuffer;
use multiversx_sc_scenario::imports::*;

const OWNER: TestAddress = TestAddress::new("owner");
const ORACLE_ONE: TestAddress = TestAddress::new("oracle-one");
const ORACLE_TWO: TestAddress = TestAddress::new("oracle-two");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("mrv-aggregator");
const CODE_PATH: MxscPath = MxscPath::new("mxsc:output/mrv-aggregator.mxsc.json");

fn world() -> ScenarioWorld {
    let mut world = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());
    world.set_current_dir_from_workspace("contracts/mrv/aggregator");
    world.register_contract(CODE_PATH, mrv_aggregator::ContractBuilder);
    world
}

#[test]
fn aggregator_init_rs() {
    let mut world = world();

    world.account(OWNER).nonce(1).balance(1_000_000u64);

    world
        .tx()
        .from(OWNER)
        .raw_deploy()
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .whitebox(mrv_aggregator::contract_obj, |sc| {
            sc.init(2u32, 172800u64, 864000u64, 2592000u64, 3000u64);
        });

    world
        .query()
        .to(SC_ADDRESS)
        .whitebox(mrv_aggregator::contract_obj, |sc| {
            assert_eq!(sc.quorum().get(), 2u32);
            assert_eq!(sc.iot_window().get(), 172800u64);
            assert_eq!(sc.satellite_window().get(), 864000u64);
            assert_eq!(sc.govt_lab_window().get(), 2592000u64);
            assert_eq!(sc.divergence_threshold_bps().get(), 3000u64);
        });
}

#[test]
fn aggregator_submit_oracle_reading_and_try_seal_rs() {
    let mut world = world();

    world.account(OWNER).nonce(1).balance(1_000_000u64);
    world.account(ORACLE_ONE).nonce(1).balance(1_000_000u64);
    world.account(ORACLE_TWO).nonce(1).balance(1_000_000u64);

    world
        .tx()
        .from(OWNER)
        .raw_deploy()
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .whitebox(mrv_aggregator::contract_obj, |sc| {
            sc.init(2u32, 172800u64, 864000u64, 2592000u64, 3000u64);
        });

    // F-09(R3): Register oracles before submitting readings
    world
        .tx()
        .from(OWNER)
        .to(SC_ADDRESS)
        .whitebox(mrv_aggregator::contract_obj, |sc| {
            sc.register_oracle(ORACLE_ONE.to_managed_address());
            sc.register_oracle(ORACLE_TWO.to_managed_address());
            // Register devices so submit_oracle_reading passes DEVICE_NOT_REGISTERED guard
            sc.register_device(ORACLE_ONE.to_managed_address());
            sc.register_device(ORACLE_TWO.to_managed_address());
        });

    // Set block timestamp so oracle readings are not rejected as FUTURE_TIMESTAMP
    world.current_block().block_timestamp(1_710_800_000u64);

    // Submit IoT reading (source=0)
    world
        .tx()
        .from(ORACLE_ONE)
        .to(SC_ADDRESS)
        .whitebox(mrv_aggregator::contract_obj, |sc| {
            sc.submit_oracle_reading(
                ManagedBuffer::from(b"pai-001"),
                1_710_600_000u64,  // period_start
                1_710_720_000u64,  // period_end
                0u8, // SOURCE_IOT
                ManagedBuffer::from(b"bafyiot001"),
                1_710_719_000u64,
                ORACLE_ONE.to_managed_address(),
                ManagedBuffer::from(b"sig-iot-001"),  // device_signature (non-empty for IoT)
            );
        });

    // Submit Satellite reading (source=1)
    world
        .tx()
        .from(ORACLE_TWO)
        .to(SC_ADDRESS)
        .whitebox(mrv_aggregator::contract_obj, |sc| {
            sc.submit_oracle_reading(
                ManagedBuffer::from(b"pai-001"),
                1_710_600_000u64,  // period_start
                1_710_720_000u64,  // period_end
                1u8, // SOURCE_SATELLITE
                ManagedBuffer::from(b"bafysat001"),
                1_710_710_000u64,
                ORACLE_TWO.to_managed_address(),
                ManagedBuffer::new(),  // device_signature (empty OK for non-IoT)
            );
        });

    // F-03(R4): Set block timestamp past period_end
    world.current_block().block_timestamp(1_710_720_001u64);

    // F-04(R4): Acknowledge semantic discrepancy (IoT != Satellite CIDs)
    world
        .tx()
        .from(OWNER)
        .to(SC_ADDRESS)
        .whitebox(mrv_aggregator::contract_obj, |sc| {
            sc.acknowledge_discrepancy(
                ManagedBuffer::from(b"pai-001"),
                1_710_720_000u64,
                ManagedBuffer::from(b"vvb-ack-cid-001"),
            );
        });

    // Seal with quorum=2 met
    world
        .tx()
        .from(OWNER)
        .to(SC_ADDRESS)
        .whitebox(mrv_aggregator::contract_obj, |sc| {
            sc.try_seal(
                ManagedBuffer::from(b"pai-001"),
                1_710_720_000u64,
            );
        });

    world
        .query()
        .to(SC_ADDRESS)
        .whitebox(mrv_aggregator::contract_obj, |sc| {
            assert!(sc.is_sealed(ManagedBuffer::from(b"pai-001"), 1_710_720_000u64));
            let sealed = sc
                .get_sealed_event(ManagedBuffer::from(b"pai-001"), 1_710_720_000u64)
                .into_option()
                .unwrap();
            assert_eq!(sealed.reading_count, 2u32);
            // IoT CID != Satellite CID => semantic_discrepancy = true
            assert!(sealed.semantic_discrepancy);
        });
}

#[test]
fn aggregator_rejects_seal_below_quorum_rs() {
    let mut world = world();

    world.account(OWNER).nonce(1).balance(1_000_000u64);
    world.account(ORACLE_ONE).nonce(1).balance(1_000_000u64);

    world
        .tx()
        .from(OWNER)
        .raw_deploy()
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .whitebox(mrv_aggregator::contract_obj, |sc| {
            sc.init(2u32, 172800u64, 864000u64, 2592000u64, 3000u64);
        });

    world.tx().from(OWNER).to(SC_ADDRESS).whitebox(mrv_aggregator::contract_obj, |sc| {
        sc.register_oracle(ORACLE_ONE.to_managed_address());
        sc.register_device(ORACLE_ONE.to_managed_address());
    });

    // Set block timestamp so oracle readings are not rejected as FUTURE_TIMESTAMP
    world.current_block().block_timestamp(1_710_800_000u64);

    // Submit only 1 reading — below quorum of 2
    world
        .tx()
        .from(ORACLE_ONE)
        .to(SC_ADDRESS)
        .whitebox(mrv_aggregator::contract_obj, |sc| {
            sc.submit_oracle_reading(
                ManagedBuffer::from(b"pai-002"),
                1_710_600_000u64,  // period_start
                1_710_720_000u64,  // period_end
                0u8,
                ManagedBuffer::from(b"bafyiot002"),
                1_710_719_000u64,
                ORACLE_ONE.to_managed_address(),
                ManagedBuffer::from(b"sig-iot-002"),
            );
        });

    // F-03(R4): Set timestamp past period_end so we reach the quorum check
    world.current_block().block_timestamp(1_710_720_001u64);

    world
        .tx()
        .from(OWNER)
        .to(SC_ADDRESS)
        .returns(ExpectError(4u64, "insufficient oracle readings for quorum"))
        .whitebox(mrv_aggregator::contract_obj, |sc| {
            sc.try_seal(
                ManagedBuffer::from(b"pai-002"),
                1_710_720_000u64,
            );
        });
}

#[test]
fn aggregator_force_seal_after_timeout_rs() {
    let mut world = world();

    world.account(OWNER).nonce(1).balance(1_000_000u64);
    world.account(ORACLE_ONE).nonce(1).balance(1_000_000u64);

    world
        .tx()
        .from(OWNER)
        .raw_deploy()
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .whitebox(mrv_aggregator::contract_obj, |sc| {
            sc.init(2u32, 172800u64, 864000u64, 2592000u64, 3000u64);
        });

    world.tx().from(OWNER).to(SC_ADDRESS).whitebox(mrv_aggregator::contract_obj, |sc| {
        sc.register_oracle(ORACLE_ONE.to_managed_address());
        sc.register_device(ORACLE_ONE.to_managed_address());
    });

    // Set block timestamp so oracle readings are not rejected as FUTURE_TIMESTAMP
    world.current_block().block_timestamp(1_710_800_000u64);

    let period_end: u64 = 1_710_720_000;

    // Submit only 1 IoT reading (below quorum of 2)
    world
        .tx()
        .from(ORACLE_ONE)
        .to(SC_ADDRESS)
        .whitebox(mrv_aggregator::contract_obj, |sc| {
            sc.submit_oracle_reading(
                ManagedBuffer::from(b"pai-003"),
                period_end - 200_000u64,  // period_start
                period_end,
                0u8, // SOURCE_IOT
                ManagedBuffer::from(b"bafyiot003"),
                period_end - 100u64,
                ORACLE_ONE.to_managed_address(),
                ManagedBuffer::from(b"sig-iot-003"),
            );
        });

    // Set block timestamp past period_end + govt_lab_window (2592000)
    world.current_block().block_timestamp(period_end + 2_592_001u64);

    // Force seal should succeed with 1 reading after timeout
    world
        .tx()
        .from(OWNER)
        .to(SC_ADDRESS)
        .whitebox(mrv_aggregator::contract_obj, |sc| {
            sc.force_seal_after_timeout(
                ManagedBuffer::from(b"pai-003"),
                period_end,
            );
        });

    world
        .query()
        .to(SC_ADDRESS)
        .whitebox(mrv_aggregator::contract_obj, |sc| {
            assert!(sc.is_sealed(ManagedBuffer::from(b"pai-003"), period_end));
            let sealed = sc
                .get_sealed_event(ManagedBuffer::from(b"pai-003"), period_end)
                .into_option()
                .unwrap();
            assert_eq!(sealed.reading_count, 1u32);
            // P0-3 fix: single-source force-seal conservatively flags discrepancy
            assert!(sealed.semantic_discrepancy);
        });
}

#[test]
fn aggregator_force_seal_before_timeout_fails_rs() {
    let mut world = world();

    world.account(OWNER).nonce(1).balance(1_000_000u64);
    world.account(ORACLE_ONE).nonce(1).balance(1_000_000u64);

    world
        .tx()
        .from(OWNER)
        .raw_deploy()
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .whitebox(mrv_aggregator::contract_obj, |sc| {
            sc.init(2u32, 172800u64, 864000u64, 2592000u64, 3000u64);
        });

    world.tx().from(OWNER).to(SC_ADDRESS).whitebox(mrv_aggregator::contract_obj, |sc| {
        sc.register_oracle(ORACLE_ONE.to_managed_address());
        sc.register_device(ORACLE_ONE.to_managed_address());
    });

    // Set block timestamp so oracle readings are not rejected as FUTURE_TIMESTAMP
    world.current_block().block_timestamp(1_710_800_000u64);

    let period_end: u64 = 1_710_720_000;

    world
        .tx()
        .from(ORACLE_ONE)
        .to(SC_ADDRESS)
        .whitebox(mrv_aggregator::contract_obj, |sc| {
            sc.submit_oracle_reading(
                ManagedBuffer::from(b"pai-004"),
                period_end - 200_000u64,  // period_start
                period_end,
                0u8,
                ManagedBuffer::from(b"bafyiot004"),
                period_end - 100u64,
                ORACLE_ONE.to_managed_address(),
                ManagedBuffer::from(b"sig-iot-004"),
            );
        });

    // Set timestamp after period_end but before timeout window
    world.current_block().block_timestamp(period_end + 1_000u64);

    world
        .tx()
        .from(OWNER)
        .to(SC_ADDRESS)
        .returns(ExpectError(
            4u64,
            "timeout window has not elapsed \u{2014} wait for coherence window to expire",
        ))
        .whitebox(mrv_aggregator::contract_obj, |sc| {
            sc.force_seal_after_timeout(
                ManagedBuffer::from(b"pai-004"),
                period_end,
            );
        });
}

const NEW_ORACLE: TestAddress = TestAddress::new("new-oracle");

#[test]
fn aggregator_oracle_rotation_lifecycle_rs() {
    let mut world = world();

    world.account(OWNER).nonce(1).balance(1_000_000u64);
    world.account(ORACLE_ONE).nonce(1).balance(1_000_000u64);
    world.account(NEW_ORACLE).nonce(1).balance(1_000_000u64);

    world
        .tx()
        .from(OWNER)
        .raw_deploy()
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .whitebox(mrv_aggregator::contract_obj, |sc| {
            sc.init(2u32, 172800u64, 864000u64, 2592000u64, 3000u64);
        });

    world.tx().from(OWNER).to(SC_ADDRESS).whitebox(mrv_aggregator::contract_obj, |sc| {
        sc.register_oracle(ORACLE_ONE.to_managed_address());
    });

    // Propose oracle rotation
    world.tx().from(OWNER).to(SC_ADDRESS).whitebox(mrv_aggregator::contract_obj, |sc| {
        sc.propose_oracle_update(
            ORACLE_ONE.to_managed_address(),
            NEW_ORACLE.to_managed_address(),
            100_000u64,
        );
    });

    // New oracle accepts
    world.tx().from(NEW_ORACLE).to(SC_ADDRESS).whitebox(mrv_aggregator::contract_obj, |sc| {
        sc.accept_oracle_update(ORACLE_ONE.to_managed_address());
    });

    // Verify old oracle is deregistered and new one is active
    world.query().to(SC_ADDRESS).whitebox(mrv_aggregator::contract_obj, |sc| {
        assert!(!sc.is_oracle_authorized(ORACLE_ONE.to_managed_address()));
        assert!(sc.is_oracle_authorized(NEW_ORACLE.to_managed_address()));
    });
}

const DEVICE_ONE: TestAddress = TestAddress::new("device-one");

#[test]
fn aggregator_device_registration_rs() {
    let mut world = world();

    world.account(OWNER).nonce(1).balance(1_000_000u64);
    world.account(DEVICE_ONE).nonce(1).balance(1_000_000u64);

    world
        .tx()
        .from(OWNER)
        .raw_deploy()
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .whitebox(mrv_aggregator::contract_obj, |sc| {
            sc.init(2u32, 172800u64, 864000u64, 2592000u64, 3000u64);
        });

    world.tx().from(OWNER).to(SC_ADDRESS).whitebox(mrv_aggregator::contract_obj, |sc| {
        sc.register_device(DEVICE_ONE.to_managed_address());
    });

    world.query().to(SC_ADDRESS).whitebox(mrv_aggregator::contract_obj, |sc| {
        assert!(sc.is_device_registered(DEVICE_ONE.to_managed_address()));
    });

    world.tx().from(OWNER).to(SC_ADDRESS).whitebox(mrv_aggregator::contract_obj, |sc| {
        sc.deregister_device(DEVICE_ONE.to_managed_address());
    });

    world.query().to(SC_ADDRESS).whitebox(mrv_aggregator::contract_obj, |sc| {
        assert!(!sc.is_device_registered(DEVICE_ONE.to_managed_address()));
    });
}

#[test]
fn aggregator_duplicate_reading_rejection_rs() {
    let mut world = world();

    world.account(OWNER).nonce(1).balance(1_000_000u64);
    world.account(ORACLE_ONE).nonce(1).balance(1_000_000u64);

    world
        .tx()
        .from(OWNER)
        .raw_deploy()
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .whitebox(mrv_aggregator::contract_obj, |sc| {
            sc.init(2u32, 172800u64, 864000u64, 2592000u64, 3000u64);
        });

    world.tx().from(OWNER).to(SC_ADDRESS).whitebox(mrv_aggregator::contract_obj, |sc| {
        sc.register_oracle(ORACLE_ONE.to_managed_address());
        sc.register_device(ORACLE_ONE.to_managed_address());
    });

    // Set block timestamp so oracle readings are not rejected as FUTURE_TIMESTAMP
    world.current_block().block_timestamp(1_710_800_000u64);

    // Submit first IoT reading
    world
        .tx()
        .from(ORACLE_ONE)
        .to(SC_ADDRESS)
        .whitebox(mrv_aggregator::contract_obj, |sc| {
            sc.submit_oracle_reading(
                ManagedBuffer::from(b"pai-dup"),
                1_710_600_000u64,
                1_710_720_000u64,
                0u8,
                ManagedBuffer::from(b"bafydup001"),
                1_710_719_000u64,
                ORACLE_ONE.to_managed_address(),
                ManagedBuffer::from(b"sig-dup-001"),
            );
        });

    // Submit duplicate reading for same source/period
    world
        .tx()
        .from(ORACLE_ONE)
        .to(SC_ADDRESS)
        .returns(ExpectError(4u64, "READING_ALREADY_SUBMITTED: reading already exists for this source/period"))
        .whitebox(mrv_aggregator::contract_obj, |sc| {
            sc.submit_oracle_reading(
                ManagedBuffer::from(b"pai-dup"),
                1_710_600_000u64,
                1_710_720_000u64,
                0u8,
                ManagedBuffer::from(b"bafydup002"),
                1_710_719_000u64,
                ORACLE_ONE.to_managed_address(),
                ManagedBuffer::from(b"sig-dup-002"),
            );
        });
}
