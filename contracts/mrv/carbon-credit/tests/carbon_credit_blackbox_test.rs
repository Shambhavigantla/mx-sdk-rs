use multiversx_sc_scenario::imports::*;

use carbon_credit::carbon_credit_proxy::CarbonCreditModuleProxy;

const OWNER: TestAddress = TestAddress::new("owner");
const GOVERNANCE: TestAddress = TestAddress::new("governance");
const BUFFER_POOL: TestSCAddress = TestSCAddress::new("buffer-pool");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("carbon-credit");
const CODE_PATH: MxscPath = MxscPath::new("mxsc:output/carbon-credit.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());
    blockchain.set_current_dir_from_workspace("contracts/mrv/carbon-credit");
    blockchain.register_contract(CODE_PATH, carbon_credit::ContractBuilder);
    blockchain
}

/// Deploy and initialize the carbon credit contract.
#[test]
fn carbon_credit_blackbox_init() {
    let mut world = world();

    world.account(OWNER).nonce(1).balance(1_000_000u64);

    // Deploy
    world
        .tx()
        .from(OWNER)
        .typed(CarbonCreditModuleProxy)
        .init(GOVERNANCE.to_managed_address(), BUFFER_POOL.to_managed_address())
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .run();
}