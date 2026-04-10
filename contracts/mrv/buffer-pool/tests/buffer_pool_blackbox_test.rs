use multiversx_sc_scenario::imports::*;

use buffer_pool::buffer_pool_proxy::BufferPoolProxy;

const OWNER: TestAddress = TestAddress::new("owner");
const GOVERNANCE: TestAddress = TestAddress::new("governance");
const CARBON_CREDIT: TestSCAddress = TestSCAddress::new("carbon-credit");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("buffer-pool");
const CODE_PATH: MxscPath = MxscPath::new("mxsc:output/buffer-pool.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());
    blockchain.set_current_dir_from_workspace("contracts/mrv/buffer-pool");
    blockchain.register_contract(CODE_PATH, buffer_pool::ContractBuilder);
    blockchain
}

/// Deploy and initialize the buffer pool contract.
#[test]
fn buffer_pool_blackbox_init() {
    let mut world = world();

    world.account(OWNER).nonce(1).balance(1_000_000u64);

    // Deploy
    world
        .tx()
        .from(OWNER)
        .typed(BufferPoolProxy)
        .init(GOVERNANCE.to_managed_address(), CARBON_CREDIT.to_managed_address())
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .run();
}

/// Deposit buffer credits.
#[test]
fn buffer_pool_blackbox_deposit_credits() {
    let mut world = world();

    world.account(OWNER).nonce(1).balance(1_000_000u64);

    // Deploy
    world
        .tx()
        .from(OWNER)
        .typed(BufferPoolProxy)
        .init(GOVERNANCE.to_managed_address(), CARBON_CREDIT.to_managed_address())
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .run();

    // Deposit credits
    world
        .tx()
        .from(OWNER)
        .to(SC_ADDRESS)
        .typed(BufferPoolProxy)
        .deposit_buffer_credits(
            ManagedBuffer::from(b"PROJECT-001"),
            1000u64.into(),
            1u64,
        )
        .run();
}