use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn carbon_credit_init_go() {
    world().run("scenarios/carbon-credit-init.scen.json");
}
