use multiversx_sc_scenario::imports::*;

use gsoc_registry::GsocRegistryProxy;

const OWNER: TestAddress = TestAddress::new("owner");
const GOVERNANCE: TestAddress = TestAddress::new("governance");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("gsoc-registry");
const CODE_PATH: MxscPath = MxscPath::new("mxsc:output/gsoc-registry.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());
    blockchain.set_current_dir_from_workspace("contracts/mrv/gsoc-registry");
    blockchain.register_contract(CODE_PATH, gsoc_registry::ContractBuilder);
    blockchain
}

/// Deploy and initialize the gsoc registry contract.
#[test]
fn gsoc_registry_blackbox_init() {
    let mut world = world();

    world.account(OWNER).nonce(1).balance(1_000_000u64);

    // Deploy
    world
        .tx()
        .from(OWNER)
        .typed(GsocRegistryProxy)
        .init(GOVERNANCE.to_managed_address())
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .run();
}

/// Reserve a serial.
#[test]
fn gsoc_registry_blackbox_reserve_serial() {
    let mut world = world();

    world.account(OWNER).nonce(1).balance(1_000_000u64);

    // Deploy
    world
        .tx()
        .from(OWNER)
        .typed(GsocRegistryProxy)
        .init(GOVERNANCE.to_managed_address())
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .run();

    // Reserve serial
    world
        .tx()
        .from(OWNER)
        .to(SC_ADDRESS)
        .typed(GsocRegistryProxy)
        .reserve_serial(ManagedBuffer::from(b"SERIAL-001"))
        .returns(ReturnsResult)
        .run();
}