multiversx_sc::proxy! {
    #[multiversx_sc::contract_abi("mrv-aggregator")]
    pub trait MrvAggregatorProxy {
        #[init]
        fn init(
            &self,
            quorum: u32,
            iot_window: u64,
            satellite_window: u64,
            govt_lab_window: u64,
            divergence_threshold_bps: u64,
        );

        #[endpoint(submitOracleReading)]
        fn submit_oracle_reading(
            &self,
            pai_id: ManagedBuffer,
            period_start: u64,
            period_end: u64,
            source: u8,
            data_cid: ManagedBuffer,
            source_timestamp: u64,
            device_did: ManagedAddress,
            device_signature: ManagedBuffer,
        );

        #[endpoint(trySeal)]
        fn try_seal(&self, pai_id: ManagedBuffer, period_end: u64) -> bool;

        #[endpoint(forceSealAfterTimeout)]
        fn force_seal_after_timeout(&self, pai_id: ManagedBuffer, period_end: u64) -> bool;

        #[endpoint(acknowledgeDiscrepancy)]
        fn acknowledge_discrepancy(&self, pai_id: ManagedBuffer, period_end: u64);

        #[endpoint(setQuorum)]
        fn set_quorum(&self, quorum: u32);

        #[endpoint(setCoherenceWindows)]
        fn set_coherence_windows(&self, iot_window: u64, satellite_window: u64, govt_lab_window: u64);

        #[endpoint(registerOracle)]
        fn register_oracle(&self, oracle: ManagedAddress);

        #[endpoint(deregisterOracle)]
        fn deregister_oracle(&self, oracle: ManagedAddress);

        #[endpoint(registerVerifier)]
        fn register_verifier(&self, verifier: ManagedAddress);

        #[endpoint(deregisterVerifier)]
        fn deregister_verifier(&self, verifier: ManagedAddress);

        #[endpoint(registerDevice)]
        fn register_device(&self, device_did: ManagedAddress, oracle: ManagedAddress);

        #[endpoint(deregisterDevice)]
        fn deregister_device(&self, device_did: ManagedAddress);

        #[endpoint(proposeOracleUpdate)]
        fn propose_oracle_update(&self, device_did: ManagedAddress, new_oracle: ManagedAddress);

        #[endpoint(acceptOracleUpdate)]
        fn accept_oracle_update(&self, device_did: ManagedAddress);
    }
}