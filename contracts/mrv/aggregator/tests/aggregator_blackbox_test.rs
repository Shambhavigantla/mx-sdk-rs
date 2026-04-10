use multiversx_sc_scenario::imports::*;

use mrv_aggregator::mrv_aggregator_proxy::MrvAggregatorProxy;

const OWNER: TestAddress = TestAddress::new("owner");
const ORACLE: TestAddress = TestAddress::new("oracle");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("mrv-aggregator");
const CODE_PATH: MxscPath = MxscPath::new("mxsc:output/mrv-aggregator.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());
    blockchain.set_current_dir_from_workspace("contracts/mrv/aggregator");
    blockchain.register_contract(CODE_PATH, mrv_aggregator::ContractBuilder);
    blockchain
}

/// Deploy and initialize the aggregator contract.
#[test]
fn aggregator_blackbox_init() {
    let mut world = world();

    world.account(OWNER).nonce(1).balance(1_000_000u64);

    // Deploy
    world
        .tx()
        .from(OWNER)
        .typed(MrvAggregatorProxy)
        .init(2u32, 172800u64, 864000u64, 2592000u64, 3000u64)
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .run();

    // TODO: Add queries to verify init values if views are available
}

/// Register an oracle and submit a reading.
#[test]
fn aggregator_blackbox_register_oracle_and_submit() {
    let mut world = world();

    world.account(OWNER).nonce(1).balance(1_000_000u64);
    world.account(ORACLE).nonce(1).balance(1_000_000u64);

    // Deploy
    world
        .tx()
        .from(OWNER)
        .typed(MrvAggregatorProxy)
        .init(2u32, 172800u64, 864000u64, 2592000u64, 3000u64)
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .run();

    // Register oracle
    world
        .tx()
        .from(OWNER)
        .to(SC_ADDRESS)
        .typed(MrvAggregatorProxy)
        .register_oracle(ORACLE.to_managed_address())
        .run();

    // Submit reading from oracle
    world
        .tx()
        .from(ORACLE)
        .to(SC_ADDRESS)
        .typed(MrvAggregatorProxy)
        .submit_oracle_reading(
            ManagedBuffer::from(b"PAI-001"),
            1000000u64,
            10086400u64,
            0u8, // IoT
            ManagedBuffer::from(b"cid123"),
            1000000u64,
            ORACLE.to_managed_address(),
            ManagedBuffer::from(b"signature"),
        )
        .run();

    // TODO: Add more tests for sealing, etc.
}