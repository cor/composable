
//! Autogenerated weights for oracle
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-08-25, STEPS: `[5, ]`, REPEAT: 2, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("picasso-dev"), DB CACHE: 128

// Executed Command:
// ./target/release/composable
// benchmark
// --chain=picasso-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=oracle
// --extrinsic=*
// --steps=5
// --repeat=2
// --raw
// --output=./runtime/picasso/src/weights


#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for oracle.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> oracle::WeightInfo for WeightInfo<T> {
	fn add_asset_and_info() -> Weight {
		(30_913_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn request_price() -> Weight {
		(95_050_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn set_signer() -> Weight {
		(151_495_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn add_stake() -> Weight {
		(219_457_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn remove_stake() -> Weight {
		(42_512_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn reclaim_stake() -> Weight {
		(51_245_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn submit_price() -> Weight {
		(181_547_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
