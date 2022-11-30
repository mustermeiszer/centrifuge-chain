//! Autogenerated weights for pallet_uniques
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-02, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("altair-dev"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=altair-dev
// --steps=50
// --repeat=20
// --pallet=pallet_uniques
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/altair/src/weights/pallet_uniques.rs
// --template=./scripts/runtime-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weights for pallet_uniques using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_uniques::weights::WeightInfo for WeightInfo<T> {
	fn create() -> Weight {
		Weight::from_ref_time(63_680_000)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}

	fn force_create() -> Weight {
		Weight::from_ref_time(38_598_000)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}

	fn destroy(n: u32, m: u32, a: u32) -> Weight {
		Weight::from_ref_time(0) // Standard Error: 46_000
			.saturating_add(Weight::from_ref_time(27_176_000).saturating_mul(n as u64)) // Standard Error: 46_000
			.saturating_add(Weight::from_ref_time(4_340_000).saturating_mul(m as u64)) // Standard Error: 46_000
			.saturating_add(Weight::from_ref_time(4_593_000).saturating_mul(a as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(m as u64)))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(a as u64)))
	}

	fn mint() -> Weight {
		Weight::from_ref_time(89_871_000)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}

	fn burn() -> Weight {
		Weight::from_ref_time(81_614_000)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}

	fn transfer() -> Weight {
		Weight::from_ref_time(62_067_000)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}

	fn redeposit(i: u32) -> Weight {
		Weight::from_ref_time(0) // Standard Error: 39_000
			.saturating_add(Weight::from_ref_time(33_380_000).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(i as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(i as u64)))
	}

	fn freeze() -> Weight {
		Weight::from_ref_time(48_191_000)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn thaw() -> Weight {
		Weight::from_ref_time(48_725_000)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn freeze_collection() -> Weight {
		Weight::from_ref_time(37_364_000)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn thaw_collection() -> Weight {
		Weight::from_ref_time(37_496_000)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn transfer_ownership() -> Weight {
		Weight::from_ref_time(55_480_000)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}

	fn set_team() -> Weight {
		Weight::from_ref_time(39_546_000)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn force_item_status() -> Weight {
		Weight::from_ref_time(44_502_000)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}

	fn set_attribute() -> Weight {
		Weight::from_ref_time(99_252_000)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}

	fn clear_attribute() -> Weight {
		Weight::from_ref_time(97_589_000)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}

	fn set_metadata() -> Weight {
		Weight::from_ref_time(75_636_000)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}

	fn clear_metadata() -> Weight {
		Weight::from_ref_time(75_380_000)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}

	fn set_collection_metadata() -> Weight {
		Weight::from_ref_time(73_642_000)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}

	fn clear_collection_metadata() -> Weight {
		Weight::from_ref_time(69_511_000)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn approve_transfer() -> Weight {
		Weight::from_ref_time(50_664_000)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn cancel_approval() -> Weight {
		Weight::from_ref_time(50_620_000)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn set_accept_ownership() -> Weight {
		Weight::from_ref_time(46_620_000)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn set_collection_max_supply() -> Weight {
		Weight::from_ref_time(44_219_000)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn buy_item() -> Weight {
		Weight::from_ref_time(44_219_000)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn set_price() -> Weight {
		Weight::from_ref_time(44_219_000)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
