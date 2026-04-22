use drwa_common::{
    DrwaCallerDomain, DrwaSyncEnvelope, DrwaSyncOperation, DrwaSyncOperationType,
};
use multiversx_sc::{
    codec::top_encode_to_vec_u8_or_panic,
    types::{ManagedAddress, ManagedBuffer, ManagedVec},
};
use multiversx_sc_scenario::imports::StaticApi;

#[test]
fn drwa_sync_envelope_top_encode_smoke() {
    let mut operations = ManagedVec::<StaticApi, DrwaSyncOperation<StaticApi>>::new();
    operations.push(DrwaSyncOperation {
        operation_type: DrwaSyncOperationType::TokenPolicy,
        token_id: ManagedBuffer::from(b"CARBON-ab12cd"),
        holder: ManagedAddress::zero(),
        version: 7,
        body: ManagedBuffer::from(
            br#"{"drwa_enabled":true,"global_pause":false,"strict_auditor_mode":true}"#,
        ),
    });

    let envelope = DrwaSyncEnvelope::<StaticApi> {
        caller_domain: DrwaCallerDomain::PolicyRegistry,
        payload_hash: ManagedBuffer::from(&[9u8; 32]),
        operations,
    };

    let encoded = top_encode_to_vec_u8_or_panic(&envelope);
    assert!(!encoded.is_empty(), "encoded envelope must not be empty");
}
