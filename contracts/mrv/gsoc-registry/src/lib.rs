#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod gsoc_registry_proxy;

use mrv_common::MrvGovernanceModule;

/// Serial record stored after successful batch registration.
#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, Clone, PartialEq, Eq)]
pub struct GsocSerialBatchRecord<M: ManagedTypeApi> {
    pub project_id: ManagedBuffer<M>,
    pub vintage_year: u32,
    pub serial_start: ManagedBuffer<M>,
    pub serial_end: ManagedBuffer<M>,
    pub quantity: u64,
    pub registered_at: u64,
    pub retired: bool,
}

/// Retirement record for a specific serial range.
#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem, Clone, PartialEq, Eq)]
pub struct GsocRetirementRecord<M: ManagedTypeApi> {
    pub serial: ManagedBuffer<M>,
    pub beneficiary_name: ManagedBuffer<M>,
    pub beneficiary_address: ManagedAddress<M>,
    pub retired_at: u64,
    pub burn_tx_hash: ManagedBuffer<M>,
}

/// GSOC ITMO serial registry contract.
///
/// Manages the full lifecycle of ITMO serials: reservation, batch
/// registration, and retirement with beneficiary tracking.
#[multiversx_sc::contract]
pub trait GsocRegistry: mrv_common::MrvGovernanceModule {
    /// Initializes the registry with a governance address and zeroed supply counters.
    #[init]
    fn init(&self, governance: ManagedAddress) {
        require!(!governance.is_zero(), "governance must not be zero");
        self.governance().set(governance);
        self.total_supply().set(0u64);
        self.total_retired().set(0u64);
    }

    /// Reserves an ITMO serial before off-chain persistence and final registration.
    #[endpoint(reserveSerial)]
    fn reserve_serial(&self, itmo_serial: ManagedBuffer) -> bool {
        self.require_governance_or_owner();
        require!(!itmo_serial.is_empty(), "empty itmo_serial");

        if self.reserved_serials().contains(&itmo_serial) {
            return false;
        }
        if self.serial_records().contains_key(&itmo_serial) {
            return false;
        }

        self.reserved_serials().insert(itmo_serial.clone());
        self.serial_reserved_event(&itmo_serial);
        true
    }

    /// Finalizes registration for a serial that has already been reserved.
    #[endpoint(registerSerialBatch)]
    fn register_serial_batch(
        &self,
        itmo_serial: ManagedBuffer,
        project_id: ManagedBuffer,
        vintage_year: u32,
        serial_start: ManagedBuffer,
        serial_end: ManagedBuffer,
        quantity: u64,
    ) {
        self.require_governance_or_owner();
        require!(!itmo_serial.is_empty(), "empty itmo_serial");
        require!(!project_id.is_empty(), "empty project_id");
        require!(vintage_year >= 2020 && vintage_year <= 2100, "vintage_year out of range");
        require!(quantity > 0, "quantity must be positive");

        require!(
            self.reserved_serials().contains(&itmo_serial),
            "SERIAL_NOT_RESERVED: call reserveSerial() first"
        );

        require!(
            !self.serial_records().contains_key(&itmo_serial),
            "DUPLICATE_SERIAL: serial already registered"
        );

        let record = GsocSerialBatchRecord {
            project_id: project_id.clone(),
            vintage_year,
            serial_start,
            serial_end,
            quantity,
            registered_at: self.blockchain().get_block_timestamp(),
            retired: false,
        };

        self.serial_records().insert(itmo_serial.clone(), record);
        self.total_supply().update(|s| *s += quantity);
        self.project_serial_count(&project_id).update(|c| *c += 1u64);

        self.reserved_serials().swap_remove(&itmo_serial);

        self.serial_batch_registered_event(&itmo_serial, &project_id, quantity);
    }

    /// Cancels a reserved serial that has not been finalized.
    #[endpoint(cancelReservation)]
    fn cancel_reservation(&self, itmo_serial: ManagedBuffer) {
        self.require_governance_or_owner();
        require!(!itmo_serial.is_empty(), "empty itmo_serial");
        require!(
            self.reserved_serials().contains(&itmo_serial),
            "SERIAL_NOT_RESERVED: cannot cancel a non-reserved serial"
        );
        require!(
            !self.serial_records().contains_key(&itmo_serial),
            "SERIAL_ALREADY_REGISTERED: cannot cancel a registered serial"
        );
        self.reserved_serials().swap_remove(&itmo_serial);
        self.serial_reservation_cancelled_event(&itmo_serial);
    }

    /// Marks a registered serial as retired and records beneficiary details.
    #[endpoint(recordRetirement)]
    fn record_retirement(
        &self,
        itmo_serial: ManagedBuffer,
        beneficiary_name: ManagedBuffer,
        beneficiary_address: ManagedAddress,
        burn_tx_hash: ManagedBuffer,
    ) {
        self.require_governance_or_owner();
        require!(!itmo_serial.is_empty(), "empty itmo_serial");
        require!(
            self.serial_records().contains_key(&itmo_serial),
            "serial not registered"
        );

        let record = self.serial_records().get(&itmo_serial).unwrap();
        require!(!record.retired, "SERIAL_ALREADY_RETIRED");

        self.serial_records().entry(itmo_serial.clone()).and_modify(|r| {
            r.retired = true;
        });

        let retirement = GsocRetirementRecord {
            serial: itmo_serial.clone(),
            beneficiary_name: beneficiary_name.clone(),
            beneficiary_address: beneficiary_address.clone(),
            retired_at: self.blockchain().get_block_timestamp(),
            burn_tx_hash,
        };

        self.retirement_records().insert(itmo_serial.clone(), retirement);
        self.total_retired().update(|r| *r += record.quantity);

        self.serial_retired_event(&itmo_serial, &beneficiary_name);
    }

    #[view(getSerialRecord)]
    fn get_serial_record(
        &self,
        itmo_serial: ManagedBuffer,
    ) -> OptionalValue<GsocSerialBatchRecord<Self::Api>> {
        match self.serial_records().get(&itmo_serial) {
            Some(r) => OptionalValue::Some(r),
            None => OptionalValue::None,
        }
    }

    #[view(getTotalSupply)]
    fn get_total_supply(&self) -> u64 {
        self.total_supply().get()
    }

    #[view(getTotalRetired)]
    fn get_total_retired(&self) -> u64 {
        self.total_retired().get()
    }

    #[view(isSerialRetired)]
    fn is_serial_retired(&self, itmo_serial: ManagedBuffer) -> bool {
        match self.serial_records().get(&itmo_serial) {
            Some(r) => r.retired,
            None => false,
        }
    }

    #[view(getProjectSerials)]
    fn get_project_serials(&self, project_id: ManagedBuffer) -> u64 {
        if self.project_serial_count(&project_id).is_empty() {
            0
        } else {
            self.project_serial_count(&project_id).get()
        }
    }

    #[view(isSerialReserved)]
    fn is_serial_reserved(&self, itmo_serial: ManagedBuffer) -> bool {
        self.reserved_serials().contains(&itmo_serial)
    }

    #[storage_mapper("serialRecords")]
    fn serial_records(&self) -> MapMapper<ManagedBuffer, GsocSerialBatchRecord<Self::Api>>;

    #[storage_mapper("retirementRecords")]
    fn retirement_records(&self) -> MapMapper<ManagedBuffer, GsocRetirementRecord<Self::Api>>;

    #[storage_mapper("reservedSerials")]
    fn reserved_serials(&self) -> UnorderedSetMapper<ManagedBuffer>;

    #[storage_mapper("totalSupply")]
    fn total_supply(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("totalRetired")]
    fn total_retired(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("projectSerialCount")]
    fn project_serial_count(&self, project_id: &ManagedBuffer) -> SingleValueMapper<u64>;

    #[event("serialReserved")]
    fn serial_reserved_event(&self, #[indexed] itmo_serial: &ManagedBuffer);

    #[event("serialReservationCancelled")]
    fn serial_reservation_cancelled_event(&self, #[indexed] itmo_serial: &ManagedBuffer);

    #[event("serialBatchRegistered")]
    fn serial_batch_registered_event(
        &self,
        #[indexed] itmo_serial: &ManagedBuffer,
        #[indexed] project_id: &ManagedBuffer,
        quantity: u64,
    );

    #[event("serialRetired")]
    fn serial_retired_event(
        &self,
        #[indexed] itmo_serial: &ManagedBuffer,
        #[indexed] beneficiary_name: &ManagedBuffer,
    );

    #[upgrade]
    fn upgrade(&self) {}
}
