// Copyright 2021 Centrifuge GmbH (centrifuge.io).
// This file is part of Centrifuge chain project.

// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).

// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

//! Crowdloan reward pallet testing environment and utilities
//!
//! The main components implemented in this module is a mock runtime
//! and some helper functions.

use crate as pallet_migration_manager;

use codec::{Decode, Encode};
use frame_support::sp_runtime::traits::ConvertInto;
use frame_support::{
	parameter_types,
	traits::{InstanceFilter, MaxEncodedLen},
	weights::Weight,
};
use sp_core::{RuntimeDebug, H256};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	AccountId32, Perbill,
};

pub type AccountId = AccountId32;
pub type Balance = u128;
pub type Index = u32;
pub type BlockNumber = u32;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<MockRuntime>;
type Block = frame_system::mocking::MockBlock<MockRuntime>;

// Build mock runtime
frame_support::construct_runtime!(
	pub enum MockRuntime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
		Vesting: pallet_vesting::{Pallet, Call, Storage, Event<T>},
		Proxy: pallet_proxy::{Pallet, Call, Storage, Event<T>},
		Migration: pallet_migration_manager::{Pallet, Call, Config, Storage, Event<T>},
	}
);

parameter_types! {
	// One storage item; value is size 4+4+16+32 bytes = 56 bytes.
	pub const ProxyDepositBase: Balance = 30;
	// Additional storage item size of 32 bytes.
	pub const ProxyDepositFactor: Balance = 5;
	pub const MaxProxies: u16 = 32;
	pub const AnnouncementDepositBase: Balance = 8;
	pub const AnnouncementDepositFactor: Balance = 1;
	pub const MaxPending: u16 = 32;
}

/// The type used to represent the kinds of proxying allowed.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, RuntimeDebug)]
pub enum ProxyType {
	Any,
	NonTransfer,
	Governance,
	_Staking,
	NonProxy,
}
impl MaxEncodedLen for ProxyType {
	fn max_encoded_len() -> usize {
		8
	}
}
impl Default for ProxyType {
	fn default() -> Self {
		Self::Any
	}
}
impl InstanceFilter<Call> for ProxyType {
	fn filter(&self, _c: &Call) -> bool {
		true
	}

	fn is_superset(&self, _o: &Self) -> bool {
		false
	}
}

impl pallet_proxy::Config for MockRuntime {
	type Event = Event;
	type Call = Call;
	type Currency = Balances;
	type ProxyType = ProxyType;
	type ProxyDepositBase = ProxyDepositBase;
	type ProxyDepositFactor = ProxyDepositFactor;
	type MaxProxies = MaxProxies;
	type WeightInfo = pallet_proxy::weights::SubstrateWeight<Self>;
	type MaxPending = MaxPending;
	type CallHasher = BlakeTwo256;
	type AnnouncementDepositBase = AnnouncementDepositBase;
	type AnnouncementDepositFactor = AnnouncementDepositFactor;
}

// Parameterize balances pallet
parameter_types! {
	pub const MaxLocks: u32 = 10;
	pub const ExistentialDeposit: u64 = 1;
}

// Implement balances pallet configuration for mock runtime
impl pallet_balances::Config for MockRuntime {
	type MaxLocks = ();
	type Balance = Balance;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxReserves = ();
	type ReserveIdentifier = ();
}

// Parameterize vesting pallet
parameter_types! {
	pub const MinVestedTransfer: u64 = 16;
}

// Implement vesting pallet configuration for mock runtime
impl pallet_vesting::Config for MockRuntime {
	type Event = Event;
	type Currency = Balances;
	type BlockNumberToBalance = ConvertInto;
	type MinVestedTransfer = MinVestedTransfer;
	type WeightInfo = ();
}

// Parameterize frame system pallet
parameter_types! {
	pub const BlockHashCount: BlockNumber = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

// Implement frame system pallet configuration for mock runtime
impl frame_system::Config for MockRuntime {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type Origin = Origin;
	type Index = Index;
	type Call = Call;
	type BlockNumber = BlockNumber;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = sp_runtime::generic::Header<BlockNumber, BlakeTwo256>;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
}
pub const ACCOUNTS: u64 = 100;
pub const VESTINGS: u64 = 10;
pub const PROXIES: u64 = 10;

parameter_types! {
	pub const MigrationMaxAccounts: u64 = ACCOUNTS;
	pub const MigrationMaxVestings: u64 = VESTINGS;
	pub const MigrationMaxProxies: u64 = PROXIES;
}

// Implement the migration manager pallet
// The actual associated type, which executes the migration can be found in the migration folder
impl pallet_migration_manager::Config for MockRuntime {
	type MigrationMaxAccounts = MigrationMaxAccounts;
	type MigrationMaxVestings = MigrationMaxVestings;
	type MigrationMaxProxies = MigrationMaxProxies;
	type Event = Event;
	type WeightInfo = ();
}

// ----------------------------------------------------------------------------
// Test externalities
// ----------------------------------------------------------------------------

pub fn get_test_account_one() -> (Vec<u8>, Vec<u8>) {
	let key: [u8; 80] = [
		38, 170, 57, 78, 234, 86, 48, 224, 124, 72, 174, 12, 149, 88, 206, 247, 185, 157, 136, 14,
		198, 129, 121, 156, 12, 243, 14, 136, 134, 55, 29, 169, 0, 16, 127, 162, 131, 58, 130, 79,
		133, 13, 114, 26, 227, 47, 96, 49, 240, 147, 42, 183, 214, 6, 46, 107, 98, 233, 70, 25, 35,
		45, 103, 16, 73, 83, 205, 222, 41, 37, 61, 27, 96, 153, 146, 103, 165, 62, 205, 46,
	];
	let key: Vec<u8> = key.iter().cloned().collect();
	let value: [u8; 80] = [
		0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 76, 115, 59, 154, 201, 101, 4, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	];
	let value: Vec<u8> = value.iter().cloned().collect();

	(key, value)
}

pub fn get_test_account_two() -> (Vec<u8>, Vec<u8>) {
	let key: [u8; 80] = [
		38, 170, 57, 78, 234, 86, 48, 224, 124, 72, 174, 12, 149, 88, 206, 247, 185, 157, 136, 14,
		198, 129, 121, 156, 12, 243, 14, 136, 134, 55, 29, 169, 0, 17, 1, 21, 88, 185, 78, 148, 82,
		206, 45, 245, 216, 48, 198, 106, 2, 204, 8, 205, 35, 58, 40, 206, 222, 103, 45, 236, 236,
		191, 197, 38, 167, 199, 191, 24, 117, 120, 28, 227, 13, 184, 223, 7, 81, 39, 11, 99,
	];
	let key = key.iter().cloned().collect();
	let value: [u8; 80] = [
		0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 112, 165, 154, 127, 7, 87, 177, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
	];
	let value = value.iter().cloned().collect();

	(key, value)
}

// Test externalities builder type declaraction.
//
// This type is mainly used for mocking storage in tests. It is the type alias
// for an in-memory, hashmap-based externalities implementation.
pub struct TestExternalitiesBuilder {
	_existential_deposit: u64,
}

// Implement default trait for test externalities builder
impl Default for TestExternalitiesBuilder {
	fn default() -> Self {
		Self {
			_existential_deposit: 1,
		}
	}
}

// Implement test externalities builder
impl TestExternalitiesBuilder {
	pub fn existential_deposit(mut self, existential_deposit: u64) -> Self {
		self._existential_deposit = existential_deposit;
		self
	}

	// Build a genesis storage key/value store
	pub fn build<R>(self, execute: impl FnOnce() -> R) -> sp_io::TestExternalities {
		let storage = frame_system::GenesisConfig::default()
			.build_storage::<MockRuntime>()
			.unwrap();

		/*pallet_balances::GenesisConfig::<MockRuntime> { balances: vec![] }
			.assimilate_storage(&mut storage)
			.unwrap();

		pallet_vesting::GenesisConfig::<MockRuntime> { vesting: vec![] }
			.assimilate_storage(&mut storage)
			.unwrap();*/

		let mut ext = sp_io::TestExternalities::new(storage);
		ext.execute_with(|| {
			System::set_block_number(1);
		});
		ext.execute_with(execute);
		ext
	}
}

pub(crate) fn reward_events() -> Vec<pallet_migration_manager::Event<MockRuntime>> {
	System::events()
		.into_iter()
		.map(|r| r.event)
		.filter_map(|e| {
			if let Event::Migration(inner) = e {
				Some(inner)
			} else {
				None
			}
		})
		.collect()
}
