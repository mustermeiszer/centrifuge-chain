
//! Autogenerated weights for `pallet_identity`
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
// --pallet=pallet_identity
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/altair/src/weights/pallet_identity.rs

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
		// Minimum execution time: 16_000 nanoseconds.
		Weight::from_ref_time(17_303_984 as u64)
			// Standard Error: 5_704
			.saturating_add(Weight::from_ref_time(88_026 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn set_identity(_r: u32, x: u32, ) -> Weight {
		// Minimum execution time: 35_000 nanoseconds.
		Weight::from_ref_time(36_632_926 as u64)
			// Standard Error: 2_491
			.saturating_add(Weight::from_ref_time(446_378 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SubsOf (r:1 w:1)
	// Storage: Identity SuperOf (r:2 w:2)
	/// The range of component `s` is `[0, 100]`.
	fn set_subs_new(s: u32, ) -> Weight {
		// Minimum execution time: 9_000 nanoseconds.
		Weight::from_ref_time(25_551_932 as u64)
			// Standard Error: 4_731
			.saturating_add(Weight::from_ref_time(2_982_876 as u64).saturating_mul(s as u64))
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
		Weight::from_ref_time(24_811_504 as u64)
			// Standard Error: 4_853
			.saturating_add(Weight::from_ref_time(1_287_853 as u64).saturating_mul(p as u64))
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
		Weight::from_ref_time(19_180_220 as u64)
			// Standard Error: 36_575
			.saturating_add(Weight::from_ref_time(409_698 as u64).saturating_mul(r as u64))
			// Standard Error: 7_142
			.saturating_add(Weight::from_ref_time(1_332_435 as u64).saturating_mul(s as u64))
			// Standard Error: 7_142
			.saturating_add(Weight::from_ref_time(294_712 as u64).saturating_mul(x as u64))
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
		Weight::from_ref_time(35_063_844 as u64)
			// Standard Error: 5_677
			.saturating_add(Weight::from_ref_time(47_111 as u64).saturating_mul(r as u64))
			// Standard Error: 1_107
			.saturating_add(Weight::from_ref_time(467_522 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn cancel_request(r: u32, x: u32, ) -> Weight {
		// Minimum execution time: 32_000 nanoseconds.
		Weight::from_ref_time(31_333_915 as u64)
			// Standard Error: 4_739
			.saturating_add(Weight::from_ref_time(70_224 as u64).saturating_mul(r as u64))
			// Standard Error: 924
			.saturating_add(Weight::from_ref_time(462_908 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn set_fee(r: u32, ) -> Weight {
		// Minimum execution time: 8_000 nanoseconds.
		Weight::from_ref_time(8_833_226 as u64)
			// Standard Error: 3_239
			.saturating_add(Weight::from_ref_time(83_274 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn set_account_id(r: u32, ) -> Weight {
		// Minimum execution time: 8_000 nanoseconds.
		Weight::from_ref_time(8_986_070 as u64)
			// Standard Error: 3_121
			.saturating_add(Weight::from_ref_time(81_295 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity Registrars (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	fn set_fields(r: u32, ) -> Weight {
		// Minimum execution time: 8_000 nanoseconds.
		Weight::from_ref_time(9_162_425 as u64)
			// Standard Error: 3_356
			.saturating_add(Weight::from_ref_time(66_551 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity Registrars (r:1 w:0)
	// Storage: Identity IdentityOf (r:1 w:1)
	/// The range of component `r` is `[1, 19]`.
	/// The range of component `x` is `[0, 100]`.
	fn provide_judgement(r: u32, x: u32, ) -> Weight {
		// Minimum execution time: 25_000 nanoseconds.
		Weight::from_ref_time(24_451_904 as u64)
			// Standard Error: 10_912
			.saturating_add(Weight::from_ref_time(52_561 as u64).saturating_mul(r as u64))
			// Standard Error: 2_019
			.saturating_add(Weight::from_ref_time(773_389 as u64).saturating_mul(x as u64))
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
		Weight::from_ref_time(39_598_618 as u64)
			// Standard Error: 39_658
			.saturating_add(Weight::from_ref_time(459_777 as u64).saturating_mul(r as u64))
			// Standard Error: 7_744
			.saturating_add(Weight::from_ref_time(1_279_939 as u64).saturating_mul(s as u64))
			// Standard Error: 7_744
			.saturating_add(Weight::from_ref_time(265_644 as u64).saturating_mul(x as u64))
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
		Weight::from_ref_time(36_138_010 as u64)
			// Standard Error: 2_869
			.saturating_add(Weight::from_ref_time(66_952 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	/// The range of component `s` is `[1, 100]`.
	fn rename_sub(s: u32, ) -> Weight {
		// Minimum execution time: 12_000 nanoseconds.
		Weight::from_ref_time(13_776_218 as u64)
			// Standard Error: 1_313
			.saturating_add(Weight::from_ref_time(23_106 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Identity IdentityOf (r:1 w:0)
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[1, 100]`.
	fn remove_sub(s: u32, ) -> Weight {
		// Minimum execution time: 34_000 nanoseconds.
		Weight::from_ref_time(37_224_006 as u64)
			// Standard Error: 2_911
			.saturating_add(Weight::from_ref_time(55_457 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Identity SuperOf (r:1 w:1)
	// Storage: Identity SubsOf (r:1 w:1)
	/// The range of component `s` is `[0, 99]`.
	fn quit_sub(s: u32, ) -> Weight {
		// Minimum execution time: 23_000 nanoseconds.
		Weight::from_ref_time(25_999_137 as u64)
			// Standard Error: 1_833
			.saturating_add(Weight::from_ref_time(53_954 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
