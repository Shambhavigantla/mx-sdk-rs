#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod drwa_policy_registry_proxy;

use multiversx_sc::api::HandleConstraints;

use drwa_common::{
    DrwaCallerDomain, DrwaSyncEnvelope, DrwaSyncOperation, DrwaSyncOperationType, DrwaTokenPolicy,
    build_sync_hook_payload, invoke_drwa_sync_hook, require_valid_token_id,
    serialize_sync_envelope_payload,
};

const MAX_INVESTOR_CLASSES: usize = 100;
const MAX_JURISDICTIONS: usize = 200;

/// Manages per-token compliance policies (pause, auditor mode, investor-class
/// and jurisdiction allow-lists) and syncs policy state to the native DRWA
/// mirror on every mutation.
///
/// Governance is transferable via a propose-accept pattern with a time-limited
/// acceptance window.
#[multiversx_sc::contract]
pub trait DrwaPolicyRegistry: drwa_common::DrwaGovernanceModule {
    /// Initializes the contract with the governance address.
    #[init]
    fn init(&self, governance: ManagedAddress) {
        require!(!governance.is_zero(), "governance must not be zero");
        self.governance().set(&governance);
    }

    /// Creates or updates a token policy, increments its version, and syncs it
    /// to the native mirror.
    ///
    /// Access is limited to the governance address or the contract owner.
    /// Reverts if the token identifier is invalid or the input lists exceed the
    /// configured maximum sizes.
    #[endpoint(setTokenPolicy)]
    fn set_token_policy(
        &self,
        token_id: ManagedBuffer,
        drwa_enabled: bool,
        global_pause: bool,
        strict_auditor_mode: bool,
        metadata_protection_enabled: bool,
        allowed_investor_classes: ManagedVec<ManagedBuffer>,
        allowed_jurisdictions: ManagedVec<ManagedBuffer>,
    ) -> DrwaSyncEnvelope<Self::Api> {
        self.require_governance_or_owner();

        self.require_valid_token_id(&token_id);
        require!(
            allowed_investor_classes.len() <= MAX_INVESTOR_CLASSES,
            "too many investor classes: max 100"
        );
        require!(
            allowed_jurisdictions.len() <= MAX_JURISDICTIONS,
            "too many jurisdictions: max 200"
        );

        let next_version = self.token_policy_version(&token_id).get() + 1;

        let policy = DrwaTokenPolicy {
            drwa_enabled,
            global_pause,
            strict_auditor_mode,
            metadata_protection_enabled,
            token_policy_version: next_version,
            allowed_investor_classes,
            allowed_jurisdictions,
        };

        self.token_policy(&token_id).set(policy.clone());
        self.token_policy_version(&token_id).set(next_version);
        self.drwa_token_policy_event(
            &token_id,
            policy.drwa_enabled,
            policy.global_pause,
            policy.strict_auditor_mode,
            next_version,
        );

        let body = self.serialize_policy_json(&policy);
        let mut operations = ManagedVec::new();
        operations.push(DrwaSyncOperation {
            operation_type: DrwaSyncOperationType::TokenPolicy,
            token_id: token_id.clone(),
            holder: ManagedAddress::default(),
            version: next_version,
            body,
        });

        let caller_domain = DrwaCallerDomain::PolicyRegistry;
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

    /// Deactivates an existing token policy by setting `drwa_enabled = false`,
    /// incrementing its version, and syncing the update to the native mirror.
    ///
    /// Access is limited to the governance address or the contract owner.
    /// Reverts if the token policy does not exist.
    #[endpoint(deactivateTokenPolicy)]
    fn deactivate_token_policy(&self, token_id: ManagedBuffer) -> DrwaSyncEnvelope<Self::Api> {
        self.require_governance_or_owner();
        self.require_valid_token_id(&token_id);
        require!(
            !self.token_policy(&token_id).is_empty(),
            "token policy does not exist"
        );

        let mut policy = self.token_policy(&token_id).get();
        policy.drwa_enabled = false;

        let next_version = self.token_policy_version(&token_id).get() + 1;
        policy.token_policy_version = next_version;

        self.token_policy(&token_id).set(policy.clone());
        self.token_policy_version(&token_id).set(next_version);
        self.drwa_token_policy_event(
            &token_id,
            policy.drwa_enabled,
            policy.global_pause,
            policy.strict_auditor_mode,
            next_version,
        );

        let body = self.serialize_policy_json(&policy);
        let mut operations = ManagedVec::new();
        operations.push(DrwaSyncOperation {
            operation_type: DrwaSyncOperationType::TokenPolicy,
            token_id: token_id.clone(),
            holder: ManagedAddress::default(),
            version: next_version,
            body,
        });

        let caller_domain = DrwaCallerDomain::PolicyRegistry;
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

    /// Maps a token identifier to its full compliance policy.
    #[view(getTokenPolicy)]
    #[storage_mapper("tokenPolicy")]
    fn token_policy(
        &self,
        token_id: &ManagedBuffer,
    ) -> SingleValueMapper<DrwaTokenPolicy<Self::Api>>;

    /// Monotonically increasing version counter per token, used for staleness
    /// detection.
    #[view(getTokenPolicyVersion)]
    #[storage_mapper("tokenPolicyVersion")]
    fn token_policy_version(&self, token_id: &ManagedBuffer) -> SingleValueMapper<u64>;

    /// Emits when a token policy is created or updated.
    #[event("drwaTokenPolicy")]
    fn drwa_token_policy_event(
        &self,
        #[indexed] token_id: &ManagedBuffer,
        #[indexed] drwa_enabled: bool,
        #[indexed] global_pause: bool,
        #[indexed] strict_auditor_mode: bool,
        #[indexed] token_policy_version: u64,
    );

    /// Validates that a policy key is safe to embed in the hand-built JSON
    /// payload sent to the native enforcement decoder.
    ///
    /// Accepted bytes are limited to ASCII alphanumeric, `.`, `_`, and `-`.
    fn require_json_safe_key(&self, key: &ManagedBuffer) {
        require!(!key.is_empty(), "policy key must not be empty");
        let bytes = key.to_boxed_bytes();
        for &b in bytes.as_slice() {
            let is_ascii_alpha = b.is_ascii_alphabetic();
            let is_ascii_digit = b.is_ascii_digit();
            let is_safe_punct = b == b'.' || b == b'_' || b == b'-';
            require!(
                is_ascii_alpha || is_ascii_digit || is_safe_punct,
                "policy key contains unsupported character"
            );
        }
    }

    /// Serializes the JSON policy body expected by the native mirror.
    ///
    /// SECURITY: JSON is constructed by concatenation with validated keys.
    /// `require_json_safe_key` restricts keys to `[a-zA-Z0-9._-]` only.
    /// This approach is used because `no_std` environments lack serde_json.
    /// Do NOT extend the key character set without reviewing injection risk.
    ///
    /// The policy version is carried in `DrwaSyncOperation.version`, not in the
    /// JSON body.
    fn serialize_policy_json(&self, policy: &DrwaTokenPolicy<Self::Api>) -> ManagedBuffer {
        for class in policy.allowed_investor_classes.iter() {
            self.require_json_safe_key(&class);
        }
        for jur in policy.allowed_jurisdictions.iter() {
            self.require_json_safe_key(&jur);
        }

        let mut body = ManagedBuffer::new();
        body.append_bytes(b"{\"drwa_enabled\":");
        body.append_bytes(if policy.drwa_enabled {
            b"true"
        } else {
            b"false"
        });
        body.append_bytes(b",\"global_pause\":");
        body.append_bytes(if policy.global_pause {
            b"true"
        } else {
            b"false"
        });
        body.append_bytes(b",\"strict_auditor_mode\":");
        body.append_bytes(if policy.strict_auditor_mode {
            b"true"
        } else {
            b"false"
        });
        body.append_bytes(b",\"metadata_protection_enabled\":");
        body.append_bytes(if policy.metadata_protection_enabled {
            b"true"
        } else {
            b"false"
        });
        if !policy.allowed_investor_classes.is_empty() {
            body.append_bytes(b",\"allowed_investor_classes\":{");
            let mut first = true;
            for class in policy.allowed_investor_classes.iter() {
                if !first {
                    body.append_bytes(b",");
                }
                body.append_bytes(b"\"");
                body.append(&class);
                body.append_bytes(b"\":true");
                first = false;
            }
            body.append_bytes(b"}");
        }
        if !policy.allowed_jurisdictions.is_empty() {
            body.append_bytes(b",\"allowed_jurisdictions\":{");
            let mut first = true;
            for jur in policy.allowed_jurisdictions.iter() {
                if !first {
                    body.append_bytes(b",");
                }
                body.append_bytes(b"\"");
                body.append(&jur);
                body.append_bytes(b"\":true");
                first = false;
            }
            body.append_bytes(b"}");
        }
        body.append_bytes(b"}");
        body
    }

    /// Validates the token identifier format accepted by this contract.
    fn require_valid_token_id(&self, token_id: &ManagedBuffer) {
        require_valid_token_id(token_id);
    }

    /// Leaves storage unchanged during contract upgrades.
    #[upgrade]
    fn upgrade(&self) {}
}
