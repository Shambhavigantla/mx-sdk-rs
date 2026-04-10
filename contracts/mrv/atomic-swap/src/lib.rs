#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod atomic_swap_proxy;

/// Escrowed RFQ lifecycle record tracking margin deposit, settlement, and expiry.
#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, Clone, PartialEq, Eq)]
pub struct EscrowedRfqRecord<M: ManagedTypeApi> {
    pub rfq_id: ManagedBuffer<M>,
    pub buyer: ManagedAddress<M>,
    pub dealer: ManagedAddress<M>,
    pub token_id: TokenIdentifier<M>,
    pub quantity: BigUint<M>,
    pub margin_amount: BigUint<M>,
    pub price_come_per_unit: BigUint<M>,
    pub funded_epoch: u64,
    pub expiry_epoch: u64,
    pub status: u8,
}

/// RFQ status code for records awaiting the buyer's margin deposit.
const RFQ_PENDING_DEPOSIT: u8 = 0;
/// RFQ status code for records with margin deposited in escrow.
const RFQ_DEPOSITED: u8 = 1;
/// RFQ status code for settled records. Status code `2` is intentionally unused.
const RFQ_COMPLETED: u8 = 3;
/// RFQ status code for records reclaimed after expiry.
const RFQ_EXPIRED: u8 = 4;
/// Status used when a deposited RFQ is cancelled before expiry.
const RFQ_CANCELLED: u8 = 5;

/// Escrow-based RFQ settlement contract for COME-funded atomic swaps.
///
/// Buyer deposits COME margin, dealer delivers RWA tokens, and margin is
/// released on settlement or returned to buyer on expiry or cancellation.
#[multiversx_sc::contract]
pub trait AtomicSwap {
    /// Initializes the contract with the accepted COME token identifier.
    #[init]
    fn init(&self, come_token_id: TokenIdentifier) {
        require!(come_token_id.is_valid_esdt_identifier(), "invalid COME token ID");
        self.come_token_id().set(come_token_id);
    }

    /// Create an RFQ record after buyer and dealer agree on terms.
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
    ) {
        let caller = self.blockchain().get_caller();
        require!(
            caller == self.blockchain().get_owner_address(),
            "only owner can create RFQs"
        );
        require!(!rfq_id.is_empty(), "empty rfq_id");
        require!(!buyer.is_zero(), "buyer must not be zero");
        require!(!dealer.is_zero(), "dealer must not be zero");
        require!(token_id.is_valid_esdt_identifier(), "invalid token_id");
        require!(quantity > 0u64, "quantity must be positive");
        require!(margin_amount > 0u64, "margin must be positive");
        require!(price_come_per_unit > 0u64, "price must be positive");
        require!(
            expiry_epoch > self.blockchain().get_block_epoch(),
            "expiry must be in the future"
        );
        require!(
            !self.rfqs().contains_key(&rfq_id),
            "RFQ already exists"
        );

        let record = EscrowedRfqRecord {
            rfq_id: rfq_id.clone(),
            buyer,
            dealer,
            token_id,
            quantity,
            margin_amount,
            price_come_per_unit,
            funded_epoch: 0u64,
            expiry_epoch,
            status: RFQ_PENDING_DEPOSIT,
        };

        self.rfqs().insert(rfq_id.clone(), record);
        self.rfq_created_event(&rfq_id);
    }

    /// Buyer deposits margin (COME) to contract escrow.
    #[payable("*")]
    #[endpoint(depositMargin)]
    fn deposit_margin(&self, rfq_id: ManagedBuffer) {
        let rfq = self.rfqs().get(&rfq_id);
        require!(rfq.is_some(), "RFQ not found");
        let rfq = rfq.unwrap();
        require!(rfq.status == RFQ_PENDING_DEPOSIT, "NOT_AWAITING_DEPOSIT");

        let caller = self.blockchain().get_caller();
        require!(caller == rfq.buyer, "only buyer can deposit margin");

        let payment = self.call_value().single_esdt();
        require!(
            payment.token_identifier == self.come_token_id().get(),
            "must deposit COME token"
        );
        require!(payment.amount == rfq.margin_amount, "WRONG_MARGIN_AMOUNT");

        self.rfqs().entry(rfq_id.clone()).and_modify(|r| {
            r.status = RFQ_DEPOSITED;
            r.funded_epoch = self.blockchain().get_block_epoch();
        });

        self.locked_balances(&rfq.buyer).update(|b| *b += &payment.amount);
        self.margin_deposited_event(&rfq_id, &payment.amount);
    }

    /// Dealer settles by transferring the quoted RWA tokens to the buyer and
    /// releasing the escrowed margin to the dealer.
    #[payable("*")]
    #[endpoint(settle)]
    fn settle(&self, rfq_id: ManagedBuffer) {
        let rfq = self.rfqs().get(&rfq_id);
        require!(rfq.is_some(), "RFQ not found");
        let rfq = rfq.unwrap();
        require!(rfq.status == RFQ_DEPOSITED, "NOT_DEPOSITED");
        require!(
            self.blockchain().get_block_epoch() <= rfq.expiry_epoch,
            "EXPIRED"
        );

        let caller = self.blockchain().get_caller();
        require!(caller == rfq.dealer, "ONLY_DEALER");

        let payment = self.call_value().single_esdt();
        require!(payment.token_identifier == rfq.token_id, "wrong RWA token");
        require!(payment.amount == rfq.quantity, "wrong RWA quantity");

        self.send().direct_esdt(&rfq.buyer, &rfq.token_id, 0u64, &rfq.quantity);

        self.send().direct_esdt(&rfq.dealer, &self.come_token_id().get(), 0u64, &rfq.margin_amount);

        let buyer_locked = self.locked_balances(&rfq.buyer).get();
        require!(buyer_locked >= rfq.margin_amount, "LOCKED_BALANCE_UNDERFLOW: accounting discrepancy");
        self.locked_balances(&rfq.buyer).set(&buyer_locked - &rfq.margin_amount);

        self.rfqs().entry(rfq_id.clone()).and_modify(|r| {
            r.status = RFQ_COMPLETED;
        });

        self.rfq_settled_event(&rfq_id, &rfq.buyer, &rfq.dealer);
    }

    /// Returns the escrowed margin to the buyer after expiry.
    /// Any caller may trigger the reclaim, but the funds always go to the buyer.
    #[endpoint(autoReclaim)]
    fn auto_reclaim(&self, rfq_id: ManagedBuffer) {
        let rfq = self.rfqs().get(&rfq_id);
        require!(rfq.is_some(), "RFQ not found");
        let rfq = rfq.unwrap();
        require!(rfq.status == RFQ_DEPOSITED, "NOT_DEPOSITED");
        require!(
            self.blockchain().get_block_epoch() > rfq.expiry_epoch,
            "NOT_EXPIRED"
        );

        self.send().direct_esdt(&rfq.buyer, &self.come_token_id().get(), 0u64, &rfq.margin_amount);

        let buyer_locked = self.locked_balances(&rfq.buyer).get();
        require!(buyer_locked >= rfq.margin_amount, "LOCKED_BALANCE_UNDERFLOW: accounting discrepancy");
        self.locked_balances(&rfq.buyer).set(&buyer_locked - &rfq.margin_amount);

        self.rfqs().entry(rfq_id.clone()).and_modify(|r| {
            r.status = RFQ_EXPIRED;
        });

        self.margin_returned_event(&rfq_id, &rfq.margin_amount, &rfq.buyer);
    }

    /// Cancels a deposited RFQ before expiry and returns the margin to the buyer.
    /// Either counterparty may cancel.
    #[endpoint(cancelRfq)]
    fn cancel_rfq(&self, rfq_id: ManagedBuffer) {
        let rfq = self.rfqs().get(&rfq_id);
        require!(rfq.is_some(), "RFQ not found");
        let rfq = rfq.unwrap();
        require!(rfq.status == RFQ_DEPOSITED, "NOT_DEPOSITED");

        let caller = self.blockchain().get_caller();
        require!(
            caller == rfq.buyer || caller == rfq.dealer,
            "only buyer or dealer can cancel"
        );

        self.send().direct_esdt(&rfq.buyer, &self.come_token_id().get(), 0u64, &rfq.margin_amount);

        let buyer_locked = self.locked_balances(&rfq.buyer).get();
        require!(buyer_locked >= rfq.margin_amount, "LOCKED_BALANCE_UNDERFLOW: accounting discrepancy");
        self.locked_balances(&rfq.buyer).set(&buyer_locked - &rfq.margin_amount);

        self.rfqs().entry(rfq_id.clone()).and_modify(|r| {
            r.status = RFQ_CANCELLED;
        });

        self.margin_returned_event(&rfq_id, &rfq.margin_amount, &rfq.buyer);
    }

    #[view(getRfq)]
    fn get_rfq(&self, rfq_id: ManagedBuffer) -> OptionalValue<EscrowedRfqRecord<Self::Api>> {
        match self.rfqs().get(&rfq_id) {
            Some(r) => OptionalValue::Some(r),
            None => OptionalValue::None,
        }
    }

    #[view(getLockedBalance)]
    fn get_locked_balance(&self, holder: ManagedAddress) -> BigUint {
        self.locked_balances(&holder).get()
    }

    #[storage_mapper("comeTokenId")]
    fn come_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("rfqs")]
    fn rfqs(&self) -> MapMapper<ManagedBuffer, EscrowedRfqRecord<Self::Api>>;

    #[storage_mapper("lockedBalances")]
    fn locked_balances(&self, holder: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[event("rfqCreated")]
    fn rfq_created_event(&self, #[indexed] rfq_id: &ManagedBuffer);

    #[event("marginDeposited")]
    fn margin_deposited_event(
        &self,
        #[indexed] rfq_id: &ManagedBuffer,
        amount: &BigUint,
    );

    #[event("rfqSettled")]
    fn rfq_settled_event(
        &self,
        #[indexed] rfq_id: &ManagedBuffer,
        #[indexed] buyer: &ManagedAddress,
        #[indexed] dealer: &ManagedAddress,
    );

    #[event("marginReturned")]
    fn margin_returned_event(
        &self,
        #[indexed] rfq_id: &ManagedBuffer,
        #[indexed] amount: &BigUint,
        #[indexed] recipient: &ManagedAddress,
    );

    #[upgrade]
    fn upgrade(&self) {}
}
