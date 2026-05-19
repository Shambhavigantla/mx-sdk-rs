use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn atomic_swap_init_go() {
    world().run("scenarios/atomic-swap-init.scen.json");
}
