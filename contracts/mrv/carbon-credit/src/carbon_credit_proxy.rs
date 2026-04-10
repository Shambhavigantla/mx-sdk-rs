multiversx_sc::proxy! {
    #[multiversx_sc::contract_abi("carbon-credit")]
    pub trait CarbonCreditModuleProxy {
        #[init]
        fn init(&self, governance: ManagedAddress, buffer_pool_addr: ManagedAddress);

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
        );
    }
}