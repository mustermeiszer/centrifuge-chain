
//! Autogenerated weights for `pallet_restricted_tokens`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-10, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `mq.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("altair-dev"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=altair-dev
// --steps=50
// --repeat=20
// --pallet=pallet_restricted_tokens
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/altair/src/weights/pallet_restricted_tokens.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_restricted_tokens`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_restricted_tokens::WeightInfo for WeightInfo<T> {
	// Storage: System Account (r:1 w:1)
	fn transfer_native() -> Weight {
		// Minimum execution time: 42_000 nanoseconds.
		Weight::from_ref_time(43_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: OrmlTokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn transfer_other() -> Weight {
		// Minimum execution time: 43_000 nanoseconds.
		Weight::from_ref_time(44_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: System Account (r:1 w:1)
	fn transfer_keep_alive_native() -> Weight {
		// Minimum execution time: 38_000 nanoseconds.
		Weight::from_ref_time(39_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: OrmlTokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn transfer_keep_alive_other() -> Weight {
		// Minimum execution time: 40_000 nanoseconds.
		Weight::from_ref_time(41_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: System Account (r:1 w:1)
	fn transfer_all_native() -> Weight {
		// Minimum execution time: 45_000 nanoseconds.
		Weight::from_ref_time(46_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: OrmlTokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn transfer_all_other() -> Weight {
		// Minimum execution time: 46_000 nanoseconds.
		Weight::from_ref_time(47_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: System Account (r:1 w:1)
	fn force_transfer_native() -> Weight {
		// Minimum execution time: 42_000 nanoseconds.
		Weight::from_ref_time(43_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: OrmlTokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn force_transfer_other() -> Weight {
		// Minimum execution time: 42_000 nanoseconds.
		Weight::from_ref_time(43_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: System Account (r:1 w:1)
	fn set_balance_native() -> Weight {
		// Minimum execution time: 43_000 nanoseconds.
		Weight::from_ref_time(44_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: OrmlTokens Accounts (r:1 w:1)
	// Storage: OrmlTokens TotalIssuance (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn set_balance_other() -> Weight {
		// Minimum execution time: 51_000 nanoseconds.
		Weight::from_ref_time(51_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
}
