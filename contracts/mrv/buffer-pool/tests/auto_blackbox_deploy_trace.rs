use multiversx_sc_scenario::{imports::*, ScenarioTxRun};

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/mrv/buffer-pool");
    blockchain.register_contract("file:output/mrv-buffer-pool.wasm", mrv_buffer_pool::ContractBuilder);
    blockchain.register_contract("mxsc:output/mrv-buffer-pool.mxsc.json", mrv_buffer_pool::ContractBuilder);
    blockchain
}

#[test]
fn generate_buffer_pool_trace() {
    let mut world = world();
    let owner = TestAddress::new("owner");
    let contract = TestSCAddress::new("buffer_pool_contract");
    world.account(owner).nonce(1).balance(1_000_000u64);

    let mock_gov = TestAddress::new("mock_gov");
    let mock_carbon = TestAddress::new("mock_carbon");

    world.start_trace();

    world.tx().from(owner).raw_deploy()
        .argument(&mock_gov.to_address())
        .argument(&mock_carbon.to_address())
        .code(MxscPath::new("output/mrv-buffer-pool.mxsc.json"))
        .new_address(contract)
        .run();

    // Interact: Deposit Buffer Credits
    world.tx().from(owner).to(contract).raw_call("depositBufferCredits")
        .argument(&b"PROJ-001")      // project_id
        .argument(&1_000u64)         // amount_scaled
        .argument(&1u64)             // monitoring_period_n
        .run();

    world.write_scenario_trace("scenarios/buffer-pool-lifecycle.scen.json");
}