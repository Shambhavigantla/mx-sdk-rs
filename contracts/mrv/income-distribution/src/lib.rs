#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod income_distribution_proxy;

use mrv_common::MrvGovernanceModule;

const MINIMUM_CLAIM_WINDOW_EPOCHS: u64 = 5_000;

/// Merkle-gated COME distribution record with funding, claim tracking, and expiry.
#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, Clone, PartialEq, Eq)]
pub struct DistributionRecord<M: ManagedTypeApi> {
    pub distribution_id: ManagedBuffer<M>,
    pub issuer: ManagedAddress<M>,
    pub merkle_root: ManagedBuffer<M>,
    pub snapshot_block: u64,
    pub manifest_cid: ManagedBuffer<M>,
    pub total_amount_scaled: BigUint<M>,
    pub total_claimed_scaled: BigUint<M>,
    pub expiry_epoch: u64,
    pub funded_at: u64,
    pub reclaimed: bool,
}

/// Merkle-based income distribution contract.
///
/// Issuers fund distributions with COME, and holders claim against a
/// recorded Merkle root until the configured expiry. Merkle proof depth
/// is capped at 64 levels to bound on-chain execution cost.
#[multiversx_sc::contract]
pub trait IncomeDistribution: mrv_common::MrvGovernanceModule {
    /// Initializes the contract with a governance address and the COME token
    /// identifier used for distribution funding and claims.
    #[init]
    fn init(&self, governance: ManagedAddress, come_token_id: TokenIdentifier) {
        require!(!governance.is_zero(), "governance must not be zero");
        require!(come_token_id.is_valid_esdt_identifier(), "invalid COME token ID");
        self.governance().set(governance);
        self.come_token_id().set(come_token_id);
    }

    /// Funds a distribution with COME and records its Merkle root and expiry.
    #[payable("*")]
    #[endpoint(fundDistribution)]
    fn fund_distribution(
        &self,
        distribution_id: ManagedBuffer,
        merkle_root: ManagedBuffer,
        snapshot_block: u64,
        manifest_cid: ManagedBuffer,
        expiry_epoch: u64,
    ) {
        self.require_governance_or_owner();
        require!(!distribution_id.is_empty(), "empty distribution_id");
        require!(merkle_root.len() == 32, "merkle_root must be 32 bytes");
        let zero_root = ManagedBuffer::from(&[0u8; 32]);
        require!(merkle_root != zero_root, "merkle_root must not be all zeros");
        require!(!manifest_cid.is_empty(), "empty manifest_cid");

        let current_epoch = self.blockchain().get_block_epoch();
        require!(
            expiry_epoch >= current_epoch + MINIMUM_CLAIM_WINDOW_EPOCHS,
            "expiry_epoch must be at least MINIMUM_CLAIM_WINDOW_EPOCHS from now"
        );

        require!(
            !self.distributions().contains_key(&distribution_id),
            "distribution already exists"
        );

        let payment = self.call_value().single_esdt();
        require!(
            payment.token_identifier == self.come_token_id().get(),
            "must pay with COME token"
        );
        require!(payment.amount > 0u64, "must fund with positive amount");

        let record = DistributionRecord {
            distribution_id: distribution_id.clone(),
            issuer: self.blockchain().get_caller(),
            merkle_root,
            snapshot_block,
            manifest_cid,
            total_amount_scaled: payment.amount.clone(),
            total_claimed_scaled: BigUint::zero(),
            expiry_epoch,
            funded_at: self.blockchain().get_block_timestamp(),
            reclaimed: false,
        };

        self.distributions().insert(distribution_id.clone(), record);
        self.distribution_funded_event(&distribution_id, &payment.amount);
    }

    /// Claims a funded amount for the caller by verifying the provided Merkle proof.
    ///
    /// The proof depth is capped at 64 levels to bound execution cost, and
    /// the leaf binds `distribution_id` to prevent cross-distribution replay.
    /// Merkle verification uses `keccak256`.
    /// Other contracts in this workspace may use SHA-256 for content-addressed
    /// data such as CIDs, but that does not affect this proof format.
    #[endpoint(claimYield)]
    fn claim_yield(
        &self,
        distribution_id: ManagedBuffer,
        amount_scaled: BigUint,
        merkle_proof: ManagedVec<ManagedBuffer>,
    ) {
        require!(merkle_proof.len() <= 64, "MERKLE_PROOF_TOO_DEEP");
        require!(
            !self.distribution_paused(&distribution_id).get(),
            "DISTRIBUTION_PAUSED: claims are temporarily suspended"
        );
        let holder = self.blockchain().get_caller();
        let dist = self.distributions().get(&distribution_id);
        require!(dist.is_some(), "distribution not found");
        let dist = dist.unwrap();

        let current_epoch = self.blockchain().get_block_epoch();
        require!(current_epoch <= dist.expiry_epoch, "DISTRIBUTION_EXPIRED");
        require!(!dist.reclaimed, "distribution already reclaimed");

        let claim_key = (distribution_id.clone(), holder.as_managed_buffer().clone());
        require!(
            !self.claimed().contains_key(&claim_key),
            "ALREADY_CLAIMED"
        );

        let mut leaf_preimage = ManagedBuffer::new();
        leaf_preimage.append(&distribution_id);
        leaf_preimage.append(holder.as_managed_buffer());
        leaf_preimage.append(&amount_scaled.to_bytes_be_buffer());
        let leaf = self.crypto().keccak256(&leaf_preimage);

        let mut current_hash = leaf.as_managed_buffer().clone();
        for i in 0..merkle_proof.len() {
            let sibling = merkle_proof.get(i);
            let mut combined = ManagedBuffer::new();
            let current_bytes = current_hash.to_boxed_bytes();
            let sibling_bytes = sibling.to_boxed_bytes();
            if current_bytes.as_slice() <= sibling_bytes.as_slice() {
                combined.append(&current_hash);
                combined.append(&sibling);
            } else {
                combined.append(&sibling);
                combined.append(&current_hash);
            }
            current_hash = self.crypto().keccak256(&combined).as_managed_buffer().clone();
        }
        require!(current_hash == dist.merkle_root, "INVALID_MERKLE_PROOF");

        require!(
            &dist.total_claimed_scaled + &amount_scaled <= dist.total_amount_scaled,
            "CLAIMS_EXCEED_FUNDED: cumulative claims would exceed distribution total"
        );

        self.claimed().insert(claim_key, true);
        self.distributions().entry(distribution_id.clone()).and_modify(|r| {
            r.total_claimed_scaled += &amount_scaled;
        });

        self.send().direct_esdt(
            &holder,
            &self.come_token_id().get(),
            0u64,
            &amount_scaled,
        );

        self.yield_claimed_event(&distribution_id, &holder, &amount_scaled);
    }

    /// Pauses claims for a distribution.
    #[endpoint(pauseDistribution)]
    fn pause_distribution(&self, distribution_id: ManagedBuffer) {
        self.require_governance_or_owner();
        self.distribution_paused(&distribution_id).set(true);
    }

    /// Resumes claims for a distribution.
    #[endpoint(unpauseDistribution)]
    fn unpause_distribution(&self, distribution_id: ManagedBuffer) {
        self.require_governance_or_owner();
        self.distribution_paused(&distribution_id).set(false);
    }

    /// Marks an expired distribution reclaimed and returns any remaining funded
    /// balance to the original issuer.
    #[endpoint(reclaimExpired)]
    fn reclaim_expired(&self, distribution_id: ManagedBuffer) {
        self.require_governance_or_owner();
        let dist = self.distributions().get(&distribution_id);
        require!(dist.is_some(), "distribution not found");
        let dist = dist.unwrap();

        let current_epoch = self.blockchain().get_block_epoch();
        require!(current_epoch > dist.expiry_epoch, "distribution not yet expired");
        require!(!dist.reclaimed, "already reclaimed");

        let unclaimed = &dist.total_amount_scaled - &dist.total_claimed_scaled;

        self.distributions().entry(distribution_id.clone()).and_modify(|r| {
            r.reclaimed = true;
        });

        if unclaimed > 0u64 {
            let sc_balance = self.blockchain().get_sc_balance(&EgldOrEsdtTokenIdentifier::esdt(self.come_token_id().get()), 0u64);
            let transfer_amount = if unclaimed <= sc_balance { unclaimed.clone() } else { sc_balance.clone() };
            if sc_balance < unclaimed {
                let shortfall = &unclaimed - &sc_balance;
                self.reclaim_shortfall(&distribution_id).set(shortfall.clone());
                self.reclaim_shortfall_event(&distribution_id, &unclaimed, &sc_balance, &shortfall);
            }
            if transfer_amount > 0u64 {
                self.send().direct_esdt(
                    &dist.issuer,
                    &self.come_token_id().get(),
                    0u64,
                    &transfer_amount,
                );
            }
        }

        self.distribution_reclaimed_event(&distribution_id);
    }

    #[view(getDistribution)]
    fn get_distribution(
        &self,
        distribution_id: ManagedBuffer,
    ) -> OptionalValue<DistributionRecord<Self::Api>> {
        match self.distributions().get(&distribution_id) {
            Some(r) => OptionalValue::Some(r),
            None => OptionalValue::None,
        }
    }

    #[view(isClaimed)]
    fn is_claimed(&self, distribution_id: ManagedBuffer, holder: ManagedAddress) -> bool {
        let key = (distribution_id, holder.as_managed_buffer().clone());
        self.claimed().contains_key(&key)
    }

    #[storage_mapper("comeTokenId")]
    fn come_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("distributions")]
    fn distributions(&self) -> MapMapper<ManagedBuffer, DistributionRecord<Self::Api>>;

    #[storage_mapper("claimed")]
    fn claimed(&self) -> MapMapper<(ManagedBuffer, ManagedBuffer), bool>;

    /// Pause flag keyed by distribution identifier.
    #[storage_mapper("distributionPaused")]
    fn distribution_paused(&self, distribution_id: &ManagedBuffer) -> SingleValueMapper<bool>;

    #[event("distributionFunded")]
    fn distribution_funded_event(
        &self,
        #[indexed] distribution_id: &ManagedBuffer,
        total_amount: &BigUint,
    );

    #[event("yieldClaimed")]
    fn yield_claimed_event(
        &self,
        #[indexed] distribution_id: &ManagedBuffer,
        #[indexed] holder: &ManagedAddress,
        amount: &BigUint,
    );

    /// Shortfall amount recorded when sc_balance < unclaimed during reclaim.
    #[storage_mapper("reclaimShortfall")]
    fn reclaim_shortfall(&self, distribution_id: &ManagedBuffer) -> SingleValueMapper<BigUint>;

    #[event("reclaimShortfall")]
    fn reclaim_shortfall_event(
        &self,
        #[indexed] distribution_id: &ManagedBuffer,
        #[indexed] expected_amount: &BigUint,
        #[indexed] actual_amount: &BigUint,
        shortfall: &BigUint,
    );

    #[event("distributionReclaimed")]
    fn distribution_reclaimed_event(&self, #[indexed] distribution_id: &ManagedBuffer);

    /// Preserves storage across upgrades.
    #[upgrade]
    fn upgrade(&self) {}
}
