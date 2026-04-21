use multiversx_sc_scenario::{imports::*, ScenarioTxRun};

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/mrv/gsoc-registry");
    blockchain.register_contract("file:output/mrv-gsoc-registry.wasm", mrv_gsoc_registry::ContractBuilder);
    blockchain.register_contract("mxsc:output/mrv-gsoc-registry.mxsc.json", mrv_gsoc_registry::ContractBuilder);
    blockchain
}

#[test]
fn generate_gsoc_registry_trace() {
    let mut world = world();
    let owner = TestAddress::new("owner");
    let contract = TestSCAddress::new("gsoc_registry_contract");
    let mock_gov = TestAddress::new("mock_gov");

      world.start_trace();
    world.account(owner).nonce(1).balance(1_000_000u64);


    world.tx().from(owner).raw_deploy()
        .argument(&mock_gov.to_address())
        .code(MxscPath::new("output/mrv-gsoc-registry.mxsc.json"))
        .new_address(contract)
        .run();

    // Interact: Reserve an ITMO serial
    world.tx().from(owner).to(contract).raw_call("reserveSerial")
        .argument(&b"ITMO-SN-9999") 
        .run();

    world.write_scenario_trace("scenarios/gsoc-registry-lifecycle.scen.json");
}