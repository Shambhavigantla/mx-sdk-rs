#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MVR_DIR="$ROOT_DIR/contracts/mrv"

echo "Generating deploy blackbox tests for MRV contracts in $MVR_DIR"

for d in "$MVR_DIR"/*/; do
  [ -d "$d" ] || continue
  dir_name=$(basename "$d")
  src_file="$d/src/lib.rs"
  scen_dir="$d/scenarios"
  tests_dir="$d/tests"

  # Only generate if src/lib.rs exists
  if [ -f "$src_file" ]; then
    mkdir -p "$tests_dir"
    test_file="$tests_dir/auto_blackbox_deploy_trace.rs"

    # determine artifact name
    pkg_name=""
    if [ -f "$d/Cargo.toml" ]; then
      pkg_name=$(grep -E '^name\s*=\s*"' "$d/Cargo.toml" | head -n1 | sed -E 's/name\s*=\s*"([^\"]+)"/\1/')
    fi
    if [ -z "$pkg_name" ]; then
      artifact_name="mrv-${dir_name}"
    else
      artifact_name="$pkg_name"
    fi

    sanitized_name=$(echo "$dir_name" | tr '-' '_')

    cat > "$test_file" <<EOF
use multiversx_sc_scenario::imports::*;

const OWNER: TestAddress = TestAddress::new("owner");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("${dir_name}");
const CODE_PATH: MxscPath = MxscPath::new("mxsc:../output/${artifact_name}.mxsc.json");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());
    blockchain.set_current_dir_from_workspace("contracts/mrv/${dir_name}");
    blockchain.start_trace();
    blockchain
}

#[test]
fn ${sanitized_name}_deploy_and_trace() {
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
    world.write_scenario_trace("scenarios/${dir_name}_deploy_trace.json");
}
EOF

    echo "Wrote $test_file (artifact: $artifact_name)"
  fi

done

echo "Generation complete."