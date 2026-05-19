use drwa_policy_registry;
use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    // The "compiled_tests_or" strategy is the framework's standard for avoiding drift.
    // It uses Wasmer if artifacts exist, but falls back to the Rust ContractBuilder
    // (default executor) if they are missing.
    let mut blockchain = ScenarioWorld::new()
        .executor_config(ExecutorConfig::compiled_tests_or(ExecutorConfig::default()));
    
    blockchain.set_current_dir_from_workspace("contracts/drwa/asset-manager");
    
    // REGISTRATION STRINGS MAPPED TO CANONICAL LOCATIONS
    // These keys match the resolved paths from the scenario JSON files.
    
    // 1. Asset Manager (Self)
    // JSON (scenarios/): mxsc:../output/drwa-asset-manager.mxsc.json
    // Registry (root): mxsc:output/drwa-asset-manager.mxsc.json
    blockchain.register_contract(
        "mxsc:output/drwa-asset-manager.mxsc.json",
        drwa_asset_manager::ContractBuilder,
    );
    
    // 2. Policy Registry (Dependency)
    // JSON (scenarios/): mxsc:../../policy-registry/output/drwa-policy-registry.mxsc.json
    // Registry (root): mxsc:../policy-registry/output/drwa-policy-registry.mxsc.json
    blockchain.register_contract(
        "mxsc:../policy-registry/output/drwa-policy-registry.mxsc.json",
        drwa_policy_registry::ContractBuilder,
    );
    
    blockchain
}

#[test]
fn asset_manager_init_rs() {
    world().run("scenarios/asset-manager-init.scen.json");
}

#[test]
fn asset_manager_denial_signals_rs() {
    world().run("scenarios/asset-manager-denial-signals.scen.json");
}
