use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.register_contract(
        "mxsc:output/mrv-registry.mxsc.json",
        mrv_registry::ContractBuilder,
    );
    blockchain
}

#[test]
fn registry_init_rs() {
    world().run("scenarios/registry-init.scen.json");
}

#[test]
fn registry_methodology_lifecycle_rs() {
    world().run("scenarios/registry-methodology-lifecycle.scen.json");
}

#[test]
fn registry_report_anchor_rs() {
    world().run("scenarios/registry-report-anchor.scen.json");
}
