multiversx_sc::proxy! {
    #[multiversx_sc::contract_abi("mrv-registry")]
    pub trait MrvRegistryProxy {
        #[init]
        fn init(&self, governance: ManagedAddress);

        #[endpoint(registerMethodology)]
        fn register_methodology(
            &self,
            methodology_id: ManagedBuffer,
            version_label: ManagedBuffer,
            pack_digest: ManagedBuffer,
            approval_status: ManagedBuffer,
            effective_from: u64,
            effective_to: u64,
        );
    }
}