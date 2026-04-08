#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod drwa_identity_registry_proxy;

use multiversx_sc::api::HandleConstraints;

use drwa_common::{
    DrwaCallerDomain, DrwaHolderProfile, DrwaSyncEnvelope, DrwaSyncOperation,
    DrwaSyncOperationType, build_sync_hook_payload, invoke_drwa_sync_hook,
    push_len_prefixed, require_valid_aml_status, require_valid_kyc_status,
    serialize_sync_envelope_payload,
};

const DEFAULT_IDENTITY_VALIDITY_ROUNDS: u64 = 10_000;
const MAX_IDENTITY_VALIDITY_ROUNDS: u64 = 100_000;

/// Stores the identity data tracked for a holder address.
///
/// The `subject` field is stored in the value as well as used as the storage
/// key so off-chain consumers can read it without reconstructing the key.
#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, Clone)]
pub struct IdentityRecord<M: ManagedTypeApi> {
    pub subject: ManagedAddress<M>,
    pub legal_name: ManagedBuffer<M>,
    pub jurisdiction_code: ManagedBuffer<M>,
    pub registration_number: ManagedBuffer<M>,
    pub entity_type: ManagedBuffer<M>,
    pub kyc_status: ManagedBuffer<M>,
    pub aml_status: ManagedBuffer<M>,
    pub investor_class: ManagedBuffer<M>,
    pub expiry_round: u64,
}

/// Manages per-holder identity records (KYC, AML, investor class, jurisdiction)
/// and syncs holder-profile state to the native DRWA mirror on every mutation.
///
/// Governance is transferable via a propose-accept pattern with a time-limited
/// acceptance window.
#[multiversx_sc::contract]
pub trait DrwaIdentityRegistry: drwa_common::DrwaGovernanceModule {
    /// Initializes the contract with the governance address.
    #[init]
    fn init(&self, governance: ManagedAddress) {
        require!(!governance.is_zero(), "governance must not be zero");
        self.governance().set(governance);
        self.default_validity_rounds().set(DEFAULT_IDENTITY_VALIDITY_ROUNDS);
        self.max_validity_rounds().set(MAX_IDENTITY_VALIDITY_ROUNDS);
    }

    /// Registers a new identity for `subject`.
    ///
    /// Sets both KYC and AML status to `"pending"` and sets the initial expiry
    /// round to the current block round plus `DEFAULT_IDENTITY_VALIDITY_ROUNDS`.
    /// Access is limited to the governance address or the contract owner.
    /// Reverts if `subject` is the zero address or an identity already exists.
    #[endpoint(registerIdentity)]
    fn register_identity(
        &self,
        subject: ManagedAddress,
        legal_name: ManagedBuffer,
        jurisdiction_code: ManagedBuffer,
        registration_number: ManagedBuffer,
        entity_type: ManagedBuffer,
    ) -> DrwaSyncEnvelope<Self::Api> {
        self.require_governance_or_owner();
        require!(!subject.is_zero(), "subject must not be zero");
        require!(
            self.identity(&subject).is_empty(),
            "IDENTITY_ALREADY_REGISTERED: use updateComplianceStatus to modify existing identity"
        );

        let record = IdentityRecord {
            subject: subject.clone(),
            legal_name,
            jurisdiction_code,
            registration_number,
            entity_type,
            kyc_status: ManagedBuffer::from(b"pending"),
            aml_status: ManagedBuffer::from(b"pending"),
            investor_class: ManagedBuffer::new(),
            expiry_round: self
                .blockchain()
                .get_block_round()
                .saturating_add(self.default_validity_rounds().get()),
        };

        self.identity(&subject).set(record.clone());
        self.emit_holder_profile_sync(subject, &record)
    }

    /// Updates the compliance fields for an existing identity and syncs the
    /// holder profile to the native mirror.
    ///
    /// Access is limited to the governance address or the contract owner.
    /// Reverts if the subject is missing, `expiry_round` is in the past
    /// unless it is `0`, or `expiry_round` exceeds the configured maximum
    /// validity window.
    #[endpoint(updateComplianceStatus)]
    fn update_compliance_status(
        &self,
        subject: ManagedAddress,
        kyc_status: ManagedBuffer,
        aml_status: ManagedBuffer,
        investor_class: ManagedBuffer,
        expiry_round: u64,
    ) -> DrwaSyncEnvelope<Self::Api> {
        self.require_governance_or_owner();
        require!(!subject.is_zero(), "subject must not be zero");
        require_valid_kyc_status(&kyc_status);
        require_valid_aml_status(&aml_status);
        require!(
            !self.identity(&subject).is_empty(),
            "identity not registered - call registerIdentity first"
        );
        let current_round = self.blockchain().get_block_round();
        require!(
            expiry_round == 0 || expiry_round > current_round,
            "expiry_round must be in the future or 0 for permanent"
        );
        require!(
            expiry_round == 0
                || expiry_round
                    <= current_round.saturating_add(self.max_validity_rounds().get()),
            "expiry_round exceeds maximum identity validity window"
        );

        self.identity(&subject).update(|record| {
            record.kyc_status = kyc_status;
            record.aml_status = aml_status;
            record.investor_class = investor_class;
            record.expiry_round = expiry_round;
        });
        let record = self.identity(&subject).get();
        self.emit_holder_profile_sync(subject, &record)
    }

    /// Deactivates an existing identity by setting both KYC and AML status to
    /// `"deactivated"`, incrementing the holder profile version, and syncing
    /// the change to the native mirror.
    ///
    /// This preserves the audit trail (the record is not deleted).
    /// Access is limited to the governance address or the contract owner.
    /// Reverts if the identity does not exist or `subject` is the zero address.
    #[endpoint(deactivateIdentity)]
    fn deactivate_identity(&self, subject: ManagedAddress) -> DrwaSyncEnvelope<Self::Api> {
        self.require_governance_or_owner();
        require!(!subject.is_zero(), "subject address must not be zero");
        require!(
            !self.identity(&subject).is_empty(),
            "identity not registered"
        );

        self.identity(&subject).update(|record| {
            record.kyc_status = ManagedBuffer::from(b"deactivated");
            record.aml_status = ManagedBuffer::from(b"deactivated");
        });
        let record = self.identity(&subject).get();
        self.emit_holder_profile_sync(subject, &record)
    }

    /// Maps a holder address to its identity record.
    #[view(getIdentity)]
    #[storage_mapper("identity")]
    fn identity(&self, subject: &ManagedAddress) -> SingleValueMapper<IdentityRecord<Self::Api>>;

    /// Monotonically increasing version counter per holder, used for
    /// staleness detection.
    #[storage_mapper("holderProfileVersion")]
    fn holder_profile_version(&self, subject: &ManagedAddress) -> SingleValueMapper<u64>;

    /// Storage-backed default identity validity window (in rounds).
    /// Initialized from `DEFAULT_IDENTITY_VALIDITY_ROUNDS` during `init`.
    #[storage_mapper("default_validity_rounds")]
    fn default_validity_rounds(&self) -> SingleValueMapper<u64>;

    /// Storage-backed maximum identity validity window (in rounds).
    /// Initialized from `MAX_IDENTITY_VALIDITY_ROUNDS` during `init`.
    #[storage_mapper("max_validity_rounds")]
    fn max_validity_rounds(&self) -> SingleValueMapper<u64>;

    /// Updates the identity validity configuration.
    ///
    /// Access is limited to the governance address or the contract owner.
    /// Reverts if `default_rounds` is zero, `max_rounds` is less than
    /// `default_rounds`, or `max_rounds` exceeds the hard cap of 1,000,000.
    #[endpoint(setValidityConfig)]
    fn set_validity_config(&self, default_rounds: u64, max_rounds: u64) {
        self.require_governance_or_owner();
        require!(default_rounds > 0, "default_rounds must be positive");
        require!(max_rounds >= default_rounds, "max_rounds must be >= default_rounds");
        require!(max_rounds <= 1_000_000, "max_rounds cap exceeded");
        self.default_validity_rounds().set(default_rounds);
        self.max_validity_rounds().set(max_rounds);
    }

    /// Builds, stores, and emits the holder-profile sync payload sent to the
    /// native mirror.
    fn emit_holder_profile_sync(
        &self,
        subject: ManagedAddress,
        record: &IdentityRecord<Self::Api>,
    ) -> DrwaSyncEnvelope<Self::Api> {
        let next_version = self.holder_profile_version(&subject).get() + 1;
        let profile = DrwaHolderProfile {
            holder_profile_version: next_version,
            kyc_status: record.kyc_status.clone(),
            aml_status: record.aml_status.clone(),
            investor_class: record.investor_class.clone(),
            jurisdiction_code: record.jurisdiction_code.clone(),
            expiry_round: record.expiry_round,
        };

        self.holder_profile_version(&subject).set(next_version);

        let body = self.serialize_holder_profile(&profile);
        let mut operations = ManagedVec::new();
        operations.push(DrwaSyncOperation {
            operation_type: DrwaSyncOperationType::HolderProfile,
            token_id: ManagedBuffer::new(),
            holder: subject.clone(),
            version: next_version,
            body,
        });

        let caller_domain = DrwaCallerDomain::IdentityRegistry;
        let payload_hash = self
            .crypto()
            .keccak256(&serialize_sync_envelope_payload(
                &caller_domain,
                &operations,
            ))
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

    /// Serializes the holder profile in the binary field order consumed by the
    /// native mirror.
    fn serialize_holder_profile(&self, profile: &DrwaHolderProfile<Self::Api>) -> ManagedBuffer {
        let mut result = ManagedBuffer::new();
        result.append_bytes(&profile.holder_profile_version.to_be_bytes());
        push_len_prefixed(&mut result, &profile.kyc_status);
        push_len_prefixed(&mut result, &profile.aml_status);
        push_len_prefixed(&mut result, &profile.investor_class);
        push_len_prefixed(&mut result, &profile.jurisdiction_code);
        result.append_bytes(&profile.expiry_round.to_be_bytes());
        result
    }

    /// Leaves storage unchanged during contract upgrades.
    #[upgrade]
    fn upgrade(&self) {}
}
