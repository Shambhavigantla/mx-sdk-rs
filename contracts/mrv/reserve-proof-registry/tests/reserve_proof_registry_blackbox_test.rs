use multiversx_sc_scenario::imports::*;

use reserve_proof_registry::ReserveProofRegistryProxy;

const OWNER: TestAddress = TestAddress::new("owner");
const GOVERNANCE: TestAddress = TestAddress::new("governance");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("reserve-proof-registry");
const CODE_PATH: MxscPath = MxscPath::new("mxsc:output/reserve-proof-registry.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());
    blockchain.set_current_dir_from_workspace("contracts/mrv/reserve-proof-registry");
    blockchain.register_contract(CODE_PATH, reserve_proof_registry::ContractBuilder);
    blockchain
}

/// Deploy and initialize the reserve proof registry contract.
#[test]
fn reserve_proof_registry_blackbox_init() {
    let mut world = world();

    world.account(OWNER).nonce(1).balance(1_000_000u64);

    // Deploy
    world
        .tx()
        .from(OWNER)
        .typed(ReserveProofRegistryProxy)
        .init(GOVERNANCE.to_managed_address())
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .run();
}