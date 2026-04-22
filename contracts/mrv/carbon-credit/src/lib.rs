#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

/// IME validation record used to gate dVCU issuance.
#[type_abi]
#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, Clone, PartialEq, Eq,
)]
pub struct ImeValidationRecord<M: ManagedTypeApi> {
    pub project_id: ManagedBuffer<M>,
    pub science_service_image_digest: ManagedBuffer<M>,
    pub parameter_pack_hash: ManagedBuffer<M>,
    pub calibration_dataset_hash: ManagedBuffer<M>,
    pub strata_protocol_hash: ManagedBuffer<M>,
    pub methodology_version: ManagedBuffer<M>,
    pub domain_codes: ManagedVec<M, ManagedBuffer<M>>,
    pub valid_until: u64,
    pub revoked: bool,
}

/// Execution bundle fields checked against the active IME record during issuance.
#[type_abi]
#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, Clone, PartialEq, Eq,
)]
pub struct ExecutionBundleRef<M: ManagedTypeApi> {
    pub science_service_image_digest: ManagedBuffer<M>,
    pub parameter_pack_hash: ManagedBuffer<M>,
    pub calibration_dataset_hash: ManagedBuffer<M>,
    pub strata_protocol_hash: ManagedBuffer<M>,
    pub methodology_version: ManagedBuffer<M>,
}

/// Retirement record for the two-phase retirement workflow.
#[type_abi]
#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, Clone, PartialEq, Eq,
)]
pub struct RetirementRecord<M: ManagedTypeApi> {
    pub retirement_id: ManagedBuffer<M>,
    pub project_id: ManagedBuffer<M>,
    pub amount_scaled: BigUint<M>,
    pub beneficiary: ManagedAddress<M>,
    pub status: ManagedBuffer<M>,
    pub initiated_at: u64,
    pub burn_tx_hash: ManagedBuffer<M>,
}

/// M-03 (AUD-008): non-indexed payload for the
/// `gsocSerialPartiallyRetired` event. Bundles the two BigUints that
/// event-log framing cannot carry as separate non-indexed fields
/// (framing allows exactly one non-indexed argument).
#[type_abi]
#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, Clone, PartialEq, Eq,
)]
pub struct GsocPartialRetirementEventPayload<M: ManagedTypeApi> {
    pub amount_scaled: BigUint<M>,
    pub remaining_after: BigUint<M>,
}

/// M-03 (AUD-008): append-only GSOC retirement-event log entry.
///
/// Written by `burn_and_retire_gsoc` on every partial or full
/// retirement. Together with `gsoc_serial_remaining` and the
/// pre-existing immutable `gsoc_serial_records`, this gives readers a
/// consistent, replay-safe view of every retirement against a serial
/// — replacing the previous in-place mutation of the remaining
/// balance (which left no audit trail and produced inconsistent
/// snapshots across two reads between retirements).
#[type_abi]
#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, Clone, PartialEq, Eq,
)]
pub struct GsocRetirementEventRecord<M: ManagedTypeApi> {
    /// Per-serial monotonically-increasing sequence number, starting at 0.
    pub seq: u64,
    /// Amount retired in THIS event (not cumulative).
    pub amount_scaled: BigUint<M>,
    /// Remaining balance on the serial AFTER this event is applied.
    pub remaining_after: BigUint<M>,
    pub beneficiary_name: ManagedBuffer<M>,
    pub beneficiary_address: ManagedAddress<M>,
    /// Block round at which this retirement was recorded.
    pub retired_at_round: u64,
}

/// Carbon credit issuance and retirement contract.
///
/// dVCU issuance is gated by an active IME record and a committed execution
/// bundle. GSOC credits follow a parallel track with verifier validation,
/// DNA project reference, and ITMO serial uniqueness. Both tracks enforce a
/// configurable buffer pool contribution.
#[multiversx_sc::contract]
pub trait CarbonCreditModule: mrv_common::MrvGovernanceModule {
    #[init]
    fn init(&self, governance: ManagedAddress, buffer_pool_addr: ManagedAddress) {
        require!(!governance.is_zero(), "governance must not be zero");
        require!(
            !buffer_pool_addr.is_zero(),
            "buffer_pool_addr must not be zero"
        );
        self.governance().set(governance);
        self.buffer_pool_addr().set(buffer_pool_addr);
        self.storage_version().set(1u32);
    }

    /// Issues dVCU credits after validating the IME record, bundle binding,
    /// and jurisdiction membership for the requested period.
    #[endpoint(issueCredits)]
    fn issue_credits(
        &self,
        project_id: ManagedBuffer,
        pai_id: ManagedBuffer,
        monitoring_period_n: u64,
        jurisdiction_code: ManagedBuffer,
        gross_removals_scaled: BigUint,
        buffer_pct_bps: u64,
        bundle_ref: ExecutionBundleRef<Self::Api>,
        committed_bundle_hash: ManagedBuffer,
    ) {
        self.require_governance_or_owner();
        require!(!project_id.is_empty(), "empty project_id");
        require!(!pai_id.is_empty(), "empty pai_id");
        require!(monitoring_period_n > 0, "invalid monitoring_period_n");
        require!(
            gross_removals_scaled > 0u64,
            "gross_removals must be positive"
        );
        require!(
            buffer_pct_bps > 0 && buffer_pct_bps <= 2500,
            "buffer_pct_bps must be 1-2500"
        );

        require!(
            committed_bundle_hash.len() == 32,
            "committed_bundle_hash must be 32 bytes"
        );
        let bundle_key = (pai_id.clone(), mrv_common::period_key(monitoring_period_n));
        let registered = self.committed_bundles().get(&bundle_key);
        require!(
            registered.is_some(),
            "BUNDLE_NOT_REGISTERED: call registerCommittedBundle(pai_id, period, hash) first"
        );
        let registered_hash = registered.unwrap_or_else(|| sc_panic!("BUNDLE_NOT_REGISTERED"));
        require!(
            registered_hash == committed_bundle_hash,
            "BUNDLE_HASH_MISMATCH: committed_bundle_hash does not match registered hash for this PAI/period"
        );
        require!(
            !self.bound_bundle_hashes().contains_key(&bundle_key),
            "credits already issued for this PAI/period"
        );
        self.bound_bundle_hashes()
            .insert(bundle_key, committed_bundle_hash);

        let ime = self.active_ime_record(&project_id);
        require!(!ime.is_empty(), "IME_NOT_REGISTERED");
        let ime = ime.get();
        require!(!ime.revoked, "IME_REVOKED");
        require!(
            ime.valid_until
                > self
                    .blockchain()
                    .get_block_timestamp_seconds()
                    .as_u64_seconds(),
            "IME_EXPIRED"
        );

        require!(
            bundle_ref.science_service_image_digest == ime.science_service_image_digest,
            "IME_IMAGE_MISMATCH"
        );
        require!(
            bundle_ref.parameter_pack_hash == ime.parameter_pack_hash,
            "IME_PARAMETER_MISMATCH"
        );
        require!(
            bundle_ref.calibration_dataset_hash == ime.calibration_dataset_hash,
            "IME_CALIBRATION_MISMATCH"
        );
        require!(
            bundle_ref.strata_protocol_hash == ime.strata_protocol_hash,
            "IME_STRATA_PROTOCOL_MISMATCH"
        );
        require!(
            bundle_ref.methodology_version == ime.methodology_version,
            "IME_METHODOLOGY_MISMATCH"
        );
        let mut jurisdiction_valid = false;
        for i in 0..ime.domain_codes.len() {
            if *ime.domain_codes.get(i) == jurisdiction_code {
                jurisdiction_valid = true;
                break;
            }
        }
        require!(jurisdiction_valid, "IME_JURISDICTION_NOT_IN_DOMAIN");

        // M-05 (AUD-011): buffer is a non-permanence reserve against future
        // reversal, so rounding MUST be conservative (up). The previous floor
        // division `(gross * bps) / 10_000` accumulated dust across many
        // issuances and slowly underfunded the buffer pool.
        // `ceil(x / d)` is computed as `(x + d - 1) / d` for positive integers.
        let buffer_numerator = &gross_removals_scaled * buffer_pct_bps;
        let buffer_contribution = (buffer_numerator + 9_999u64) / 10_000u64;
        require!(
            buffer_contribution > 0u64,
            "BUFFER_ROUNDS_TO_ZERO: increase gross_removals_scaled to produce non-zero buffer"
        );
        let net_issuable = &gross_removals_scaled - &buffer_contribution;
        require!(
            net_issuable > 0u64,
            "NET_ISSUABLE_ZERO: gross_removals too small after buffer deduction"
        );

        let issuance_key = (
            project_id.clone(),
            pai_id.clone(),
            mrv_common::period_key(monitoring_period_n),
        );
        require!(
            !self.issuances().contains_key(&issuance_key),
            "credits already issued for this PAI/period"
        );
        self.issuances().insert(issuance_key, net_issuable.clone());

        let buffer_deposit_key = (
            project_id.clone(),
            pai_id.clone(),
            mrv_common::period_key(monitoring_period_n),
        );
        self.pending_buffer_deposits()
            .insert(buffer_deposit_key, buffer_contribution.clone());

        self.credits_issued_event(&project_id, &pai_id, &net_issuable);
        self.buffer_deposit_pending_event(&project_id, &pai_id, &buffer_contribution);
    }

    /// Registers the committed execution bundle hash for a PAI and monitoring period.
    #[endpoint(registerCommittedBundle)]
    fn register_committed_bundle(
        &self,
        pai_id: ManagedBuffer,
        monitoring_period_n: u64,
        bundle_hash: ManagedBuffer,
    ) {
        self.require_governance_or_owner();
        require!(!pai_id.is_empty(), "empty pai_id");
        require!(monitoring_period_n > 0, "invalid monitoring_period_n");
        require!(bundle_hash.len() == 32, "bundle_hash must be 32 bytes");
        let key = (pai_id, mrv_common::period_key(monitoring_period_n));
        require!(
            !self.committed_bundles().contains_key(&key),
            "bundle already registered for this PAI/period"
        );
        self.committed_bundles().insert(key, bundle_hash);
    }

    /// Registers the active IME validation record for a project.
    #[endpoint(registerImeRecord)]
    fn register_ime_record(
        &self,
        project_id: ManagedBuffer,
        science_service_image_digest: ManagedBuffer,
        parameter_pack_hash: ManagedBuffer,
        calibration_dataset_hash: ManagedBuffer,
        strata_protocol_hash: ManagedBuffer,
        methodology_version: ManagedBuffer,
        valid_until: u64,
        domain_codes: MultiValueEncoded<ManagedBuffer>,
    ) {
        self.require_governance_or_owner();
        require!(!project_id.is_empty(), "empty project_id");
        require!(
            valid_until
                > self
                    .blockchain()
                    .get_block_timestamp_seconds()
                    .as_u64_seconds(),
            "valid_until must be in the future"
        );

        let record = ImeValidationRecord {
            project_id: project_id.clone(),
            science_service_image_digest,
            parameter_pack_hash,
            calibration_dataset_hash,
            strata_protocol_hash,
            methodology_version,
            domain_codes: domain_codes.to_vec(),
            valid_until,
            revoked: false,
        };

        self.active_ime_record(&project_id).set(record);
        self.ime_registered_event(&project_id);
    }

    /// Clears a pending buffer deposit after the corresponding buffer-pool deposit succeeds.
    #[endpoint(confirmBufferDeposit)]
    fn confirm_buffer_deposit(
        &self,
        project_id: ManagedBuffer,
        pai_id: ManagedBuffer,
        monitoring_period_n: u64,
    ) {
        self.require_governance_or_owner();
        let key = (
            project_id,
            pai_id,
            mrv_common::period_key(monitoring_period_n),
        );
        require!(
            self.pending_buffer_deposits().contains_key(&key),
            "no pending buffer deposit for this project/PAI/period"
        );
        self.pending_buffer_deposits().remove(&key);
    }

    /// Revokes the active IME record for a project. Future issuance attempts
    /// for this project will fail until a new record is registered.
    #[endpoint(revokeImeRecord)]
    fn revoke_ime_record(&self, project_id: ManagedBuffer) {
        self.require_governance_or_owner();
        require!(
            !self.active_ime_record(&project_id).is_empty(),
            "IME not registered"
        );
        self.active_ime_record(&project_id)
            .update(|r| r.revoked = true);
        self.ime_revoked_event(&project_id);
    }

    /// Starts a retirement record that can later be burned or reverted.
    #[endpoint(initiateRetirement)]
    fn initiate_retirement(
        &self,
        retirement_id: ManagedBuffer,
        project_id: ManagedBuffer,
        amount_scaled: BigUint,
        beneficiary: ManagedAddress,
    ) {
        self.require_governance_or_owner();
        require!(!retirement_id.is_empty(), "empty retirement_id");
        require!(amount_scaled > 0u64, "amount must be positive");
        require!(
            !beneficiary.is_zero(),
            "ZERO_ADDRESS: beneficiary must not be zero"
        );
        require!(
            !self.retirements().contains_key(&retirement_id),
            "retirement already initiated"
        );

        let record = RetirementRecord {
            retirement_id: retirement_id.clone(),
            project_id: project_id.clone(),
            amount_scaled: amount_scaled.clone(),
            beneficiary: beneficiary.clone(),
            status: ManagedBuffer::from(b"initiated"),
            initiated_at: self
                .blockchain()
                .get_block_timestamp_seconds()
                .as_u64_seconds(),
            burn_tx_hash: ManagedBuffer::new(),
        };

        self.retirements().insert(retirement_id.clone(), record);
        self.retirement_initiated_event(&retirement_id, &project_id, &amount_scaled);
    }

    /// Confirms a retirement burn by recording the burn transaction hash and
    /// transitioning the retirement to `burned` status.
    #[endpoint(confirmRetirementBurn)]
    fn confirm_retirement_burn(&self, retirement_id: ManagedBuffer, burn_tx_hash: ManagedBuffer) {
        self.require_governance_or_owner();
        require!(
            self.retirements().contains_key(&retirement_id),
            "retirement not found"
        );
        require!(!burn_tx_hash.is_empty(), "empty burn_tx_hash");

        let record = self
            .retirements()
            .get(&retirement_id)
            .unwrap_or_else(|| sc_panic!("RETIREMENT_NOT_FOUND"));
        require!(
            record.status == b"initiated",
            "retirement not in initiated state"
        );

        self.retirements()
            .entry(retirement_id.clone())
            .and_modify(|r| {
                r.status = ManagedBuffer::from(b"burned");
                r.burn_tx_hash = burn_tx_hash.clone();
            });

        self.retirement_burned_event(&retirement_id, &burn_tx_hash);
    }

    /// Reverts an initiated retirement back to `reverted` status. Only
    /// retirements in `initiated` state can be reverted.
    #[endpoint(revertRetirement)]
    fn revert_retirement(&self, retirement_id: ManagedBuffer) {
        self.require_governance_or_owner();
        require!(
            self.retirements().contains_key(&retirement_id),
            "retirement not found"
        );

        let record = self
            .retirements()
            .get(&retirement_id)
            .unwrap_or_else(|| sc_panic!("RETIREMENT_NOT_FOUND"));
        require!(
            record.status == b"initiated",
            "can only revert initiated retirements"
        );

        self.retirements()
            .entry(retirement_id.clone())
            .and_modify(|r| {
                r.status = ManagedBuffer::from(b"reverted");
            });

        self.retirement_reverted_event(&retirement_id);
    }

    /// Issues GSOC credits after validating the registered bundle, verifier,
    /// DNA reference, and ITMO serial for the period.
    #[endpoint(issueGsocCredits)]
    fn issue_gsoc_credits(
        &self,
        project_id: ManagedBuffer,
        pai_id: ManagedBuffer,
        monitoring_period_n: u64,
        gsoc_bundle_hash: ManagedBuffer,
        verifier_did: ManagedAddress,
        dna_project_ref: ManagedBuffer,
        itmo_serial: ManagedBuffer,
        gross_removals_scaled: BigUint,
        buffer_pct_bps: u64,
    ) {
        self.require_governance_or_owner();
        require!(!project_id.is_empty(), "empty project_id");
        require!(!pai_id.is_empty(), "empty pai_id");
        require!(monitoring_period_n > 0, "invalid monitoring_period_n");
        require!(
            gross_removals_scaled > 0u64,
            "gross_removals must be positive"
        );
        require!(
            buffer_pct_bps > 0 && buffer_pct_bps <= 2500,
            "buffer_pct_bps must be 1-2500"
        );

        require!(
            gsoc_bundle_hash.len() == 32,
            "gsoc_bundle_hash must be 32 bytes"
        );
        let bundle_key = (pai_id.clone(), mrv_common::period_key(monitoring_period_n));
        let registered = self.gsoc_bundles().get(&bundle_key);
        require!(
            registered.is_some(),
            "GSOC_BUNDLE_NOT_REGISTERED: call registerGsocBundle first"
        );
        let registered_hash = registered.unwrap_or_else(|| sc_panic!("GSOC_BUNDLE_NOT_REGISTERED"));
        require!(
            registered_hash == gsoc_bundle_hash,
            "GSOC_BUNDLE_HASH_MISMATCH"
        );

        require!(!verifier_did.is_zero(), "empty verifier_did");
        require!(
            self.approved_gsoc_verifiers().contains(&verifier_did),
            "GSOC_VERIFIER_NOT_APPROVED"
        );

        require!(!dna_project_ref.is_empty(), "DNA_PROJECT_REF_REQUIRED");

        require!(!itmo_serial.is_empty(), "ITMO_SERIAL_REQUIRED");

        let issuance_key = (
            project_id.clone(),
            pai_id.clone(),
            mrv_common::period_key(monitoring_period_n),
        );
        require!(
            !self.gsoc_issuances().contains_key(&issuance_key),
            "GSOC credits already issued for this PAI/period"
        );

        // M-05 (AUD-011): conservative (ceiling) rounding on the GSOC buffer
        // contribution. Matches the dVCU issuance path above.
        let buffer_numerator = &gross_removals_scaled * buffer_pct_bps;
        let buffer_contribution = (buffer_numerator + 9_999u64) / 10_000u64;
        require!(buffer_contribution > 0u64, "BUFFER_ROUNDS_TO_ZERO");
        let net_issuable = &gross_removals_scaled - &buffer_contribution;
        require!(net_issuable > 0u64, "net_issuable must be positive");

        require!(
            !self.gsoc_serial_records().contains_key(&itmo_serial),
            "GSOC_SERIAL_ALREADY_ISSUED: itmo_serial already has an issuance record"
        );

        self.gsoc_issuances()
            .insert(issuance_key, net_issuable.clone());
        self.gsoc_serial_records().insert(
            itmo_serial.clone(),
            (
                project_id.clone(),
                monitoring_period_n,
                net_issuable.clone(),
            ),
        );

        self.gsoc_credits_issued_event(&project_id, &pai_id, &itmo_serial, &net_issuable);
    }

    /// Registers the committed GSOC bundle hash for a PAI and monitoring period.
    #[endpoint(registerGsocBundle)]
    fn register_gsoc_bundle(
        &self,
        pai_id: ManagedBuffer,
        monitoring_period_n: u64,
        bundle_hash: ManagedBuffer,
    ) {
        self.require_governance_or_owner();
        require!(!pai_id.is_empty(), "empty pai_id");
        require!(monitoring_period_n > 0, "invalid monitoring_period_n");
        require!(bundle_hash.len() == 32, "bundle_hash must be 32 bytes");
        let key = (pai_id, mrv_common::period_key(monitoring_period_n));
        require!(
            !self.gsoc_bundles().contains_key(&key),
            "GSOC bundle already registered for this PAI/period"
        );
        self.gsoc_bundles().insert(key, bundle_hash);
    }

    /// Retires GSOC credits for a serial and emits the corresponding retirement event.
    ///
    /// M-03 (AUD-008): this method no longer mutates the third field
    /// of `gsoc_serial_records` in place. Instead:
    ///  - `gsoc_serial_records(serial)` is now read-only after
    ///    issuance. The `BigUint` field carries the IMMUTABLE initial
    ///    amount minted for the serial and is authoritative for the
    ///    replay-safe lineage view.
    ///  - `gsoc_serial_remaining(serial)` carries the running total.
    ///    It is initialized implicitly from the initial amount (by
    ///    fallback in the read path) and decrements on every
    ///    retirement.
    ///  - `gsoc_retirement_events(serial, seq_key)` is the append-only
    ///    log of every retirement touching the serial. Each entry
    ///    captures the amount retired, the beneficiary, the block
    ///    round, and the `remaining_after` balance, so a reader can
    ///    reconstruct the full retirement history without trusting a
    ///    mutable snapshot.
    ///  - `gsoc_retirement_seq_count(serial)` stores the next
    ///    sequence number to be written.
    #[endpoint(burnAndRetireGsoc)]
    fn burn_and_retire_gsoc(
        &self,
        itmo_serial: ManagedBuffer,
        amount_scaled: BigUint,
        beneficiary_name: ManagedBuffer,
        beneficiary_address: ManagedAddress,
    ) {
        self.require_governance_or_owner();
        require!(!itmo_serial.is_empty(), "empty itmo_serial");
        require!(amount_scaled > 0u64, "amount must be positive");
        require!(
            !beneficiary_address.is_zero(),
            "ZERO_ADDRESS: beneficiary_address must not be zero"
        );
        require!(!beneficiary_name.is_empty(), "empty beneficiary_name");

        require!(
            self.gsoc_serial_records().contains_key(&itmo_serial),
            "GSOC serial not issued"
        );

        require!(
            !self.gsoc_retired_serials().contains(&itmo_serial),
            "GSOC_SERIAL_FULLY_RETIRED: no remaining balance on this serial"
        );

        // Read the IMMUTABLE initial amount from records. Do NOT mutate.
        let (_project_id, _period_n, initial_amount) = self
            .gsoc_serial_records()
            .get(&itmo_serial)
            .unwrap_or_else(|| sc_panic!("GSOC_SERIAL_NOT_FOUND"));

        // Read the current remaining balance. If the running-total
        // slot has never been written for this serial (i.e., this is
        // the first retirement), fall back to the immutable initial
        // amount. This is the sole correct bridge between the new
        // remaining-tracking schema and any pre-M-03 issuance record.
        let remaining = if self.gsoc_serial_remaining(&itmo_serial).is_empty() {
            initial_amount
        } else {
            self.gsoc_serial_remaining(&itmo_serial).get()
        };
        require!(
            remaining > 0u64,
            "GSOC_SERIAL_FULLY_RETIRED: remaining balance is zero"
        );
        require!(
            amount_scaled <= remaining,
            "GSOC_AMOUNT_EXCEEDS_REMAINING: cannot retire more than remaining quantity"
        );

        let new_remaining = &remaining - &amount_scaled;
        self.gsoc_serial_remaining(&itmo_serial).set(&new_remaining);

        // Append the event to the per-serial log. `seq` is stable
        // once written; `gsoc_retirement_seq_count` tracks the next
        // sequence number for future appends on the same serial.
        let seq = self.gsoc_retirement_seq_count(&itmo_serial).get();
        let retired_at_round = self.blockchain().get_block_round();
        let event_record = GsocRetirementEventRecord {
            seq,
            amount_scaled: amount_scaled.clone(),
            remaining_after: new_remaining.clone(),
            beneficiary_name: beneficiary_name.clone(),
            beneficiary_address: beneficiary_address.clone(),
            retired_at_round,
        };
        self.gsoc_retirement_events(&itmo_serial, seq)
            .set(event_record);
        self.gsoc_retirement_seq_count(&itmo_serial)
            .set(seq.saturating_add(1));

        if new_remaining == 0u64 {
            self.gsoc_retired_serials().insert(itmo_serial.clone());
        }

        self.gsoc_credit_retired_event(
            &itmo_serial,
            &amount_scaled,
            &beneficiary_name,
            &beneficiary_address,
        );
        // M-03: fires on EVERY partial or full retirement so indexers
        // can track the running remaining balance and sequence order
        // without re-deriving from the transaction log.
        let partial_payload = GsocPartialRetirementEventPayload {
            amount_scaled: amount_scaled.clone(),
            remaining_after: new_remaining.clone(),
        };
        self.gsoc_serial_partially_retired_event(&itmo_serial, seq, &partial_payload);
    }

    /// Adds a verifier to the approved GSOC verifier set. Owner-only.
    ///
    /// This contract maintains a local GSOC verifier set separate from the
    /// governance contract's GSOC verifier registry.  The governance contract
    /// is the authoritative source; this set is a local cache for fast lookups
    /// during issuance without cross-contract calls.
    ///
    /// Off-chain operators must monitor `GovernanceGsocVerifierAdded` /
    /// `GovernanceGsocVerifierRemoved` events and call the corresponding
    /// endpoints here to keep both registries in sync.
    #[only_owner]
    #[endpoint(addApprovedGsocVerifier)]
    fn add_approved_gsoc_verifier(&self, verifier: ManagedAddress) {
        require!(!verifier.is_zero(), "verifier must not be zero");
        self.approved_gsoc_verifiers().insert(verifier);
    }

    /// Removes a verifier from the approved GSOC verifier set. Owner-only.
    #[only_owner]
    #[endpoint(removeApprovedGsocVerifier)]
    fn remove_approved_gsoc_verifier(&self, verifier: ManagedAddress) {
        self.approved_gsoc_verifiers().swap_remove(&verifier);
    }

    #[view(isGsocVerifierApproved)]
    fn is_gsoc_verifier_approved(&self, verifier: ManagedAddress) -> bool {
        self.approved_gsoc_verifiers().contains(&verifier)
    }

    #[view(getImeRecord)]
    fn get_ime_record(
        &self,
        project_id: ManagedBuffer,
    ) -> OptionalValue<ImeValidationRecord<Self::Api>> {
        if self.active_ime_record(&project_id).is_empty() {
            OptionalValue::None
        } else {
            OptionalValue::Some(self.active_ime_record(&project_id).get())
        }
    }

    #[view(getRetirement)]
    fn get_retirement(
        &self,
        retirement_id: ManagedBuffer,
    ) -> OptionalValue<RetirementRecord<Self::Api>> {
        match self.retirements().get(&retirement_id) {
            Some(r) => OptionalValue::Some(r),
            None => OptionalValue::None,
        }
    }

    /// Buffer-pool contract address. Pending contributions are tracked locally
    /// and confirmed via `confirmBufferDeposit` after the buffer-pool deposit.
    #[storage_mapper("bufferPoolAddr")]
    fn buffer_pool_addr(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("activeImeRecord")]
    fn active_ime_record(
        &self,
        project_id: &ManagedBuffer,
    ) -> SingleValueMapper<ImeValidationRecord<Self::Api>>;

    #[storage_mapper("issuances")]
    fn issuances(&self) -> MapMapper<(ManagedBuffer, ManagedBuffer, ManagedBuffer), BigUint>;

    #[storage_mapper("retirements")]
    fn retirements(&self) -> MapMapper<ManagedBuffer, RetirementRecord<Self::Api>>;

    /// Committed bundle hashes keyed by `(pai_id, period_key)`.
    #[storage_mapper("committedBundles")]
    fn committed_bundles(&self) -> MapMapper<(ManagedBuffer, ManagedBuffer), ManagedBuffer>;

    /// Bound bundle hashes keyed by `(pai_id, period_key)` after issuance.
    #[storage_mapper("boundBundleHashes")]
    fn bound_bundle_hashes(&self) -> MapMapper<(ManagedBuffer, ManagedBuffer), ManagedBuffer>;

    /// Pending buffer contributions keyed by `(project_id, pai_id, period_key)`.
    #[storage_mapper("pendingBufferDeposits")]
    fn pending_buffer_deposits(
        &self,
    ) -> MapMapper<(ManagedBuffer, ManagedBuffer, ManagedBuffer), BigUint>;

    #[event("creditsIssued")]
    fn credits_issued_event(
        &self,
        #[indexed] project_id: &ManagedBuffer,
        #[indexed] pai_id: &ManagedBuffer,
        net_issuable: &BigUint,
    );

    #[event("bufferDepositPending")]
    fn buffer_deposit_pending_event(
        &self,
        #[indexed] project_id: &ManagedBuffer,
        #[indexed] pai_id: &ManagedBuffer,
        buffer_contribution: &BigUint,
    );

    #[event("imeRegistered")]
    fn ime_registered_event(&self, #[indexed] project_id: &ManagedBuffer);

    #[event("imeRevoked")]
    fn ime_revoked_event(&self, #[indexed] project_id: &ManagedBuffer);

    #[event("retirementInitiated")]
    fn retirement_initiated_event(
        &self,
        #[indexed] retirement_id: &ManagedBuffer,
        #[indexed] project_id: &ManagedBuffer,
        amount: &BigUint,
    );

    #[event("retirementBurned")]
    fn retirement_burned_event(
        &self,
        #[indexed] retirement_id: &ManagedBuffer,
        burn_tx_hash: &ManagedBuffer,
    );

    #[event("retirementReverted")]
    fn retirement_reverted_event(&self, #[indexed] retirement_id: &ManagedBuffer);

    #[event("gsocCreditsIssued")]
    fn gsoc_credits_issued_event(
        &self,
        #[indexed] project_id: &ManagedBuffer,
        #[indexed] pai_id: &ManagedBuffer,
        #[indexed] itmo_serial: &ManagedBuffer,
        net_issuable: &BigUint,
    );

    #[event("gsocCreditRetired")]
    fn gsoc_credit_retired_event(
        &self,
        #[indexed] itmo_serial: &ManagedBuffer,
        amount: &BigUint,
        #[indexed] beneficiary_name: &ManagedBuffer,
        #[indexed] beneficiary_address: &ManagedAddress,
    );

    /// M-03 (AUD-008): fires on every partial or full GSOC retirement.
    /// Carries `seq` (per-serial sequence number) as an indexed topic
    /// and a `GsocPartialRetirementEventPayload` with the pair
    /// `{amount_scaled, remaining_after}`. Indexers reconstruct the
    /// per-serial retirement lineage by replaying these events in
    /// `seq` order.
    ///
    /// Event-log framing allows only ONE non-indexed data argument,
    /// so the two `BigUint` values are bundled into a single payload
    /// struct below.
    #[event("gsocSerialPartiallyRetired")]
    fn gsoc_serial_partially_retired_event(
        &self,
        #[indexed] itmo_serial: &ManagedBuffer,
        #[indexed] seq: u64,
        payload: &GsocPartialRetirementEventPayload<Self::Api>,
    );

    /// GSOC bundle hashes keyed by (pai_id, period_key).
    #[storage_mapper("gsocBundles")]
    fn gsoc_bundles(&self) -> MapMapper<(ManagedBuffer, ManagedBuffer), ManagedBuffer>;

    /// GSOC issuances keyed by (project_id, pai_id, period_key).
    #[storage_mapper("gsocIssuances")]
    fn gsoc_issuances(&self) -> MapMapper<(ManagedBuffer, ManagedBuffer, ManagedBuffer), BigUint>;

    /// GSOC serial records: `itmo_serial → (project_id, period, initial_amount)`.
    ///
    /// M-03 (AUD-008): the `BigUint` field is the INITIAL minted
    /// amount and is IMMUTABLE after issuance. It is NOT a running
    /// remaining balance. Consumers that want "how much is left on
    /// this serial" must read `gsoc_serial_remaining` below (falling
    /// back to this initial amount only if the remaining slot has
    /// never been written).
    #[storage_mapper("gsocSerialRecords")]
    fn gsoc_serial_records(&self) -> MapMapper<ManagedBuffer, (ManagedBuffer, u64, BigUint)>;

    /// M-03 (AUD-008): running remaining balance per serial. Only
    /// written by `burn_and_retire_gsoc`. Absent ⇒ no retirements yet
    /// (remaining equals the initial amount on the serial record).
    #[view(getGsocSerialRemaining)]
    #[storage_mapper("gsocSerialRemaining")]
    fn gsoc_serial_remaining(&self, itmo_serial: &ManagedBuffer) -> SingleValueMapper<BigUint>;

    /// M-03 (AUD-008): append-only log of every retirement event that
    /// has touched the given serial. Keyed by `(serial, seq)`;
    /// `seq` is 0-based and strictly increasing per serial.
    #[view(getGsocRetirementEvent)]
    #[storage_mapper("gsocRetirementEvents")]
    fn gsoc_retirement_events(
        &self,
        itmo_serial: &ManagedBuffer,
        seq: u64,
    ) -> SingleValueMapper<GsocRetirementEventRecord<Self::Api>>;

    /// M-03 (AUD-008): next sequence number to be written to
    /// `gsoc_retirement_events` for the given serial. Equal to the
    /// total number of retirements logged for that serial so far.
    #[view(getGsocRetirementSeqCount)]
    #[storage_mapper("gsocRetirementSeqCount")]
    fn gsoc_retirement_seq_count(&self, itmo_serial: &ManagedBuffer) -> SingleValueMapper<u64>;

    /// GSOC serials that have been fully retired.
    #[storage_mapper("gsocRetiredSerials")]
    fn gsoc_retired_serials(&self) -> UnorderedSetMapper<ManagedBuffer>;

    /// Approved GSOC verifiers.
    #[storage_mapper("approvedGsocVerifiers")]
    fn approved_gsoc_verifiers(&self) -> UnorderedSetMapper<ManagedAddress>;

    /// Storage layout version for forward-compatible upgrades.
    #[view(getStorageVersion)]
    #[storage_mapper("storageVersion")]
    fn storage_version(&self) -> SingleValueMapper<u32>;

    /// Upgrades storage layout version if needed and preserves existing state.
    #[upgrade]
    fn upgrade(&self) {
        let current = self.storage_version().get();
        if current < 1u32 {
            self.storage_version().set(1u32);
        }
    }
}
