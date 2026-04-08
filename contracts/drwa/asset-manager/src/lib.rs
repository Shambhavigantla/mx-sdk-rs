#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod drwa_asset_manager_proxy;

use multiversx_sc::api::HandleConstraints;

use drwa_common::{
    DrwaCallerDomain, DrwaHolderMirror, DrwaSyncEnvelope, DrwaSyncOperation, DrwaSyncOperationType,
    build_sync_hook_payload, invoke_drwa_sync_hook, push_len_prefixed, require_valid_aml_status,
    require_valid_kyc_status, require_valid_token_id, serialize_sync_envelope_payload,
};

/// Stores the regulated asset metadata associated with a token identifier.
#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, Clone)]
pub struct AssetRecord<M: ManagedTypeApi> {
    pub token_id: ManagedBuffer<M>,
    pub carrier_type: ManagedBuffer<M>,
    pub asset_class: ManagedBuffer<M>,
    pub policy_id: ManagedBuffer<M>,
    pub regulated: bool,
}

/// Event payload emitted when holder compliance data is updated.
#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, Clone)]
pub struct HolderComplianceEventPayload<M: ManagedTypeApi> {
    pub holder_policy_version: u64,
    pub kyc_status: ManagedBuffer<M>,
    pub aml_status: ManagedBuffer<M>,
    pub investor_class: ManagedBuffer<M>,
    pub jurisdiction_code: ManagedBuffer<M>,
    pub expiry_round: u64,
    pub transfer_locked: bool,
    pub receive_locked: bool,
    pub auditor_authorized: bool,
}

/// Manages regulated asset registration and per-holder, per-token compliance
/// state. Syncs both asset records and holder compliance mirrors to the native
/// DRWA layer on every mutation.
///
/// Governance is transferable via a propose-accept pattern with a time-limited
/// acceptance window.
#[multiversx_sc::contract]
pub trait DrwaAssetManager: drwa_common::DrwaGovernanceModule {
    /// Initializes the contract with the governance address.
    #[init]
    fn init(&self, governance: ManagedAddress) {
        require!(!governance.is_zero(), "governance must not be zero");
        self.governance().set(&governance);
    }

    /// Registers a new regulated asset and syncs it to the native mirror.
    ///
    /// Access is limited to the governance address or the contract owner.
    /// Stores `regulated = true` for the new asset.
    /// Reverts if `token_id` is invalid or the asset is already registered.
    #[endpoint(registerAsset)]
    fn register_asset(
        &self,
        token_id: ManagedBuffer,
        carrier_type: ManagedBuffer,
        asset_class: ManagedBuffer,
        policy_id: ManagedBuffer,
    ) -> DrwaSyncEnvelope<Self::Api> {
        self.require_governance_or_owner();

        self.require_valid_token_id(&token_id);
        require!(
            self.asset(&token_id).is_empty(),
            "asset already registered - use an upgrade endpoint to modify"
        );

        self.asset(&token_id).set(AssetRecord {
            token_id: token_id.clone(),
            carrier_type,
            asset_class,
            policy_id: policy_id.clone(),
            regulated: true,
        });
        self.drwa_asset_registered_event(&token_id, &policy_id, true);

        // The sync body uses `token_id:policy_id`, and validated token IDs do
        // not contain `:`, so the separator remains unambiguous.
        let mut body = ManagedBuffer::new();
        body.append(&token_id);
        body.append_bytes(b":");
        body.append(&policy_id);

        let mut operations = ManagedVec::new();
        operations.push(DrwaSyncOperation {
            operation_type: DrwaSyncOperationType::AssetRecord,
            token_id: token_id.clone(),
            holder: ManagedAddress::default(),
            version: 1u64,
            body,
        });

        let caller_domain = DrwaCallerDomain::AssetManager;
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

    /// Writes per-holder, per-token compliance state and syncs it to the
    /// native mirror.
    ///
    /// Access is limited to the governance address or the contract owner.
    /// Increments the holder policy version monotonically.
    /// Reverts if `holder` is the zero address, `token_id` is invalid,
    /// `expiry_round` is in the past unless it is `0`, or the native mirror
    /// sync fails.
    #[endpoint(syncHolderCompliance)]
    fn sync_holder_compliance(
        &self,
        token_id: ManagedBuffer,
        holder: ManagedAddress,
        kyc_status: ManagedBuffer,
        aml_status: ManagedBuffer,
        investor_class: ManagedBuffer,
        jurisdiction_code: ManagedBuffer,
        expiry_round: u64,
        transfer_locked: bool,
        receive_locked: bool,
        auditor_authorized: bool,
    ) -> DrwaSyncEnvelope<Self::Api> {
        self.require_governance_or_owner();
        require!(!holder.is_zero(), "ZERO_ADDRESS: holder must not be zero");

        self.require_valid_token_id(&token_id);
        require!(
            !self.asset(&token_id).is_empty(),
            "asset not registered: use registerAsset first"
        );
        require_valid_kyc_status(&kyc_status);
        require_valid_aml_status(&aml_status);

        let current_round = self.blockchain().get_block_round();
        require!(
            expiry_round == 0 || expiry_round > current_round,
            "expiry_round must be in the future or 0 for permanent"
        );

        let next_version = self.holder_policy_version(&token_id, &holder).get() + 1;

        let mirror = DrwaHolderMirror {
            holder_policy_version: next_version,
            kyc_status,
            aml_status,
            investor_class,
            jurisdiction_code,
            expiry_round,
            transfer_locked,
            receive_locked,
            auditor_authorized,
        };

        self.holder_mirror(&token_id, &holder).set(mirror.clone());
        self.holder_policy_version(&token_id, &holder)
            .set(next_version);
        self.drwa_holder_compliance_event(
            &token_id,
            &holder,
            &HolderComplianceEventPayload {
                holder_policy_version: next_version,
                kyc_status: mirror.kyc_status.clone(),
                aml_status: mirror.aml_status.clone(),
                investor_class: mirror.investor_class.clone(),
                jurisdiction_code: mirror.jurisdiction_code.clone(),
                expiry_round: mirror.expiry_round,
                transfer_locked: mirror.transfer_locked,
                receive_locked: mirror.receive_locked,
                auditor_authorized: mirror.auditor_authorized,
            },
        );

        let body = self.serialize_holder(&mirror);
        let mut operations = ManagedVec::new();
        operations.push(DrwaSyncOperation {
            operation_type: DrwaSyncOperationType::HolderMirror,
            token_id: token_id.clone(),
            holder: holder.clone(),
            version: next_version,
            body,
        });

        let caller_domain = DrwaCallerDomain::AssetManager;
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

    /// Updates the carrier_type, asset_class, and policy_id of an existing
    /// registered asset. Does not re-register or change the `regulated` flag.
    ///
    /// Access is limited to the governance address or the contract owner.
    /// Reverts if the asset is not registered.
    #[endpoint(updateAsset)]
    fn update_asset(
        &self,
        token_id: ManagedBuffer,
        carrier_type: ManagedBuffer,
        asset_class: ManagedBuffer,
        policy_id: ManagedBuffer,
    ) {
        self.require_governance_or_owner();
        self.require_valid_token_id(&token_id);
        require!(
            !self.asset(&token_id).is_empty(),
            "asset not registered: use registerAsset first"
        );

        self.asset(&token_id).update(|record| {
            record.carrier_type = carrier_type;
            record.asset_class = asset_class;
            record.policy_id = policy_id;
        });
    }

    /// Maps a token identifier to its regulated asset record.
    #[view(getAsset)]
    #[storage_mapper("asset")]
    fn asset(&self, token_id: &ManagedBuffer) -> SingleValueMapper<AssetRecord<Self::Api>>;

    /// Returns the holder compliance mirror for a given (token_id, holder) pair.
    #[view(getHolderMirror)]
    fn get_holder_mirror(
        &self,
        token_id: ManagedBuffer,
        holder: ManagedAddress,
    ) -> DrwaHolderMirror<Self::Api> {
        require!(
            !self.holder_mirror(&token_id, &holder).is_empty(),
            "holder mirror not found"
        );
        self.holder_mirror(&token_id, &holder).get()
    }

    /// Maps (token_id, holder) to the holder's compliance mirror state.
    #[storage_mapper("holderMirror")]
    fn holder_mirror(
        &self,
        token_id: &ManagedBuffer,
        holder: &ManagedAddress,
    ) -> SingleValueMapper<DrwaHolderMirror<Self::Api>>;

    /// Monotonically increasing version counter per `(token_id, holder)` pair,
    /// used for staleness detection.
    #[storage_mapper("holderPolicyVersion")]
    fn holder_policy_version(
        &self,
        token_id: &ManagedBuffer,
        holder: &ManagedAddress,
    ) -> SingleValueMapper<u64>;

    /// Emits when an asset record is created.
    #[event("drwaAssetRegistered")]
    fn drwa_asset_registered_event(
        &self,
        #[indexed] token_id: &ManagedBuffer,
        #[indexed] policy_id: &ManagedBuffer,
        #[indexed] regulated: bool,
    );

    /// Emits when holder compliance data is written.
    #[event("drwaHolderCompliance")]
    fn drwa_holder_compliance_event(
        &self,
        #[indexed] token_id: &ManagedBuffer,
        #[indexed] holder: &ManagedAddress,
        payload: &HolderComplianceEventPayload<Self::Api>,
    );

    /// Serializes holder compliance data in the binary field order consumed by
    /// the native mirror.
    fn serialize_holder(&self, holder: &DrwaHolderMirror<Self::Api>) -> ManagedBuffer {
        let mut result = ManagedBuffer::new();
        result.append_bytes(&holder.holder_policy_version.to_be_bytes());
        push_len_prefixed(&mut result, &holder.kyc_status);
        push_len_prefixed(&mut result, &holder.aml_status);
        push_len_prefixed(&mut result, &holder.investor_class);
        push_len_prefixed(&mut result, &holder.jurisdiction_code);
        result.append_bytes(&holder.expiry_round.to_be_bytes());
        result.append_bytes(&[holder.transfer_locked as u8]);
        result.append_bytes(&[holder.receive_locked as u8]);
        result.append_bytes(&[holder.auditor_authorized as u8]);
        result
    }

    /// Validates the token identifier format accepted by this contract.
    fn require_valid_token_id(&self, token_id: &ManagedBuffer) {
        require_valid_token_id(token_id);
    }

    /// Leaves storage unchanged during contract upgrades.
    #[upgrade]
    fn upgrade(&self) {}
}
