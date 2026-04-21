use multiversx_sc_scenario::{imports::*, ScenarioTxRun};

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/mrv/aggregator");
    blockchain.register_contract("file:output/mrv-aggregator.wasm", mrv_aggregator::ContractBuilder);
    blockchain.register_contract("mxsc:output/mrv-aggregator.mxsc.json", mrv_aggregator::ContractBuilder);
    blockchain
}

#[test]
fn generate_aggregator_trace() {
    let mut world = world();
    let owner = TestAddress::new("owner");
    let contract = TestSCAddress::new("aggregator_contract");
    world.account(owner).nonce(1).balance(1_000_000u64);

    let mock_device = TestAddress::new("iot_device_01");

    world.start_trace();

    world.tx().from(owner).raw_deploy()
        .argument(&2u32)             // quorum
        .argument(&172_800u64)       // iot_window
        .argument(&864_000u64)       // satellite_window
        .argument(&2_592_000u64)     // govt_lab_window
        .argument(&3_000u64)         // divergence_threshold_bps
        .code(MxscPath::new("output/mrv-aggregator.mxsc.json"))
        .new_address(contract)
        .run();

    // Interact: Register Device
    world.tx().from(owner).to(contract).raw_call("registerDevice")
        .argument(&mock_device.to_address()) 
        .run();

    world.write_scenario_trace("scenarios/aggregator-lifecycle.scen.json");
}