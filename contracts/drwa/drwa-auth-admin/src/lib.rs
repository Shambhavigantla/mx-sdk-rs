#![no_std]

use multiversx_sc::api::HandleConstraints;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use drwa_common::{
    DrwaCallerDomain, DrwaSyncEnvelope, DrwaSyncOperation, DrwaSyncOperationType,
    build_sync_hook_payload, invoke_drwa_sync_hook, serialize_sync_envelope_payload,
};

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, Clone, PartialEq, Eq)]
pub enum DrwaAuthAction<M: ManagedTypeApi> {
    Nothing,
    UpdateCallerAddress {
        domain: ManagedBuffer<M>,
        new_address: ManagedBuffer<M>,
    },
    AddSigner {
        new_signer: ManagedAddress<M>,
    },
    RemoveSigner {
        signer: ManagedAddress<M>,
    },
    ReplaceSigner {
        old_signer: ManagedAddress<M>,
        new_signer: ManagedAddress<M>,
    },
    ChangeQuorum {
        new_quorum: usize,
    },
}

impl<M: ManagedTypeApi> DrwaAuthAction<M> {
    pub fn is_pending(&self) -> bool {
        !matches!(self, Self::Nothing)
    }
}

// B-03 Option A (AUD-003) — mandatory timelock + fixed-threshold floors.
//
// The approved `DRWA-Key-Rotation-Procedures.md` requires:
//   - fixed 3-of-5 quorum for signer add / revoke
//   - 24-hour timelock after threshold is reached for signer change
//   - 48-hour timelock for recovery-admin rotation
//   - emergency override is a SEPARATE workflow (not implemented here; see
//     the TODO note inside `perform_action`).
//
// We encode the timelock at *propose* time so a later quorum change cannot
// retro-shorten the delay for a pending action. `action_approved_at_round`
// is set when the Nth signature lands and is cleared by `unsign` when the
// approval count drops back below quorum.

/// Minimum number of signers a deployment must carry. Matches the
/// procedures-doc promise of "3-of-5 governance signers." Enforced at
/// `init` AND at the `RemoveSigner` action; any ChangeQuorum that would
/// drop below `DRWA_AUTH_MIN_QUORUM` is rejected.
const DRWA_AUTH_MIN_SIGNER_COUNT: usize = 5;

/// Minimum quorum. See `DRWA_AUTH_MIN_SIGNER_COUNT`.
const DRWA_AUTH_MIN_QUORUM: usize = 3;

/// Block-round equivalent of a 24-hour timelock at ~6 seconds per round.
/// Applied by default to every admin action. Signer add / remove /
/// replace / caller-address updates and quorum changes all land on this
/// delay unless an operator explicitly configures a longer one for a
/// specific domain.
const DRWA_AUTH_TIMELOCK_DEFAULT_ROUNDS: u64 = 14_400;

/// Block-round equivalent of a 48-hour timelock. Applied when the action
/// targets the `recovery-admin` caller domain (see the match arm in
/// `propose_update_caller_address` / resolution inside `create_action`).
const DRWA_AUTH_TIMELOCK_RECOVERY_ADMIN_ROUNDS: u64 = 28_800;

// B-03: the recovery-admin caller-domain tag is `b"recovery_admin"`,
// matching `DrwaCallerDomain::RecoveryAdmin` serialization in
// `drwa_common`. We compare against the byte-array literal directly at
// the callsite because `ManagedBuffer` only implements PartialEq against
// fixed-size byte-array literals (`&[u8; N]`), not arbitrary `&[u8]`
// slice references.

#[multiversx_sc::contract]
pub trait DrwaAuthAdmin {
    #[init]
    fn init(
        &self,
        quorum: usize,
        proposal_ttl_rounds: u64,
        signers: MultiValueEncoded<ManagedAddress>,
    ) {
        require!(proposal_ttl_rounds > 0, "proposal TTL must be > 0");

        let mut signer_count = 0usize;
        for signer in signers {
            require!(!signer.is_zero(), "signer must not be zero");
            require!(!self.signers().contains(&signer), "duplicate signer");
            self.signers().insert(signer);
            signer_count += 1;
        }

        require!(signer_count > 0, "signers must not be empty");
        require!(quorum > 0, "quorum must be > 0");
        require!(quorum <= signer_count, "quorum exceeds signer count");

        // B-03 (AUD-003): the DRWA-Key-Rotation-Procedures doc commits the
        // deployment to at least 3-of-5. Reject any init that cannot honor
        // that contract. Existing deployments already at 3-of-5 or stronger
        // are unaffected; weaker configurations were never safe.
        require!(
            signer_count >= DRWA_AUTH_MIN_SIGNER_COUNT,
            "signer count below procedure floor (3-of-5)"
        );
        require!(
            quorum >= DRWA_AUTH_MIN_QUORUM,
            "quorum below procedure floor (3-of-5)"
        );

        self.quorum().set(quorum);
        self.proposal_ttl_rounds().set(proposal_ttl_rounds);
        self.next_action_id().set(1u64);
    }

    #[upgrade]
    fn upgrade(
        &self,
        quorum: usize,
        proposal_ttl_rounds: u64,
        signers: MultiValueEncoded<ManagedAddress>,
    ) {
        if self.next_action_id().is_empty() {
            self.init(quorum, proposal_ttl_rounds, signers);
        }
    }

    #[endpoint(proposeUpdateCallerAddress)]
    fn propose_update_caller_address(
        &self,
        domain: ManagedBuffer,
        new_address: ManagedBuffer,
    ) -> u64 {
        self.require_signer();
        require!(!domain.is_empty(), "domain must not be empty");
        self.require_valid_authorized_caller(&new_address);
        // B-03: caller-address updates for the recovery-admin domain carry a
        // 48-hour timelock per procedure §5.2 / §6.2. All other domains use
        // the default 24-hour delay.
        let timelock_rounds = if domain == b"recovery_admin" {
            DRWA_AUTH_TIMELOCK_RECOVERY_ADMIN_ROUNDS
        } else {
            DRWA_AUTH_TIMELOCK_DEFAULT_ROUNDS
        };
        self.create_action_with_timelock(
            DrwaAuthAction::UpdateCallerAddress {
                domain,
                new_address,
            },
            timelock_rounds,
        )
    }

    #[endpoint(proposeAddSigner)]
    fn propose_add_signer(&self, new_signer: ManagedAddress) -> u64 {
        self.require_signer();
        require!(!new_signer.is_zero(), "signer must not be zero");
        require!(
            !self.signers().contains(&new_signer),
            "signer already exists"
        );
        self.create_action_with_timelock(
            DrwaAuthAction::AddSigner { new_signer },
            DRWA_AUTH_TIMELOCK_DEFAULT_ROUNDS,
        )
    }

    #[endpoint(proposeRemoveSigner)]
    fn propose_remove_signer(&self, signer: ManagedAddress) -> u64 {
        self.require_signer();
        require!(!signer.is_zero(), "signer must not be zero");
        require!(self.signers().contains(&signer), "signer not found");
        self.create_action_with_timelock(
            DrwaAuthAction::RemoveSigner { signer },
            DRWA_AUTH_TIMELOCK_DEFAULT_ROUNDS,
        )
    }

    #[endpoint(proposeReplaceSigner)]
    fn propose_replace_signer(
        &self,
        old_signer: ManagedAddress,
        new_signer: ManagedAddress,
    ) -> u64 {
        self.require_signer();
        require!(!old_signer.is_zero(), "old signer must not be zero");
        require!(!new_signer.is_zero(), "new signer must not be zero");
        require!(self.signers().contains(&old_signer), "old signer not found");
        require!(
            !self.signers().contains(&new_signer),
            "new signer already exists"
        );
        self.create_action_with_timelock(
            DrwaAuthAction::ReplaceSigner {
                old_signer,
                new_signer,
            },
            DRWA_AUTH_TIMELOCK_DEFAULT_ROUNDS,
        )
    }

    #[endpoint(proposeChangeQuorum)]
    fn propose_change_quorum(&self, new_quorum: usize) -> u64 {
        self.require_signer();
        self.create_action_with_timelock(
            DrwaAuthAction::ChangeQuorum { new_quorum },
            DRWA_AUTH_TIMELOCK_DEFAULT_ROUNDS,
        )
    }

    #[endpoint(sign)]
    fn sign(&self, action_id: u64) {
        self.require_signer();
        self.require_pending_action(action_id);
        let caller = self.blockchain().get_caller();
        if !self.action_signers(action_id).contains(&caller) {
            self.action_signers(action_id).insert(caller);
        }
        // B-03: capture the round on which quorum is first reached; the
        // timelock window is measured from this instant, not from the
        // initial propose. If quorum is later lost via `unsign` the
        // stored round is cleared so a subsequent re-approval restarts
        // the timelock. This matches the procedure-doc intent of
        // "24-hour timelock AFTER threshold reached."
        //
        // Encoding: stored value = round + 1 so that a round-0 approval
        // is distinguishable from "never approved" (`SingleValueMapper<u64>`
        // reports a storage slot containing 0 as `is_empty()==true`, so
        // the offset ensures the slot always carries a non-zero value
        // once set).
        if self.action_approved_at_round(action_id).is_empty()
            && self.action_signers(action_id).len() >= self.quorum().get()
        {
            let current = self.blockchain().get_block_round();
            self.action_approved_at_round(action_id)
                .set(current.saturating_add(1));
        }
    }

    #[endpoint(unsign)]
    fn unsign(&self, action_id: u64) {
        self.require_signer();
        self.require_pending_action(action_id);
        let caller = self.blockchain().get_caller();
        self.action_signers(action_id).swap_remove(&caller);
        // B-03: if approvals drop back below quorum after this unsign,
        // invalidate the previously captured approval round so the
        // timelock cannot be satisfied by a brief-quorum / retract /
        // re-quorum dance that evades the full delay window.
        if self.action_signers(action_id).len() < self.quorum().get() {
            self.action_approved_at_round(action_id).clear();
        }
    }

    #[endpoint(discardAction)]
    fn discard_action(&self, action_id: u64) {
        self.require_signer();
        self.require_pending_action(action_id);
        let current_round = self.blockchain().get_block_round();
        let expiry_round = self.action_expiry_round(action_id).get();
        require!(
            current_round > expiry_round || self.action_signers(action_id).is_empty(),
            "cannot discard active action"
        );
        self.clear_action(action_id);
    }

    #[endpoint(performAction)]
    fn perform_action(&self, action_id: u64) -> OptionalValue<DrwaSyncEnvelope<Self::Api>> {
        self.require_signer();
        require!(
            !self.performed_action_ids().contains(&action_id),
            "action already performed"
        );
        self.require_pending_action(action_id);
        let current_round = self.blockchain().get_block_round();
        require!(
            current_round <= self.action_expiry_round(action_id).get(),
            "action expired"
        );
        require!(
            self.action_signers(action_id).len() >= self.quorum().get(),
            "insufficient approvals"
        );

        // B-03 (AUD-003): enforce the mandatory post-quorum timelock. The
        // storage key is only populated AFTER quorum is first reached
        // (see `sign`), so a missing entry here means the action has
        // reached `len() >= quorum` only via the post-quorum increment
        // path — defensive require! catches an impossible code path.
        //
        // TODO(emergency-override): the procedure doc permits a `No
        // timelock (immediate)` emergency signer rotation path. That
        // workflow is not implemented here because it requires
        // additional controls (incident-report attestation CID, higher
        // approval count, auditable justification). Until that
        // workflow lands, all admin actions inherit the full timelock.
        require!(
            !self.action_approved_at_round(action_id).is_empty(),
            "approval round not recorded; sign again after reaching quorum"
        );
        // Stored as `round + 1` (see `sign`). Subtract the offset here to
        // recover the true round at which quorum was reached.
        let approved_round = self
            .action_approved_at_round(action_id)
            .get()
            .saturating_sub(1);
        let timelock_rounds = self.action_timelock_rounds(action_id).get();
        require!(
            current_round >= approved_round.saturating_add(timelock_rounds),
            "timelock not elapsed: must wait 24h after quorum (48h for recovery-admin)"
        );

        let action = self.actions(action_id).get();
        let mut result = OptionalValue::None;

        match action {
            DrwaAuthAction::Nothing => sc_panic!("action does not exist"),
            DrwaAuthAction::UpdateCallerAddress {
                domain,
                new_address,
            } => {
                require!(!domain.is_empty(), "domain must not be empty");
                self.require_valid_authorized_caller(&new_address);

                let next_version = self
                    .authorized_caller_version(&domain)
                    .get()
                    .checked_add(1)
                    .unwrap_or_else(|| sc_panic!("version overflow"));
                self.authorized_caller(&domain).set(&new_address);
                self.authorized_caller_version(&domain).set(next_version);

                let mut operations = ManagedVec::new();
                operations.push(DrwaSyncOperation {
                    operation_type: DrwaSyncOperationType::AuthorizedCallerUpdate,
                    token_id: domain,
                    holder: ManagedAddress::zero(),
                    version: next_version,
                    body: new_address,
                });

                result = OptionalValue::Some(
                    self.emit_sync_envelope(DrwaCallerDomain::AuthAdmin, operations),
                );
            }
            DrwaAuthAction::AddSigner { new_signer } => {
                require!(!new_signer.is_zero(), "signer must not be zero");
                require!(
                    !self.signers().contains(&new_signer),
                    "signer already exists"
                );
                self.signers().insert(new_signer);
            }
            DrwaAuthAction::RemoveSigner { signer } => {
                require!(self.signers().contains(&signer), "signer not found");
                require!(
                    self.signers().len() > self.quorum().get(),
                    "cannot remove signer below quorum"
                );
                // B-03: procedure floor — deployment must keep at least
                // `DRWA_AUTH_MIN_SIGNER_COUNT` signers regardless of what
                // the configured quorum would otherwise permit.
                require!(
                    self.signers().len() - 1 >= DRWA_AUTH_MIN_SIGNER_COUNT,
                    "cannot drop signer count below procedure floor (3-of-5)"
                );
                self.signers().swap_remove(&signer);
            }
            DrwaAuthAction::ReplaceSigner {
                old_signer,
                new_signer,
            } => {
                require!(self.signers().contains(&old_signer), "old signer not found");
                require!(!new_signer.is_zero(), "new signer must not be zero");
                require!(
                    !self.signers().contains(&new_signer),
                    "new signer already exists"
                );
                self.signers().swap_remove(&old_signer);
                self.signers().insert(new_signer);
            }
            DrwaAuthAction::ChangeQuorum { new_quorum } => {
                require!(new_quorum > 0, "quorum must be > 0");
                require!(
                    new_quorum <= self.signers().len(),
                    "quorum exceeds signer count"
                );
                // B-03: procedure floor — quorum must remain >= 3.
                require!(
                    new_quorum >= DRWA_AUTH_MIN_QUORUM,
                    "quorum below procedure floor (3-of-5)"
                );
                self.quorum().set(new_quorum);
            }
        }

        self.performed_action_ids().insert(action_id);
        self.clear_action(action_id);
        result
    }

    #[view(getQuorum)]
    #[storage_mapper("quorum")]
    fn quorum(&self) -> SingleValueMapper<usize>;

    #[view(getProposalTtlRounds)]
    #[storage_mapper("proposalTtlRounds")]
    fn proposal_ttl_rounds(&self) -> SingleValueMapper<u64>;

    #[view(getNextActionId)]
    #[storage_mapper("nextActionId")]
    fn next_action_id(&self) -> SingleValueMapper<u64>;

    #[view(getAction)]
    #[storage_mapper("actions")]
    fn actions(&self, action_id: u64) -> SingleValueMapper<DrwaAuthAction<Self::Api>>;

    #[view(getActionProposer)]
    #[storage_mapper("actionProposer")]
    fn action_proposer(&self, action_id: u64) -> SingleValueMapper<ManagedAddress>;

    #[view(getActionCreatedRound)]
    #[storage_mapper("actionCreatedRound")]
    fn action_created_round(&self, action_id: u64) -> SingleValueMapper<u64>;

    #[view(getActionExpiryRound)]
    #[storage_mapper("actionExpiryRound")]
    fn action_expiry_round(&self, action_id: u64) -> SingleValueMapper<u64>;

    /// B-03 (AUD-003): round at which the action's approval count first
    /// reached `quorum`. Populated by `sign` when the quorum threshold
    /// is crossed upward; cleared by `unsign` when it drops back below.
    /// `performAction` rejects when this is empty (quorum never reached)
    /// or when `current_round < approved_round + timelock_rounds`.
    ///
    /// Encoding: the stored value is `approved_round + 1`. The `+ 1`
    /// offset lets `SingleValueMapper::is_empty()` act as the "not yet
    /// approved" sentinel even when quorum is reached at block-round 0
    /// (which is legitimate in scenario tests; no effect in real chain
    /// execution where block-round 0 is the genesis slot). Consumers of
    /// this view MUST subtract 1 to recover the real approval round.
    #[view(getActionApprovedAtRound)]
    #[storage_mapper("actionApprovedAtRound")]
    fn action_approved_at_round(&self, action_id: u64) -> SingleValueMapper<u64>;

    /// B-03 (AUD-003): timelock window in block rounds that must elapse
    /// between `action_approved_at_round` and a successful
    /// `performAction`. Set at propose time and immutable thereafter
    /// so a later `ChangeQuorum` cannot retro-shorten pending actions.
    #[view(getActionTimelockRounds)]
    #[storage_mapper("actionTimelockRounds")]
    fn action_timelock_rounds(&self, action_id: u64) -> SingleValueMapper<u64>;

    #[view(getAllSigners)]
    #[storage_mapper("signers")]
    fn signers(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getActionSigners)]
    #[storage_mapper("actionSigners")]
    fn action_signers(&self, action_id: u64) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getPerformedActionIds)]
    #[storage_mapper("performedActionIds")]
    fn performed_action_ids(&self) -> UnorderedSetMapper<u64>;

    #[view(getAuthorizedCaller)]
    #[storage_mapper("authorizedCaller")]
    fn authorized_caller(&self, domain: &ManagedBuffer) -> SingleValueMapper<ManagedBuffer>;

    #[view(getAuthorizedCallerVersion)]
    #[storage_mapper("authorizedCallerVersion")]
    fn authorized_caller_version(&self, domain: &ManagedBuffer) -> SingleValueMapper<u64>;

    fn require_signer(&self) {
        let caller = self.blockchain().get_caller();
        require!(self.signers().contains(&caller), "caller not a signer");
    }

    fn require_pending_action(&self, action_id: u64) {
        require!(
            !self.actions(action_id).is_empty() && self.actions(action_id).get().is_pending(),
            "action does not exist"
        );
    }

    /// Legacy shim: proposals that don't specify a timelock use the
    /// default 24-hour delay. All current in-tree callers go through
    /// `create_action_with_timelock` directly; this helper remains for
    /// backward-compat with any external code that calls it.
    fn create_action(&self, action: DrwaAuthAction<Self::Api>) -> u64 {
        self.create_action_with_timelock(action, DRWA_AUTH_TIMELOCK_DEFAULT_ROUNDS)
    }

    fn create_action_with_timelock(
        &self,
        action: DrwaAuthAction<Self::Api>,
        timelock_rounds: u64,
    ) -> u64 {
        // B-03: timelock must be positive. Zero-timelock is reserved for
        // the as-yet-unimplemented emergency-override workflow; no
        // regular propose path may set it.
        require!(
            timelock_rounds > 0,
            "action timelock must be > 0 (emergency override not supported)"
        );

        let action_id = self.next_action_id().get();
        self.next_action_id().set(action_id + 1);
        self.actions(action_id).set(action);
        self.action_proposer(action_id)
            .set(self.blockchain().get_caller());
        let current_round = self.blockchain().get_block_round();
        self.action_created_round(action_id).set(current_round);
        self.action_expiry_round(action_id)
            .set(current_round.saturating_add(self.proposal_ttl_rounds().get()));
        self.action_timelock_rounds(action_id).set(timelock_rounds);
        self.action_signers(action_id)
            .insert(self.blockchain().get_caller());
        // Propose-time quorum check: the proposer counts as the first
        // approval. If `quorum == 1` the action becomes immediately
        // eligible (from a quorum standpoint) and the timelock starts
        // here; otherwise the approval round is set when the Nth
        // signer calls `sign`. Same offset-by-one encoding as `sign`.
        if self.action_signers(action_id).len() >= self.quorum().get() {
            self.action_approved_at_round(action_id)
                .set(current_round.saturating_add(1));
        }
        action_id
    }

    fn clear_action(&self, action_id: u64) {
        self.actions(action_id).clear();
        self.action_proposer(action_id).clear();
        self.action_created_round(action_id).clear();
        self.action_expiry_round(action_id).clear();
        self.action_signers(action_id).clear();
        // B-03: also clear the new timelock tracking slots to avoid
        // orphan state after `discardAction` or `performAction`.
        self.action_approved_at_round(action_id).clear();
        self.action_timelock_rounds(action_id).clear();
    }

    fn require_valid_authorized_caller(&self, new_address: &ManagedBuffer) {
        require!(!new_address.is_empty(), "new address must not be empty");

        let len = new_address.len();
        require!(
            len <= 90,
            "new address must be a 64-char hex string or erd1 bech32 address"
        );

        let mut bytes = [0u8; 90];
        new_address.load_slice(0, &mut bytes[..len]);
        let address = &bytes[..len];

        let is_hex = len == 64 && address.iter().all(|b| b.is_ascii_hexdigit());
        let is_bech32 = len >= 4
            && address.starts_with(b"erd1")
            && address
                .iter()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit());

        require!(
            is_hex || is_bech32,
            "new address must be a 64-char hex string or erd1 bech32 address"
        );
    }

    fn emit_sync_envelope(
        &self,
        caller_domain: DrwaCallerDomain,
        operations: ManagedVec<DrwaSyncOperation<Self::Api>>,
    ) -> DrwaSyncEnvelope<Self::Api> {
        let payload_hash = self
            .crypto()
            .keccak256(serialize_sync_envelope_payload(&caller_domain, &operations))
            .as_managed_buffer()
            .clone();

        let hook_payload = build_sync_hook_payload(&caller_domain, &operations, &payload_hash);
        require!(
            invoke_drwa_sync_hook(hook_payload.get_handle().get_raw_handle()) == 0,
            "native mirror sync failed"
        );

        DrwaSyncEnvelope {
            caller_domain,
            payload_hash,
            operations,
        }
    }
}
