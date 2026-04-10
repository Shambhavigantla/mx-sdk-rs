use multiversx_sc_scenario::imports::*;

use atomic_swap::atomic_swap_proxy::AtomicSwapProxy;

const OWNER: TestAddress = TestAddress::new("owner");
const BUYER: TestAddress = TestAddress::new("buyer");
const DEALER: TestAddress = TestAddress::new("dealer");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("atomic-swap");
const CODE_PATH: MxscPath = MxscPath::new("mxsc:output/atomic-swap.mxsc.json");
const COME_TOKEN: &[u8] = b"COME-123456";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());
    blockchain.set_current_dir_from_workspace("contracts/mrv/atomic-swap");
    blockchain.register_contract(CODE_PATH, atomic_swap::ContractBuilder);
    blockchain
}

/// Deploy and initialize the atomic swap contract.
#[test]
fn atomic_swap_blackbox_init() {
    let mut world = world();

    world.account(OWNER).nonce(1).balance(1_000_000u64);

    // Deploy
    world
        .tx()
        .from(OWNER)
        .typed(AtomicSwapProxy)
        .init(TokenIdentifier::from(COME_TOKEN))
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .run();
}

/// Create an RFQ and deposit margin.
#[test]
fn atomic_swap_blackbox_create_rfq_and_deposit() {
    let mut world = world();

    world.account(OWNER).nonce(1).balance(1_000_000u64);
    world.account(BUYER).nonce(1).balance(1_000_000u64).esdt_balance(COME_TOKEN, 1000u64);

    // Deploy
    world
        .tx()
        .from(OWNER)
        .typed(AtomicSwapProxy)
        .init(TokenIdentifier::from(COME_TOKEN))
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .run();

    // Create RFQ
    world
        .tx()
        .from(OWNER)
        .to(SC_ADDRESS)
        .typed(AtomicSwapProxy)
        .create_rfq(
            ManagedBuffer::from(b"RFQ-001"),
            BUYER.to_managed_address(),
            DEALER.to_managed_address(),
            TokenIdentifier::from(b"RWA-123"),
            100u64.into(),
            50u64.into(),
            10u64.into(),
            1000u64,
        )
        .run();

    // Deposit margin from buyer
    world
        .tx()
        .from(BUYER)
        .to(SC_ADDRESS)
        .typed(AtomicSwapProxy)
        .deposit_margin(ManagedBuffer::from(b"RFQ-001"))
        .esdt_transfer(TokenIdentifier::from(COME_TOKEN), 0u64, 50u64)
        .run();
}