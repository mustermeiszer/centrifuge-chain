//! Autogenerated weights for pallet_scheduler
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
// --pallet=pallet_scheduler
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/altair/src/weights/pallet_scheduler.rs
// --template=./scripts/runtime-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weights for pallet_scheduler using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_scheduler::WeightInfo for WeightInfo<T> {
	fn on_initialize_periodic_named_resolved(s: u32) -> Weight {
		Weight::from_ref_time(0) // Standard Error: 1_658_000
			.saturating_add(Weight::from_ref_time(68_895_000).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((4 as u64).saturating_mul(s as u64)))
	}

	fn on_initialize_named_resolved(s: u32) -> Weight {
		Weight::from_ref_time(1_026_902_000) // Standard Error: 4_168_000
			.saturating_add(Weight::from_ref_time(30_681_000).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(s as u64)))
	}

	fn on_initialize_periodic_resolved(s: u32) -> Weight {
		Weight::from_ref_time(21_132_000) // Standard Error: 104_000
			.saturating_add(Weight::from_ref_time(47_447_000).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(s as u64)))
	}

	fn on_initialize_resolved(s: u32) -> Weight {
		Weight::from_ref_time(21_515_000) // Standard Error: 53_000
			.saturating_add(Weight::from_ref_time(40_099_000).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(s as u64)))
	}

	fn on_initialize_named_aborted(s: u32) -> Weight {
		Weight::from_ref_time(10_966_000) // Standard Error: 44_000
			.saturating_add(Weight::from_ref_time(17_973_000).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}

	fn on_initialize_aborted(s: u32) -> Weight {
		Weight::from_ref_time(14_361_000) // Standard Error: 14_000
			.saturating_add(Weight::from_ref_time(11_119_000).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}

	fn on_initialize_periodic_named(s: u32) -> Weight {
		Weight::from_ref_time(29_948_000) // Standard Error: 52_000
			.saturating_add(Weight::from_ref_time(30_506_000).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(s as u64)))
	}

	fn on_initialize_periodic(s: u32) -> Weight {
		Weight::from_ref_time(13_393_000) // Standard Error: 97_000
			.saturating_add(Weight::from_ref_time(24_304_000).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}

	fn on_initialize_named(s: u32) -> Weight {
		Weight::from_ref_time(29_155_000) // Standard Error: 37_000
			.saturating_add(Weight::from_ref_time(19_455_000).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}

	fn on_initialize(s: u32) -> Weight {
		Weight::from_ref_time(28_733_000) // Standard Error: 19_000
			.saturating_add(Weight::from_ref_time(16_365_000).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn schedule(s: u32) -> Weight {
		Weight::from_ref_time(41_487_000) // Standard Error: 4_000
			.saturating_add(Weight::from_ref_time(251_000).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}

	fn cancel(s: u32) -> Weight {
		Weight::from_ref_time(43_599_000) // Standard Error: 8_000
			.saturating_add(Weight::from_ref_time(2_974_000).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}

	fn schedule_named(s: u32) -> Weight {
		Weight::from_ref_time(49_376_000) // Standard Error: 5_000
			.saturating_add(Weight::from_ref_time(312_000).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}

	fn cancel_named(s: u32) -> Weight {
		Weight::from_ref_time(45_701_000) // Standard Error: 10_000
			.saturating_add(Weight::from_ref_time(3_061_000).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
