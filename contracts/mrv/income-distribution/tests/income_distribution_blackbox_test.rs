use multiversx_sc_scenario::imports::*;

use income_distribution::IncomeDistributionProxy;

const OWNER: TestAddress = TestAddress::new("owner");
const GOVERNANCE: TestAddress = TestAddress::new("governance");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("income-distribution");
const CODE_PATH: MxscPath = MxscPath::new("mxsc:output/income-distribution.mxsc.json");
const COME_TOKEN: &[u8] = b"COME-123456";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());
    blockchain.set_current_dir_from_workspace("contracts/mrv/income-distribution");
    blockchain.register_contract(CODE_PATH, income_distribution::ContractBuilder);
    blockchain
}

/// Deploy and initialize the income distribution contract.
#[test]
fn income_distribution_blackbox_init() {
    let mut world = world();

    world.account(OWNER).nonce(1).balance(1_000_000u64);

    // Deploy
    world
        .tx()
        .from(OWNER)
        .typed(IncomeDistributionProxy)
        .init(GOVERNANCE.to_managed_address(), TokenIdentifier::from(COME_TOKEN))
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .run();
}