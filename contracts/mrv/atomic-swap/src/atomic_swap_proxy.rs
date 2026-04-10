multiversx_sc::proxy! {
    #[multiversx_sc::contract_abi("atomic-swap")]
    pub trait AtomicSwapProxy {
        #[init]
        fn init(&self, come_token_id: TokenIdentifier);

        #[endpoint(createRfq)]
        fn create_rfq(
            &self,
            rfq_id: ManagedBuffer,
            buyer: ManagedAddress,
            dealer: ManagedAddress,
            token_id: TokenIdentifier,
            quantity: BigUint,
            margin_amount: BigUint,
            price_come_per_unit: BigUint,
            expiry_epoch: u64,
        );

        #[endpoint(depositMargin)]
        fn deposit_margin(&self, rfq_id: ManagedBuffer);

        #[endpoint(settle)]
        fn settle(&self, rfq_id: ManagedBuffer);

        #[endpoint(autoReclaim)]
        fn auto_reclaim(&self, rfq_id: ManagedBuffer);

        #[endpoint(cancelRfq)]
        fn cancel_rfq(&self, rfq_id: ManagedBuffer);
    }
}