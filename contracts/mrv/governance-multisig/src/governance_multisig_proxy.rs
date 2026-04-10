multiversx_sc::proxy! {
    #[multiversx_sc::contract_abi("governance-multisig")]
    pub trait GovernanceMultisigProxy {
        #[init]
        fn init(&self, threshold: u32, initial_signers: MultiValueEncoded<ManagedAddress>);

        #[endpoint(addSigner)]
        fn add_signer(&self, signer: ManagedAddress);

        #[endpoint(proposeAction)]
        fn propose_action(
            &self,
            to: ManagedAddress,
            egld_amount: BigUint,
            endpoint_name: ManagedBuffer,
            arguments: MultiValueEncoded<ManagedBuffer>,
        );
    }
}