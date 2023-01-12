
//! Autogenerated weights for `pallet_identity`
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
// --pallet=pallet_identity
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/centrifuge/src/weights/pallet_identity.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_identity`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_identity::WeightInfo for WeightInfo<T> {
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn add_registrar(r: u32, ) -> Weight {
		// Minimum execution time: 15_000 nanoseconds.
		Weight::from_ref_time(16_493_888 as u64)
			// Standard Error: 2_969
			.saturating_add(Weight::from_ref_time(105_053 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn set_identity(r: u32, x: u32, ) -> Weight {
		// Minimum execution time: 34_000 nanoseconds.
		Weight::from_ref_time(33_751_607 as u64)
			// Standard Error: 7_870
			.saturating_add(Weight::from_ref_time(45_898 as u64).saturating_mul(r as u64))
			// Standard Error: 1_535
			.saturating_add(Weight::from_ref_time(455_824 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:2 w:2)
	/// The range of component `s` is `[0, 100]`.
	fn set_subs_new(s: u32, ) -> Weight {
		// Minimum execution time: 9_000 nanoseconds.
		Weight::from_ref_time(24_997_029 as u64)
			// Standard Error: 4_001
			.saturating_add(Weight::from_ref_time(2_971_627 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:2)
	/// The range of component `p` is `[0, 100]`.
	fn set_subs_old(p: u32, ) -> Weight {
		// Minimum execution time: 9_000 nanoseconds.
		Weight::from_ref_time(25_210_004 as u64)
			// Standard Error: 3_895
			.saturating_add(Weight::from_ref_time(1_248_344 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(p as u64)))
	}
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity IdentityOf (r:1 w:1)
	// Storage: Identity SuperOf (r:0 w:100)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	/// The range of component `x` is `[0, 100]`.
	fn clear_identity(r: u32, s: u32, x: u32, ) -> Weight {
		// Minimum execution time: 54_000 nanoseconds.
		Weight::from_ref_time(31_944_453 as u64)
			// Standard Error: 10_799
			.saturating_add(Weight::from_ref_time(99_133 as u64).saturating_mul(r as u64))
			// Standard Error: 2_108
			.saturating_add(Weight::from_ref_time(1_234_024 as u64).saturating_mul(s as u64))
			// Standard Error: 2_108
			.saturating_add(Weight::from_ref_time(242_474 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	// Storage: Identity Registrars (r:1 w:0)
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn request_judgement(r: u32, x: u32, ) -> Weight {
		// Minimum execution time: 35_000 nanoseconds.
		Weight::from_ref_time(33_191_466 as u64)
			// Standard Error: 8_672
			.saturating_add(Weight::from_ref_time(153_004 as u64).saturating_mul(r as u64))
			// Standard Error: 1_692
			.saturating_add(Weight::from_ref_time(468_685 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn cancel_request(r: u32, x: u32, ) -> Weight {
		// Minimum execution time: 32_000 nanoseconds.
		Weight::from_ref_time(31_359_222 as u64)
			// Standard Error: 2_602
			.saturating_add(Weight::from_ref_time(44_200 as u64).saturating_mul(r as u64))
			// Standard Error: 507
			.saturating_add(Weight::from_ref_time(461_454 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn set_fee(r: u32, ) -> Weight {
		// Minimum execution time: 8_000 nanoseconds.
		Weight::from_ref_time(8_749_314 as u64)
			// Standard Error: 2_910
			.saturating_add(Weight::from_ref_time(87_677 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn set_account_id(r: u32, ) -> Weight {
		// Minimum execution time: 8_000 nanoseconds.
		Weight::from_ref_time(8_953_756 as u64)
			// Standard Error: 2_639
			.saturating_add(Weight::from_ref_time(74_185 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn set_fields(r: u32, ) -> Weight {
		// Minimum execution time: 8_000 nanoseconds.
		Weight::from_ref_time(8_682_970 as u64)
			// Standard Error: 3_173
			.saturating_add(Weight::from_ref_time(111_452 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity Registrars (r:1 w:0)
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	/// The range of component `x` is `[0, 100]`.
	fn provide_judgement(_r: u32, x: u32, ) -> Weight {
		// Minimum execution time: 25_000 nanoseconds.
		Weight::from_ref_time(27_708_653 as u64)
			// Standard Error: 1_376
			.saturating_add(Weight::from_ref_time(749_930 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity IdentityOf (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Identity SuperOf (r:0 w:100)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	/// The range of component `x` is `[0, 100]`.
	fn kill_identity(r: u32, s: u32, x: u32, ) -> Weight {
		// Minimum execution time: 68_000 nanoseconds.
		Weight::from_ref_time(39_242_400 as u64)
			// Standard Error: 37_610
			.saturating_add(Weight::from_ref_time(517_070 as u64).saturating_mul(r as u64))
			// Standard Error: 7_344
			.saturating_add(Weight::from_ref_time(1_285_144 as u64).saturating_mul(s as u64))
			// Standard Error: 7_344
			.saturating_add(Weight::from_ref_time(225_896 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(s as u64)))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[0, 99]`.
	fn add_sub(s: u32, ) -> Weight {
		// Minimum execution time: 31_000 nanoseconds.
		Weight::from_ref_time(35_332_157 as u64)
			// Standard Error: 1_232
			.saturating_add(Weight::from_ref_time(45_814 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	/// The range of component `s` is `[1, 100]`.
	fn rename_sub(s: u32, ) -> Weight {
		// Minimum execution time: 12_000 nanoseconds.
		Weight::from_ref_time(13_993_977 as u64)
			// Standard Error: 987
			.saturating_add(Weight::from_ref_time(7_957 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[1, 100]`.
	fn remove_sub(s: u32, ) -> Weight {
		// Minimum execution time: 34_000 nanoseconds.
		Weight::from_ref_time(36_858_140 as u64)
			// Standard Error: 1_809
			.saturating_add(Weight::from_ref_time(36_822 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[0, 99]`.
	fn quit_sub(s: u32, ) -> Weight {
		// Minimum execution time: 23_000 nanoseconds.
		Weight::from_ref_time(25_399_083 as u64)
			// Standard Error: 1_858
			.saturating_add(Weight::from_ref_time(65_053 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
