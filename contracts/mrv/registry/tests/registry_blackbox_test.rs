use multiversx_sc_scenario::{imports::*, ScenarioTxRun};

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/mrv/registry");
    blockchain.register_contract("file:output/mrv-registry.wasm", mrv_registry::ContractBuilder);
    blockchain.register_contract("mxsc:output/mrv-registry.mxsc.json", mrv_registry::ContractBuilder);
    blockchain
}

#[test]
fn generate_registry_lifecycle_trace() {
    let mut world = world();

    let owner = TestAddress::new("owner");
    let project_developer = TestAddress::new("project_developer");
    let registry_contract = TestSCAddress::new("registry_contract");

    world.account(owner).nonce(1).balance(100_000_000_000u64);
    world.account(project_developer).nonce(1).balance(10_000_000u64);

    // 1. START RECORDING
    world.start_trace();

    // 2. DEPLOY REGISTRY
   // 1. Create a dummy address to act as the governance contract
    let mock_governance = TestAddress::new("mock_governance");

    // 2. Execute the deployment
    world
        .tx()
        .from(owner) // (or OWNER, depending on how your file named it)
        .raw_deploy()
        .argument(&mock_governance.to_address()) // <--- WE ADDED THIS LINE
        .code(MxscPath::new("output/mrv-registry.mxsc.json")) // (or CODE_PATH)
        .new_address(registry_contract) // (or SC_ADDRESS)
        .run();

    // 3. INTERACT: Register a Methodology
    world
        .tx()
        .from(owner)
        .to(registry_contract)
        .raw_call("registerMethodology")
        .argument(&b"AgriCarbon-V1")              // 1. methodology_id
        .argument(&b"1.0.0")                      // 2. version_label
        .argument(&b"sha256:dummy_digest_hash")   // 3. pack_digest
        .argument(&b"approved")                   // 4. approval_status
        .argument(&1_600_000_000u64)              // 5. effective_from (timestamp)
        .argument(&2_000_000_000u64)              // 6. effective_to (timestamp)
        .run();

    // 4. EXPORT TRACE
    world.write_scenario_trace("scenarios/registry-lifecycle.scen.json");
}