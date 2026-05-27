# Template Test Flow

This document explains how `template-test` runs in `framework/meta`, and the changes made to ensure generated contracts use the repo-local patched executor and DRWA governance hook.

## How template tests run

Template tests are implemented in `framework/meta/tests/template_test.rs`.

There are two test modes:

- `template-test-current`: runs against the current repo checkout.
- `template-test-released`: downloads a released repo version from GitHub and runs the same contract generation checks.

Run from `framework/meta`:

```bash
cargo test --features template-test-current
```

or:

```bash
cargo test --features template-test-released
```

## Current-branch template test flow

For `template-test-current`, the test does the following:

1. Find the current repository workspace with `find_current_workspace()`.
2. Build a `RepoSource::LocalPath(workspace_path)` pointing to the local repo.
3. Create a temporary generation target under `template-test/<sub_path>/<new_name>`.
4. Use `ContractCreator::new(...).create_contract(...)` to generate a fresh contract workspace from a template.
5. Build the generated contract by running the generated contract's `meta` tool:
   - `cargo run -- build --target-dir <...>`
6. Run the generated contract's tests with `cargo test`.

That means the generated contract becomes a standalone workspace and must resolve its own dependencies correctly.

## Why patch injection is required

A generated contract workspace does not automatically inherit the parent repository's root workspace patches.

If the generated contract resolves `multiversx-chain-vm-executor` or `multiversx-chain-vm-executor-wasmer-experimental` from crates.io or from a remote patch, it may compile against an older executor version that does not expose the current DRWA hook signatures.

To fix that, the generator now injects a local patch section directly into the generated contract's `Cargo.toml`.

### Patch injection location

The code that does this is in:

- `framework/meta/src/cmd/template/contract_creator.rs`

The injected method is:

- `ContractCreator::inject_repo_patch_if_local()`

It adds a generated `Cargo.toml` patch like:

```toml
[patch.crates-io]
multiversx-chain-vm-executor = { path = "<relative-path-to>/vendor/multiversx-chain-vm-executor" }
multiversx-chain-vm-executor-wasmer-experimental = { path = "<relative-path-to>/vendor/multiversx-chain-vm-executor-wasmer-experimental" }
multiversx-chain-vm = { path = "<relative-path-to>/chain/vm" }
```

The relative paths are computed from the generated contract directory.

## What changed for the DRWA governance hook

The failing compile error came from a mismatch between the executor VM hook trait and the implementation used by `chain/vm`.

`chain/vm/src/host/vm_hooks/vh_dispatcher.rs` implements both DRWA hooks, including:

- `managed_drwa_sync_mirror(...)`
- `managed_drwa_native_governance_query(...)`

The generated contract workspace must build against an executor crate that exposes both hooks.

### Added support for `managed_drwa_native_governance_query`

The following files were updated to make the new hook available in the local executor bindings:

- `vendor/multiversx-chain-vm-executor/src/vm_hooks.rs`
- `vendor/multiversx-chain-vm-executor/src/new_traits/vm_hooks_new.rs`
- `vendor/multiversx-chain-vm-executor/src/new_traits/vm_hooks_legacy_adapter.rs`
- `vendor/multiversx-chain-vm-executor-wasmer-experimental/src/we_imports.rs`

These changes ensure the executor trait and the Wasmer host imports both recognize `managed_drwa_native_governance_query`.

## Other relevant patch settings

The repo also uses local patching in `Cargo.toml` for the top-level workspace to ensure local sources are used where appropriate.

For example:

- `multiversx-chain-core = { path = "chain/core" }`

And `framework/meta/Cargo.toml` already patches the `sc-meta` tool build to use local or specific executor sources.

## Summary

- `framework/meta/tests/template_test.rs` runs generated contract creation and build/test verification.
- `ContractCreator::inject_repo_patch_if_local()` in `framework/meta/src/cmd/template/contract_creator.rs` writes a `[patch.crates-io]` block into generated contract `Cargo.toml` so the generated workspace uses local repo executor crates.
- `managed_drwa_native_governance_query` support was added in vendor executor bindings so the `chain/vm` implementation and generated workspace build together.
- Running `cargo test --features template-test-current` from `framework/meta` now verifies this flow.
