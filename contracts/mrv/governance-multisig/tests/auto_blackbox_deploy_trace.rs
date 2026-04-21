use multiversx_sc_scenario::{imports::*, ScenarioTxRun};

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/mrv/governance-multisig");
    blockchain.register_contract("file:output/mrv-governance-multisig.wasm", mrv_governance_multisig::ContractBuilder);
    blockchain.register_contract("mxsc:output/mrv-governance-multisig.mxsc.json", mrv_governance_multisig::ContractBuilder);
    blockchain
}

#[test]
fn generate_governance_multisig_trace() {
    let mut world = world();
    let owner = TestAddress::new("owner");
    let mock_signer = TestAddress::new("mock_signer");
    let target = TestAddress::new("target_contract");
    let contract = TestSCAddress::new("multisig_contract");

    world.account(owner).nonce(1).balance(1_000_000u64);

    world.start_trace();

    world.tx().from(owner).raw_deploy()
        .argument(&2u32)                       // threshold
        .argument(&mock_signer.to_address())   // initial_signers (MultiValueEncoded - just pass flat arguments)
        .code(MxscPath::new("output/mrv-governance-multisig.mxsc.json"))
        .new_address(contract)
        .run();

    // Interact: Propose an Action (e.g., freeze an asset)
    world.tx().from(owner).to(contract).raw_call("proposeAction")
        .argument(&b"PROP-001")          // proposal_id
        .argument(&b"freeze")            // proposal_type
        .argument(&target.to_address())  // target_address
        .argument(&b"action_data_hex")   // action_data
        .run();

    world.write_scenario_trace("scenarios/governance-multisig-lifecycle.scen.json");
}