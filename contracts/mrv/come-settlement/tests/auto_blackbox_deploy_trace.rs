use multiversx_sc_scenario::{imports::*, ScenarioTxRun};

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/mrv/come-settlement");
    blockchain.register_contract("file:output/mrv-come-settlement.wasm", mrv_come_settlement::ContractBuilder);
    blockchain.register_contract("mxsc:output/mrv-come-settlement.mxsc.json", mrv_come_settlement::ContractBuilder);
    blockchain
}

#[test]
fn generate_come_settlement_trace() {
    let mut world = world();
    let owner = TestAddress::new("owner");
    let contract = TestSCAddress::new("come_settlement_contract");
    let mock_gov = TestAddress::new("mock_gov");
    
    let sender = TestAddress::new("sender");
    let receiver = TestAddress::new("receiver");

    world.account(owner).nonce(1).balance(1_000_000u64);

    world.start_trace();

    world.tx().from(owner).raw_deploy()
        .argument(&mock_gov.to_address())
        .code(MxscPath::new("output/mrv-come-settlement.mxsc.json"))
        .new_address(contract)
        .run();

    let token_id = TestTokenIdentifier::new("COME-123456");

    // Interact: Create Settlement
    world.tx().from(owner).to(contract).raw_call("createSettlement")
        .argument(&b"SETTLE-001")          // settlement_id
        .argument(&sender.to_address())    // from
        .argument(&receiver.to_address())  // to
        .argument(&token_id)               // token_id
        .argument(&50_000u64)              // amount_scaled (auto-converts to BigUint)
        .argument(&b"ipfs://reason")       // reason_cid
        .run();

    world.write_scenario_trace("scenarios/come-settlement-lifecycle.scen.json");
}