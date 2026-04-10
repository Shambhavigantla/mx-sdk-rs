multiversx_sc::proxy! {
    #[multiversx_sc::contract_abi("come-settlement")]
    pub trait ComeSettlementProxy {
        #[init]
        fn init(&self, governance: ManagedAddress);

        #[endpoint(createSettlement)]
        fn create_settlement(
            &self,
            settlement_id: ManagedBuffer,
            from: ManagedAddress,
            to: ManagedAddress,
            token_id: TokenIdentifier,
            amount_scaled: BigUint,
            reason_cid: ManagedBuffer,
        );
    }
}