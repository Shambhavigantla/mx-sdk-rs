use multiversx_sc_scenario::{imports::*, ScenarioTxRun};

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/mrv/governance");
    blockchain.register_contract("file:output/mrv-governance.wasm", mrv_governance::ContractBuilder);
    blockchain.register_contract("mxsc:output/mrv-governance.mxsc.json", mrv_governance::ContractBuilder);
    blockchain
}

#[test]
fn generate_governance_trace() {
    let mut world = world();
    let owner = TestAddress::new("owner");
    let signer2 = TestAddress::new("signer2");
    let contract = TestSCAddress::new("governance_contract");

    world.account(owner).nonce(1).balance(1_000_000u64);

    world.start_trace();

    // The contract expects a ManagedVec, so we pass a standard Rust Vec. 
    // The test framework will encode it perfectly with the length prefix.
    let signers_vec = vec![owner.to_address(), signer2.to_address()];

    world.tx().from(owner).raw_deploy()
        .argument(&signers_vec)   // initial_signers (ManagedVec)
        .argument(&2u32)          // approval_threshold
        .argument(&3600u64)       // timelock_seconds (must be >= 3600)
        .code(MxscPath::new("output/mrv-governance.mxsc.json"))
        .new_address(contract)
        .run();

    // Interact: Propose an emergency pause (Must be called by a signer)
    world.tx().from(owner).to(contract).raw_call("proposeEmergencyPause")
        .argument(&b"PAUSE-001")  // proposal_id
        .argument(&true)          // pause boolean
        .run();

    world.write_scenario_trace("scenarios/governance-lifecycle.scen.json");
}