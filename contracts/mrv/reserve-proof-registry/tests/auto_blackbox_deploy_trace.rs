use multiversx_sc_scenario::{imports::*, ScenarioTxRun};

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/mrv/reserve-proof-registry");
    blockchain.register_contract("file:output/mrv-reserve-proof-registry.wasm", mrv_reserve_proof_registry::ContractBuilder);
    blockchain.register_contract("mxsc:output/mrv-reserve-proof-registry.mxsc.json", mrv_reserve_proof_registry::ContractBuilder);
    blockchain
}

#[test]
fn generate_reserve_proof_registry_trace() {
    let mut world = world();
    let owner = TestAddress::new("owner");
    let contract = TestSCAddress::new("reserve_proof_contract");
    let mock_gov = TestAddress::new("mock_gov");

    world.account(owner).nonce(1).balance(1_000_000u64);

    world.start_trace();

    world.tx().from(owner).raw_deploy()
        .argument(&mock_gov.to_address())
        .code(MxscPath::new("output/mrv-reserve-proof-registry.mxsc.json"))
        .new_address(contract)
        .run();

    // Interact: Anchor Reserve Proof
    world.tx().from(owner).to(contract).raw_call("anchorReserveProof")
        .argument(&b"COME-123456")                               // token_id
        .argument(&100_000u64)                                   // total_supply_scaled
        .argument(&10_000u64)                                    // total_buffer_scaled
        .argument(&5_000u64)                                     // total_retired_scaled
        .argument(&b"12345678901234567890123456789012")          // merkle_root (32 bytes)
        .argument(&100u64)                                       // snapshot_block
        .run();

    world.write_scenario_trace("scenarios/reserve-proof-lifecycle.scen.json");
}