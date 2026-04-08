use mrv_income_distribution::IncomeDistribution;
use mrv_common::MrvGovernanceModule;
use multiversx_sc::types::{TokenIdentifier, ManagedBuffer};
use multiversx_sc_scenario::imports::*;

const OWNER: TestAddress = TestAddress::new("owner");
const GOVERNANCE: TestAddress = TestAddress::new("governance");
const SC_ADDRESS: TestSCAddress = TestSCAddress::new("mrv-income-distribution");
const CODE_PATH: MxscPath = MxscPath::new("mxsc:output/mrv-income-distribution.mxsc.json");
const COME_TOKEN: TestTokenIdentifier = TestTokenIdentifier::new("COME-abcdef");
const WRONG_TOKEN: TestTokenIdentifier = TestTokenIdentifier::new("FAKE-123456");

fn world() -> ScenarioWorld {
    let mut world = ScenarioWorld::new().executor_config(ExecutorConfig::full_suite());
    world.set_current_dir_from_workspace("contracts/mrv/income-distribution");
    world.register_contract(CODE_PATH, mrv_income_distribution::ContractBuilder);
    world
}

#[test]
fn income_distribution_init_rs() {
    let mut world = world();

    world.account(OWNER).nonce(1).balance(1_000_000u64);
    world.account(GOVERNANCE).nonce(1).balance(1_000_000u64);

    world
        .tx()
        .from(OWNER)
        .raw_deploy()
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .whitebox(mrv_income_distribution::contract_obj, |sc| {
            sc.init(
                GOVERNANCE.to_managed_address(),
                TokenIdentifier::from("COME-abcdef"),
            );
        });

    world
        .query()
        .to(SC_ADDRESS)
        .whitebox(mrv_income_distribution::contract_obj, |sc| {
            assert_eq!(sc.governance().get(), GOVERNANCE.to_managed_address());
            assert_eq!(
                sc.come_token_id().get(),
                TokenIdentifier::from("COME-abcdef")
            );
        });
}

#[test]
fn income_distribution_rejects_zero_governance_rs() {
    let mut world = world();

    world.account(OWNER).nonce(1).balance(1_000_000u64);

    world
        .tx()
        .from(OWNER)
        .raw_deploy()
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .returns(ExpectError(4u64, "governance must not be zero"))
        .whitebox(mrv_income_distribution::contract_obj, |sc| {
            sc.init(
                ManagedAddress::zero(),
                TokenIdentifier::from("COME-abcdef"),
            );
        });
}

fn deploy_income_distribution(world: &mut ScenarioWorld) {
    world.account(OWNER).nonce(1).balance(1_000_000u64);
    world
        .account(GOVERNANCE)
        .nonce(1)
        .balance(1_000_000u64)
        .esdt_balance(COME_TOKEN, BigUint::from(1_000_000u64))
        .esdt_balance(WRONG_TOKEN, BigUint::from(500_000u64));

    world
        .tx()
        .from(OWNER)
        .raw_deploy()
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .whitebox(mrv_income_distribution::contract_obj, |sc| {
            sc.init(
                GOVERNANCE.to_managed_address(),
                TokenIdentifier::from("COME-abcdef"),
            );
        });
}

#[test]
fn income_distribution_fund_distribution_rs() {
    let mut world = world();
    deploy_income_distribution(&mut world);

    // 32-byte merkle root
    let merkle_root: [u8; 32] = [0xAAu8; 32];

    world
        .tx()
        .from(GOVERNANCE)
        .to(SC_ADDRESS)
        .payment(Payment::try_new(COME_TOKEN, 0, 50_000u64).unwrap())
        .whitebox(mrv_income_distribution::contract_obj, |sc| {
            sc.fund_distribution(
                ManagedBuffer::from(b"dist-001"),
                ManagedBuffer::from(&merkle_root[..]),
                100u64,
                ManagedBuffer::from(b"bafymanifest001"),
                6_000u64,
            );
        });

    world
        .query()
        .to(SC_ADDRESS)
        .whitebox(mrv_income_distribution::contract_obj, |sc| {
            let dist = sc
                .get_distribution(ManagedBuffer::from(b"dist-001"))
                .into_option()
                .unwrap();
            assert_eq!(dist.total_amount_scaled, BigUint::from(50_000u64));
            assert_eq!(dist.total_claimed_scaled, BigUint::zero());
            assert_eq!(dist.expiry_epoch, 6_000u64);
            assert!(!dist.reclaimed);
        });
}

#[test]
fn income_distribution_reclaim_expired_rs() {
    let mut world = world();
    deploy_income_distribution(&mut world);

    let merkle_root: [u8; 32] = [0xBBu8; 32];

    world
        .tx()
        .from(GOVERNANCE)
        .to(SC_ADDRESS)
        .payment(Payment::try_new(COME_TOKEN, 0, 30_000u64).unwrap())
        .whitebox(mrv_income_distribution::contract_obj, |sc| {
            sc.fund_distribution(
                ManagedBuffer::from(b"dist-002"),
                ManagedBuffer::from(&merkle_root[..]),
                100u64,
                ManagedBuffer::from(b"bafymanifest002"),
                6_000u64,
            );
        });

    // Advance epoch past expiry
    world.current_block().block_epoch(6_001u64);

    world
        .tx()
        .from(GOVERNANCE)
        .to(SC_ADDRESS)
        .whitebox(mrv_income_distribution::contract_obj, |sc| {
            sc.reclaim_expired(ManagedBuffer::from(b"dist-002"));
        });

    world
        .query()
        .to(SC_ADDRESS)
        .whitebox(mrv_income_distribution::contract_obj, |sc| {
            let dist = sc
                .get_distribution(ManagedBuffer::from(b"dist-002"))
                .into_option()
                .unwrap();
            assert!(dist.reclaimed);
        });
}

#[test]
fn income_distribution_reclaim_before_expiry_fails_rs() {
    let mut world = world();
    deploy_income_distribution(&mut world);

    let merkle_root: [u8; 32] = [0xCCu8; 32];

    world
        .tx()
        .from(GOVERNANCE)
        .to(SC_ADDRESS)
        .payment(Payment::try_new(COME_TOKEN, 0, 20_000u64).unwrap())
        .whitebox(mrv_income_distribution::contract_obj, |sc| {
            sc.fund_distribution(
                ManagedBuffer::from(b"dist-003"),
                ManagedBuffer::from(&merkle_root[..]),
                100u64,
                ManagedBuffer::from(b"bafymanifest003"),
                6_000u64,
            );
        });

    // Epoch still within expiry window
    world.current_block().block_epoch(5_999u64);

    world
        .tx()
        .from(GOVERNANCE)
        .to(SC_ADDRESS)
        .returns(ExpectError(4u64, "distribution not yet expired"))
        .whitebox(mrv_income_distribution::contract_obj, |sc| {
            sc.reclaim_expired(ManagedBuffer::from(b"dist-003"));
        });
}

#[test]
fn income_distribution_fund_with_wrong_token_fails_rs() {
    let mut world = world();
    deploy_income_distribution(&mut world);

    let merkle_root: [u8; 32] = [0xDDu8; 32];

    world
        .tx()
        .from(GOVERNANCE)
        .to(SC_ADDRESS)
        .payment(Payment::try_new(WRONG_TOKEN, 0, 10_000u64).unwrap())
        .returns(ExpectError(4u64, "must pay with COME token"))
        .whitebox(mrv_income_distribution::contract_obj, |sc| {
            sc.fund_distribution(
                ManagedBuffer::from(b"dist-004"),
                ManagedBuffer::from(&merkle_root[..]),
                100u64,
                ManagedBuffer::from(b"bafymanifest004"),
                6_000u64,
            );
        });
}

#[test]
fn income_distribution_claim_with_valid_proof_rs() {
    use std::cell::RefCell;

    let mut world = world();

    let holder: TestAddress = TestAddress::new("holder");

    world.account(OWNER).nonce(1).balance(1_000_000u64);
    world
        .account(GOVERNANCE)
        .nonce(1)
        .balance(1_000_000u64)
        .esdt_balance(COME_TOKEN, BigUint::from(1_000_000u64));
    world.account(holder).nonce(1).balance(0u64);

    world
        .tx()
        .from(OWNER)
        .raw_deploy()
        .code(CODE_PATH)
        .new_address(SC_ADDRESS)
        .whitebox(mrv_income_distribution::contract_obj, |sc| {
            sc.init(
                GOVERNANCE.to_managed_address(),
                TokenIdentifier::from("COME-abcdef"),
            );
        });

    // Step 1: Compute the Merkle leaf (single-leaf tree, leaf == root) inside a
    // whitebox query so we get the exact keccak256 the contract will produce.
    // leaf = keccak256(distribution_id || holder_address || amount_scaled_be)
    let merkle_root_cell: RefCell<[u8; 32]> = RefCell::new([0u8; 32]);
    let claim_amount: u64 = 25_000;

    world
        .query()
        .to(SC_ADDRESS)
        .whitebox(mrv_income_distribution::contract_obj, |sc| {
            let mut leaf_preimage = ManagedBuffer::new();
            leaf_preimage.append(&ManagedBuffer::from(b"dist-claim-001"));
            leaf_preimage.append(holder.to_managed_address().as_managed_buffer());
            leaf_preimage.append(&BigUint::from(claim_amount).to_bytes_be_buffer());
            let hash = sc.crypto().keccak256(&leaf_preimage);
            let hash_bytes = hash.as_managed_buffer().to_boxed_bytes();
            let mut root = [0u8; 32];
            root.copy_from_slice(hash_bytes.as_slice());
            *merkle_root_cell.borrow_mut() = root;
        });

    let merkle_root = *merkle_root_cell.borrow();

    // Step 2: Fund the distribution with the computed Merkle root.
    world
        .tx()
        .from(GOVERNANCE)
        .to(SC_ADDRESS)
        .payment(Payment::try_new(COME_TOKEN, 0, 50_000u64).unwrap())
        .whitebox(mrv_income_distribution::contract_obj, |sc| {
            sc.fund_distribution(
                ManagedBuffer::from(b"dist-claim-001"),
                ManagedBuffer::from(&merkle_root[..]),
                100u64,
                ManagedBuffer::from(b"bafymanifest-claim-001"),
                6_000u64,
            );
        });

    // Step 3: Holder claims with an empty proof (single-leaf tree: leaf IS the root).
    world
        .tx()
        .from(holder)
        .to(SC_ADDRESS)
        .whitebox(mrv_income_distribution::contract_obj, |sc| {
            sc.claim_yield(
                ManagedBuffer::from(b"dist-claim-001"),
                BigUint::from(claim_amount),
                ManagedVec::new(),
            );
        });

    // Step 4: Verify the claim was recorded and total_claimed_scaled updated.
    world
        .query()
        .to(SC_ADDRESS)
        .whitebox(mrv_income_distribution::contract_obj, |sc| {
            let dist = sc
                .get_distribution(ManagedBuffer::from(b"dist-claim-001"))
                .into_option()
                .unwrap();
            assert_eq!(dist.total_claimed_scaled, BigUint::from(claim_amount));
        });
}
