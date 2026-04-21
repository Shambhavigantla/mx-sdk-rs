use multiversx_sc_scenario::{imports::*, ScenarioTxRun};

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/mrv/carbon-credit");
    blockchain.register_contract("file:output/mrv-carbon-credit.wasm", mrv_carbon_credit::ContractBuilder);
    blockchain.register_contract("mxsc:output/mrv-carbon-credit.mxsc.json", mrv_carbon_credit::ContractBuilder);
    blockchain
}

#[test]
fn generate_carbon_credit_trace() {
    let mut world = world();
    let owner = TestAddress::new("owner");
    let contract = TestSCAddress::new("carbon_credit_contract");
    world.account(owner).nonce(1).balance(1_000_000u64);

    let mock_gov = TestAddress::new("mock_gov");
    let mock_buffer = TestAddress::new("mock_buffer");

    world.start_trace();

    world.tx().from(owner).raw_deploy()
        .argument(&mock_gov.to_address())
        .argument(&mock_buffer.to_address())
        .code(MxscPath::new("output/mrv-carbon-credit.mxsc.json"))
        .new_address(contract)
        .run();

    // Interact: Register IME Record
    world.tx().from(owner).to(contract).raw_call("registerImeRecord")
        .argument(&b"PROJ-001")                   // project_id
        .argument(&b"sha256:science_digest")      // science_service_image_digest
        .argument(&b"sha256:param_hash")          // parameter_pack_hash
        .argument(&b"sha256:calib_hash")          // calibration_dataset_hash
        .argument(&b"sha256:strata_hash")         // strata_protocol_hash
        .argument(&b"v1.0")                       // methodology_version
        .argument(&2_000_000_000u64)              // valid_until (future timestamp)
        .argument(&b"JUR-01")                     // domain_codes (variadic start)
        .run();

    world.write_scenario_trace("scenarios/carbon-credit-lifecycle.scen.json");
}