
//! Autogenerated weights for `proxy`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-25, STEPS: `2`, REPEAT: 2, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `dev`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dali-dev"), DB CACHE: 1024

// Executed Command:
// /nix/store/gpsh9wvfcrwyck2nw62gpkqhf0bhc0cw-composable/bin/composable
// benchmark
// pallet
// --chain=dali-dev
// --execution=wasm
// --wasm-execution=compiled
// --wasm-instantiation-strategy=legacy-instance-reuse
// --pallet=*
// --extrinsic=*
// --steps=2
// --repeat=2
// --output=code/parachain/runtime/dali/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `proxy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> proxy::WeightInfo for WeightInfo<T> {
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: CallFilter DisabledCalls (r:1 w:0)
	/// The range of component `p` is `[1, 3]`.
	fn proxy(_p: u32, ) -> Weight {
		// Minimum execution time: 33_959 nanoseconds.
		Weight::from_ref_time(45_729_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: CallFilter DisabledCalls (r:1 w:0)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 3]`.
	fn proxy_announced(a: u32, p: u32, ) -> Weight {
		// Minimum execution time: 47_500 nanoseconds.
		Weight::from_ref_time(66_583_000 as u64)
			// Standard Error: 1_400_634
			.saturating_add(Weight::from_ref_time(1_606_209 as u64).saturating_mul(a as u64))
			// Standard Error: 21_709_834
			.saturating_add(Weight::from_ref_time(771_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Proxy Announcements (r:1 w:1)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 3]`.
	fn remove_announcement(a: u32, _p: u32, ) -> Weight {
		// Minimum execution time: 13_667 nanoseconds.
		Weight::from_ref_time(24_219_750 as u64)
			// Standard Error: 166_428
			.saturating_add(Weight::from_ref_time(69_870 as u64).saturating_mul(a as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Proxy Announcements (r:1 w:1)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 3]`.
	fn reject_announcement(_a: u32, _p: u32, ) -> Weight {
		// Minimum execution time: 19_542 nanoseconds.
		Weight::from_ref_time(52_719_250 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: Proxy Announcements (r:1 w:1)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 3]`.
	fn announce(a: u32, _p: u32, ) -> Weight {
		// Minimum execution time: 33_500 nanoseconds.
		Weight::from_ref_time(132_958_750 as u64)
			// Standard Error: 150_204
			.saturating_add(Weight::from_ref_time(291_677 as u64).saturating_mul(a as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[1, 3]`.
	fn add_proxy(_p: u32, ) -> Weight {
		// Minimum execution time: 24_125 nanoseconds.
		Weight::from_ref_time(87_042_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[1, 3]`.
	fn remove_proxy(p: u32, ) -> Weight {
		// Minimum execution time: 22_667 nanoseconds.
		Weight::from_ref_time(23_323_750 as u64)
			// Standard Error: 1_574_824
			.saturating_add(Weight::from_ref_time(1_051_750 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[1, 3]`.
	fn remove_proxies(p: u32, ) -> Weight {
		// Minimum execution time: 12_459 nanoseconds.
		Weight::from_ref_time(11_854_750 as u64)
			// Standard Error: 1_617_462
			.saturating_add(Weight::from_ref_time(1_270_750 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[1, 3]`.
	fn create_pure(_p: u32, ) -> Weight {
		// Minimum execution time: 26_459 nanoseconds.
		Weight::from_ref_time(48_531_500 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[0, 2]`.
	fn kill_pure(p: u32, ) -> Weight {
		// Minimum execution time: 12_416 nanoseconds.
		Weight::from_ref_time(12_812_000 as u64)
			// Standard Error: 2_518_295
			.saturating_add(Weight::from_ref_time(2_833_500 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
