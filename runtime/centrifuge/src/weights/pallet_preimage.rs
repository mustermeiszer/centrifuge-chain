
//! Autogenerated weights for `pallet_preimage`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-09, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `mq.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("centrifuge-dev"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=centrifuge-dev
// --steps=50
// --repeat=20
// --pallet=pallet_preimage
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/centrifuge/src/weights/pallet_preimage.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_preimage`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_preimage::WeightInfo for WeightInfo<T> {
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Preimage PreimageFor (r:0 w:1)
	/// The range of component `s` is `[0, 4194304]`.
	fn note_preimage(s: u32, ) -> Weight {
		// Minimum execution time: 28_000 nanoseconds.
		Weight::from_ref_time(29_000_000 as u64)
			// Standard Error: 1
			.saturating_add(Weight::from_ref_time(1_403 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Preimage PreimageFor (r:0 w:1)
	/// The range of component `s` is `[0, 4194304]`.
	fn note_requested_preimage(s: u32, ) -> Weight {
		// Minimum execution time: 19_000 nanoseconds.
		Weight::from_ref_time(19_000_000 as u64)
			// Standard Error: 2
			.saturating_add(Weight::from_ref_time(1_422 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Preimage PreimageFor (r:0 w:1)
	/// The range of component `s` is `[0, 4194304]`.
	fn note_no_deposit_preimage(s: u32, ) -> Weight {
		// Minimum execution time: 18_000 nanoseconds.
		Weight::from_ref_time(19_000_000 as u64)
			// Standard Error: 1
			.saturating_add(Weight::from_ref_time(1_391 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Preimage PreimageFor (r:0 w:1)
	fn unnote_preimage() -> Weight {
		// Minimum execution time: 38_000 nanoseconds.
		Weight::from_ref_time(40_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Preimage PreimageFor (r:0 w:1)
	fn unnote_no_deposit_preimage() -> Weight {
		// Minimum execution time: 27_000 nanoseconds.
		Weight::from_ref_time(29_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	fn request_preimage() -> Weight {
		// Minimum execution time: 25_000 nanoseconds.
		Weight::from_ref_time(28_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	fn request_no_deposit_preimage() -> Weight {
		// Minimum execution time: 14_000 nanoseconds.
		Weight::from_ref_time(21_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	fn request_unnoted_preimage() -> Weight {
		// Minimum execution time: 15_000 nanoseconds.
		Weight::from_ref_time(16_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	fn request_requested_preimage() -> Weight {
		// Minimum execution time: 8_000 nanoseconds.
		Weight::from_ref_time(9_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Preimage PreimageFor (r:0 w:1)
	fn unrequest_preimage() -> Weight {
		// Minimum execution time: 26_000 nanoseconds.
		Weight::from_ref_time(28_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	fn unrequest_unnoted_preimage() -> Weight {
		// Minimum execution time: 8_000 nanoseconds.
		Weight::from_ref_time(8_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	fn unrequest_multi_referenced_preimage() -> Weight {
		// Minimum execution time: 8_000 nanoseconds.
		Weight::from_ref_time(8_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
