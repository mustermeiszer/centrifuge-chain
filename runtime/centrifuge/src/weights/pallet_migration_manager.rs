//! Autogenerated weights for pallet_migration_manager
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-02, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("centrifuge-dev"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=centrifuge-dev
// --steps=50
// --repeat=20
// --pallet=pallet_migration_manager
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/centrifuge/src/weights/pallet_migration_manager.rs
// --template=./scripts/runtime-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weights for pallet_migration_manager using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_migration_manager::WeightInfo for WeightInfo<T> {
	fn finalize() -> Weight {
		Weight::from_ref_time(36_122_000)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn migrate_system_account(n: u32) -> Weight {
		Weight::from_ref_time(39_407_000) // Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(1_901_000).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(n as u64)))
	}

	fn migrate_balances_issuance() -> Weight {
		Weight::from_ref_time(41_644_000)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn migrate_vesting_vesting(n: u32) -> Weight {
		Weight::from_ref_time(140_014_000) // Standard Error: 307_000
			.saturating_add(Weight::from_ref_time(56_508_000).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(n as u64)))
	}

	fn migrate_proxy_proxies(n: u32) -> Weight {
		Weight::from_ref_time(123_008_000) // Standard Error: 202_000
			.saturating_add(Weight::from_ref_time(11_813_000).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(n as u64)))
	}
}
