use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn asset_manager_init_go() {
    world().run("scenarios/asset-manager-init.scen.json");
}

#[test]
fn asset_manager_denial_signals_go() {
    world().run("scenarios/asset-manager-denial-signals.scen.json");
}
