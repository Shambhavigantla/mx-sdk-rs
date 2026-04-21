use multiversx_sc_scenario::{imports::*, ScenarioTxRun};

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/mrv/income-distribution");
    blockchain.register_contract("file:output/mrv-income-distribution.wasm", mrv_income_distribution::ContractBuilder);
    blockchain.register_contract("mxsc:output/mrv-income-distribution.mxsc.json", mrv_income_distribution::ContractBuilder);
    blockchain
}

#[test]
fn generate_income_distribution_trace() {
    let mut world = world();
    let owner = TestAddress::new("owner");
    let contract = TestSCAddress::new("income_contract");
    let mock_gov = TestAddress::new("mock_gov");
    
    // 1. Account setup uses the testing framework's wrapper
    let come_token = TestTokenIdentifier::new("COME-123456");

    world.account(owner).nonce(1).balance(1_000_000u64)
        .esdt_balance(come_token, 1_000_000u64); 

    world.start_trace();

    world.tx().from(owner).raw_deploy()
        .argument(&mock_gov)
        .argument(&b"COME-123456") // Safe to use raw bytes for standard arguments
        .code(MxscPath::new("output/mrv-income-distribution.mxsc.json"))
        .new_address(contract)
        .run();

    // 2. FORCE STRICT TYPES FOR PAYMENT
    // We manually cast the string and number into the exact VM memory types
    let strict_token_id = EsdtTokenIdentifier::from(b"COME-123456".as_slice());
    let strict_amount = BigUint::from(50_000u64);

    // Now the compiler will perfectly recognize the payment tuple!
    world.tx().from(owner).to(contract)
        .payment((strict_token_id, 0u64, strict_amount)) 
        .raw_call("fundDistribution")
        .argument(&b"DIST-001")            
        .argument(&b"12345678901234567890123456789012")             
        .argument(&1_000u64)               
        .argument(&b"ipfs://manifest")     
        .argument(&10_000u64)              
        .run();
    world.write_scenario_trace("scenarios/income-distribution-lifecycle.scen.json");
}