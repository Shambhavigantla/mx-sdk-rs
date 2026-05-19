use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
#[ignore = "requires Go VM"]
fn buffer_pool_init_go() {
    world().run("scenarios/buffer-pool-init.scen.json");
}
