use multiversx_sc_scenario::imports::*;

use governance_multisig::governance_multisig_proxy::GovernanceMultisigProxy;

const OWNER: TestAddress = TestAddress::new("owner");
const SIGNER1: TestAddress = TestAddress::new("signer1");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("governance-multisig");
const CODE_PATH: MxscPath = MxscPath::new("mxsc:output/governance-multisig.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());
    blockchain.set_current_dir_from_workspace("contracts/mrv/governance-multisig");
    blockchain.register_contract(CODE_PATH, governance_multisig::ContractBuilder);
    blockchain
}

/// Deploy and initialize the governance multisig contract.
#[test]
fn governance_multisig_blackbox_init() {
    let mut world = world();

    world.account(OWNER).nonce(1).balance(1_000_000u64);
    world.account(SIGNER1).nonce(1).balance(1_000_000u64);

    // Deploy
    world
        .tx()
        .from(OWNER)
        .typed(GovernanceMultisigProxy)
        .init(
            2u32,
            MultiValueEncoded::new(vec![SIGNER1.to_managed_address()]),
        )
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .run();
}

/// Add a signer.
#[test]
fn governance_multisig_blackbox_add_signer() {
    let mut world = world();

    world.account(OWNER).nonce(1).balance(1_000_000u64);
    world.account(SIGNER1).nonce(1).balance(1_000_000u64);

    // Deploy
    world
        .tx()
        .from(OWNER)
        .typed(GovernanceMultisigProxy)
        .init(
            2u32,
            MultiValueEncoded::new(vec![SIGNER1.to_managed_address()]),
        )
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .run();

    // Add signer
    world
        .tx()
        .from(OWNER)
        .to(SC_ADDRESS)
        .typed(GovernanceMultisigProxy)
        .add_signer(SIGNER1.to_managed_address())
        .run();
}