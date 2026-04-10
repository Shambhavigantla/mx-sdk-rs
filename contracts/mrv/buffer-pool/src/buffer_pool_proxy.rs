multiversx_sc::proxy! {
    #[multiversx_sc::contract_abi("buffer-pool")]
    pub trait BufferPoolProxy {
        #[init]
        fn init(&self, governance: ManagedAddress, carbon_credit_addr: ManagedAddress);

        #[endpoint(setCarbonCreditAddr)]
        fn set_carbon_credit_addr(&self, addr: ManagedAddress);

        #[endpoint(depositBufferCredits)]
        fn deposit_buffer_credits(&self, project_id: ManagedBuffer, amount_scaled: BigUint, monitoring_period_n: u64);

        #[endpoint(cancelBufferCredits)]
        fn cancel_buffer_credits(&self, project_id: ManagedBuffer, amount_scaled: BigUint);

        #[endpoint(replenishBufferCredits)]
        fn replenish_buffer_credits(&self, project_id: ManagedBuffer, amount_scaled: BigUint);
    }
}