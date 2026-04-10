use multiversx_sc_scenario::imports::*;

use mrv_registry::mrv_registry_proxy::MrvRegistryProxy;

const OWNER: TestAddress = TestAddress::new("owner");
const GOVERNANCE: TestAddress = TestAddress::new("governance");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("mrv-registry");
const CODE_PATH: MxscPath = MxscPath::new("mxsc:output/mrv-registry.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());
    blockchain.set_current_dir_from_workspace("contracts/mrv/registry");
    blockchain.register_contract(CODE_PATH, mrv_registry::ContractBuilder);
    blockchain
}

/// Deploy and initialize the registry contract.
#[test]
fn mrv_registry_blackbox_init() {
    let mut world = world();

    world.account(OWNER).nonce(1).balance(1_000_000u64);

    // Deploy
    world
        .tx()
        .from(OWNER)
        .typed(MrvRegistryProxy)
        .init(GOVERNANCE.to_managed_address())
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .run();
}

/// Register a methodology.
#[test]
fn mrv_registry_blackbox_register_methodology() {
    let mut world = world();

    world.account(OWNER).nonce(1).balance(1_000_000u64);

    // Deploy
    world
        .tx()
        .from(OWNER)
        .typed(MrvRegistryProxy)
        .init(GOVERNANCE.to_managed_address())
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .run();

    // Register methodology
    world
        .tx()
        .from(OWNER)
        .to(SC_ADDRESS)
        .typed(MrvRegistryProxy)
        .register_methodology(
            ManagedBuffer::from(b"METHOD-001"),
            ManagedBuffer::from(b"v1.0"),
            ManagedBuffer::from(b"digest123"),
            ManagedBuffer::from(b"approved"),
            1000000u64,
            2000000u64,
        )
        .run();
}