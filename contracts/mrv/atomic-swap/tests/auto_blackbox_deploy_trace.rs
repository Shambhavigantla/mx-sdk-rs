use multiversx_sc_scenario::{imports::*, ScenarioTxRun};

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/mrv/atomic-swap");
    blockchain.register_contract("file:output/mrv-atomic-swap.wasm", mrv_atomic_swap::ContractBuilder);
    blockchain.register_contract("mxsc:output/mrv-atomic-swap.mxsc.json", mrv_atomic_swap::ContractBuilder);
    blockchain
}

#[test]
fn generate_atomic_swap_trace() {
    let mut world = world();
    let owner = TestAddress::new("owner");
    let buyer = TestAddress::new("buyer");
    let dealer = TestAddress::new("dealer");
    let contract = TestSCAddress::new("atomic_swap_contract");

    world.account(owner).nonce(1).balance(1_000_000u64);
    world.account(buyer).nonce(1).balance(1_000_000u64);
    world.account(dealer).nonce(1).balance(1_000_000u64);

    world.start_trace();

    world.tx().from(owner).raw_deploy()
        .argument(&b"COME-123456")
        .code(MxscPath::new("output/mrv-atomic-swap.mxsc.json"))
        .new_address(contract)
        .run();

    // Interact: Create RFQ
    world.tx().from(owner).to(contract).raw_call("createRfq")
        .argument(&b"RFQ-001")              // rfq_id
        .argument(&buyer.to_address())      // buyer
        .argument(&dealer.to_address())     // dealer
        .argument(&b"RWA-111111")           // token_id
        .argument(&100u64)                  // quantity
        .argument(&500u64)                  // margin_amount
        .argument(&5u64)                    // price_come_per_unit
        .argument(&10_000u64)               // expiry_epoch
        .run();

    world.write_scenario_trace("scenarios/atomic-swap-lifecycle.scen.json");
}