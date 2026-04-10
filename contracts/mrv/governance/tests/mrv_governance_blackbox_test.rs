use multiversx_sc_scenario::imports::*;

use mrv_governance::mrv_governance_proxy::MrvGovernanceProxy;

const OWNER: TestAddress = TestAddress::new("owner");
const SIGNER1: TestAddress = TestAddress::new("signer1");
const SIGNER2: TestAddress = TestAddress::new("signer2");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("mrv-governance");
const CODE_PATH: MxscPath = MxscPath::new("mxsc:output/mrv-governance.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());
    blockchain.set_current_dir_from_workspace("contracts/mrv/governance");
    blockchain.register_contract(CODE_PATH, mrv_governance::ContractBuilder);
    blockchain.start_trace();
    blockchain
}

/// Deploy and initialize the governance contract.
#[test]
fn mrv_governance_blackbox_init() {
    let mut world = world();

    world.account(OWNER).nonce(1).balance(1_000_000u64);
    world.account(SIGNER1).nonce(1).balance(1_000_000u64);
    world.account(SIGNER2).nonce(1).balance(1_000_000u64);

    // Deploy
    let mut signers: ManagedVec<multiversx_sc_scenario::api::StaticApi, ManagedAddress<multiversx_sc_scenario::api::StaticApi>> = ManagedVec::new();
    signers.push(SIGNER1.to_managed_address());
    signers.push(SIGNER2.to_managed_address());

    world
        .tx()
        .from(OWNER)
        .raw_deploy()
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .argument(&signers)
        .argument(&2u32)
        .argument(&3600u64)
        .run();

    world.write_scenario_trace("init_trace.json");
}

/// Propose and accept a governance action.
#[test]
fn mrv_governance_blackbox_propose_accept() {
    let mut world = world();

    world.account(OWNER).nonce(1).balance(1_000_000u64);
    world.account(SIGNER1).nonce(1).balance(1_000_000u64);
    world.account(SIGNER2).nonce(1).balance(1_000_000u64);

    // Deploy
    let mut signers: ManagedVec<multiversx_sc_scenario::api::StaticApi, ManagedAddress<multiversx_sc_scenario::api::StaticApi>> = ManagedVec::new();
    signers.push(SIGNER1.to_managed_address());
    signers.push(SIGNER2.to_managed_address());

    world
        .tx()
        .from(OWNER)
        .raw_deploy()
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .argument(&signers)
        .argument(&2u32)
        .argument(&3600u64)
        .run();

    // Propose
    world
        .tx()
        .from(SIGNER1)
        .to(SC_ADDRESS)
        .typed(MrvGovernanceProxy)
        .propose_emergency_pause(
            ManagedBuffer::from(b"Test proposal"),
            true,
        )
        .run();

    // Approve from another signer
    world
        .tx()
        .from(SIGNER2)
        .to(SC_ADDRESS)
        .typed(MrvGovernanceProxy)
        .approve_proposal(ManagedBuffer::from(b"Test proposal"))
        .run();

    world.write_scenario_trace("propose_accept_trace.json");
}