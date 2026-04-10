use multiversx_sc_scenario::imports::*;

use come_settlement::come_settlement_proxy::ComeSettlementProxy;

const OWNER: TestAddress = TestAddress::new("owner");
const GOVERNANCE: TestAddress = TestAddress::new("governance");
const FROM: TestAddress = TestAddress::new("from");
const TO: TestAddress = TestAddress::new("to");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("come-settlement");
const CODE_PATH: MxscPath = MxscPath::new("mxsc:output/come-settlement.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());
    blockchain.set_current_dir_from_workspace("contracts/mrv/come-settlement");
    blockchain.register_contract(CODE_PATH, come_settlement::ContractBuilder);
    blockchain
}

/// Deploy and initialize the come settlement contract.
#[test]
fn come_settlement_blackbox_init() {
    let mut world = world();

    world.account(OWNER).nonce(1).balance(1_000_000u64);

    // Deploy
    world
        .tx()
        .from(OWNER)
        .typed(ComeSettlementProxy)
        .init(GOVERNANCE.to_managed_address())
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .run();
}

/// Create a settlement.
#[test]
fn come_settlement_blackbox_create_settlement() {
    let mut world = world();

    world.account(OWNER).nonce(1).balance(1_000_000u64);

    // Deploy
    world
        .tx()
        .from(OWNER)
        .typed(ComeSettlementProxy)
        .init(GOVERNANCE.to_managed_address())
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .run();

    // Create settlement
    world
        .tx()
        .from(OWNER)
        .to(SC_ADDRESS)
        .typed(ComeSettlementProxy)
        .create_settlement(
            ManagedBuffer::from(b"SETTLEMENT-001"),
            FROM.to_managed_address(),
            TO.to_managed_address(),
            TokenIdentifier::from(b"COME-123"),
            1000u64.into(),
            ManagedBuffer::from(b"reason123"),
        )
        .run();
}