use multiversx_sc_scenario::imports::*;

const OWNER: TestAddress = TestAddress::new("owner");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("registry");
const CODE_PATH: MxscPath = MxscPath::new("mxsc:../output/mrv-registry.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());
    blockchain.set_current_dir_from_workspace("contracts/mrv/registry");
    blockchain.start_trace();
    blockchain
}

#[test]
fn registry_deploy_and_trace() {
    let mut world = world();

    world.account(OWNER).nonce(1).balance(1_000_000u64);

    // Raw deploy the contract (no init arguments). If contract requires init args,
    // this may fail; the generated trace will still be written for debugging.
    world
        .tx()
        .from(OWNER)
        .raw_deploy()
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .run();

    // write trace into scenarios folder for reuse
    world.write_scenario_trace("scenarios/registry_deploy_trace.json");
}
