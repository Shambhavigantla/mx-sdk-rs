// Custom proxy for MrvGovernance contract
// This file should be placed in: contracts/mrv/governance/src/mrv_governance_proxy.rs

use multiversx_sc::types::{
    BigUint, ManagedAddress, ManagedBuffer, MultiValueEncoded, NotPayable, ProxyArg, Tx, TxEnv,
    TxFrom, TxGas, TxProxyTrait, TxTo, TxTypedCall,
};
use multiversx_sc::proxy_imports::ManagedVec;
/// Proxy for the MrvGovernance smart contract
pub struct MrvGovernanceProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for MrvGovernanceProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = MrvGovernanceProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        MrvGovernanceProxyMethods { wrapped_tx: tx }
    }
}

/// Method container for the MrvGovernance contract proxy
pub struct MrvGovernanceProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

impl<Env, From, To, Gas> MrvGovernanceProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn init<
        Arg0: ProxyArg<ManagedVec<Env::Api, ManagedAddress<Env::Api>>>,
        Arg1: ProxyArg<u32>,
        Arg2: ProxyArg<u64>,
    >(
        self,
        initial_signers: Arg0,
        approval_threshold: Arg1,
        timelock_seconds: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("init")
            .argument(&initial_signers)
            .argument(&approval_threshold)
            .argument(&timelock_seconds)
            .original_result()
    }

    pub fn propose_emergency_pause<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<bool>,
    >(
        self,
        proposal_id: Arg0,
        pause: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("proposeEmergencyPause")
            .argument(&proposal_id)
            .argument(&pause)
            .original_result()
    }

    pub fn approve_proposal<Arg0: ProxyArg<ManagedBuffer<Env::Api>>>(
        self,
        proposal_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("approveProposal")
            .argument(&proposal_id)
            .original_result()
    }

    pub fn execute_proposal<Arg0: ProxyArg<ManagedBuffer<Env::Api>>>(
        self,
        proposal_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("executeProposal")
            .argument(&proposal_id)
            .original_result()
    }
}