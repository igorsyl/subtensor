
//! Autogenerated weights for `pallet_crowdloan`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 43.0.0
//! DATE: 2025-05-01, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Ubuntu-2404-noble-amd64-base`, CPU: `AMD Ryzen 9 7950X3D 16-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("local")`, DB CACHE: `1024`

// Executed Command:
// ./target/production/node-subtensor
// benchmark
// pallet
// --chain=local
// --wasm-execution=compiled
// --pallet=pallet-crowdloan
// --extrinsic=*
// --steps=50
// --repeat=20
// --output=pallets/crowdloan/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs
// --allow-missing-host-functions

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_crowdloan`.
pub trait WeightInfo {
	fn create() -> Weight;
	fn contribute() -> Weight;
	fn withdraw() -> Weight;
	fn finalize() -> Weight;
	fn refund(k: u32, ) -> Weight;
	fn dissolve() -> Weight;
	fn update_min_contribution() -> Weight;
	fn update_end() -> Weight;
	fn update_cap() -> Weight;
}

/// Weights for `pallet_crowdloan` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(104), added: 2579, mode: `MaxEncodedLen`)
	/// Storage: `Crowdloan::NextCrowdloanId` (r:1 w:1)
	/// Proof: `Crowdloan::NextCrowdloanId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Crowdloan::Contributions` (r:0 w:1)
	/// Proof: `Crowdloan::Contributions` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Crowdloan::Crowdloans` (r:0 w:1)
	/// Proof: `Crowdloan::Crowdloans` (`max_values`: None, `max_size`: Some(278), added: 2753, mode: `MaxEncodedLen`)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `156`
		//  Estimated: `6148`
		// Minimum execution time: 42_128_000 picoseconds.
		Weight::from_parts(42_930_000, 6148)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `Crowdloan::Crowdloans` (r:1 w:1)
	/// Proof: `Crowdloan::Crowdloans` (`max_values`: None, `max_size`: Some(278), added: 2753, mode: `MaxEncodedLen`)
	/// Storage: `Crowdloan::Contributions` (r:1 w:1)
	/// Proof: `Crowdloan::Contributions` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(104), added: 2579, mode: `MaxEncodedLen`)
	fn contribute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `476`
		//  Estimated: `6148`
		// Minimum execution time: 43_161_000 picoseconds.
		Weight::from_parts(44_192_000, 6148)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Crowdloan::Crowdloans` (r:1 w:1)
	/// Proof: `Crowdloan::Crowdloans` (`max_values`: None, `max_size`: Some(278), added: 2753, mode: `MaxEncodedLen`)
	/// Storage: `Crowdloan::Contributions` (r:1 w:1)
	/// Proof: `Crowdloan::Contributions` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(104), added: 2579, mode: `MaxEncodedLen`)
	fn withdraw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `436`
		//  Estimated: `6148`
		// Minimum execution time: 40_235_000 picoseconds.
		Weight::from_parts(40_907_000, 6148)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Crowdloan::Crowdloans` (r:1 w:1)
	/// Proof: `Crowdloan::Crowdloans` (`max_values`: None, `max_size`: Some(278), added: 2753, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(104), added: 2579, mode: `MaxEncodedLen`)
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:0)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Crowdloan::CurrentCrowdloanId` (r:0 w:1)
	/// Proof: `Crowdloan::CurrentCrowdloanId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn finalize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `376`
		//  Estimated: `6148`
		// Minimum execution time: 40_986_000 picoseconds.
		Weight::from_parts(41_858_000, 6148)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Crowdloan::Crowdloans` (r:1 w:1)
	/// Proof: `Crowdloan::Crowdloans` (`max_values`: None, `max_size`: Some(278), added: 2753, mode: `MaxEncodedLen`)
	/// Storage: `Crowdloan::Contributions` (r:51 w:49)
	/// Proof: `Crowdloan::Contributions` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:50 w:50)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(104), added: 2579, mode: `MaxEncodedLen`)
	/// The range of component `k` is `[3, 50]`.
	fn refund(k: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `372 + k * (49 ±0)`
		//  Estimated: `3743 + k * (2579 ±0)`
		// Minimum execution time: 78_938_000 picoseconds.
		Weight::from_parts(2_729_302, 3743)
			// Standard Error: 351_422
			.saturating_add(Weight::from_parts(31_033_274, 0).saturating_mul(k.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(k.into())))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(k.into())))
			.saturating_add(Weight::from_parts(0, 2579).saturating_mul(k.into()))
	}
	/// Storage: `Crowdloan::Crowdloans` (r:1 w:1)
	/// Proof: `Crowdloan::Crowdloans` (`max_values`: None, `max_size`: Some(278), added: 2753, mode: `MaxEncodedLen`)
	/// Storage: `Crowdloan::Contributions` (r:1 w:0)
	/// Proof: `Crowdloan::Contributions` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(104), added: 2579, mode: `MaxEncodedLen`)
	fn dissolve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `450`
		//  Estimated: `6148`
		// Minimum execution time: 43_341_000 picoseconds.
		Weight::from_parts(44_402_000, 6148)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Crowdloan::Crowdloans` (r:1 w:1)
	/// Proof: `Crowdloan::Crowdloans` (`max_values`: None, `max_size`: Some(278), added: 2753, mode: `MaxEncodedLen`)
	fn update_min_contribution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `224`
		//  Estimated: `3743`
		// Minimum execution time: 8_876_000 picoseconds.
		Weight::from_parts(9_137_000, 3743)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Crowdloan::Crowdloans` (r:1 w:1)
	/// Proof: `Crowdloan::Crowdloans` (`max_values`: None, `max_size`: Some(278), added: 2753, mode: `MaxEncodedLen`)
	fn update_end() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `224`
		//  Estimated: `3743`
		// Minimum execution time: 9_117_000 picoseconds.
		Weight::from_parts(9_438_000, 3743)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Crowdloan::Crowdloans` (r:1 w:1)
	/// Proof: `Crowdloan::Crowdloans` (`max_values`: None, `max_size`: Some(278), added: 2753, mode: `MaxEncodedLen`)
	fn update_cap() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `224`
		//  Estimated: `3743`
		// Minimum execution time: 8_766_000 picoseconds.
		Weight::from_parts(9_087_000, 3743)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(104), added: 2579, mode: `MaxEncodedLen`)
	/// Storage: `Crowdloan::NextCrowdloanId` (r:1 w:1)
	/// Proof: `Crowdloan::NextCrowdloanId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Crowdloan::Contributions` (r:0 w:1)
	/// Proof: `Crowdloan::Contributions` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Crowdloan::Crowdloans` (r:0 w:1)
	/// Proof: `Crowdloan::Crowdloans` (`max_values`: None, `max_size`: Some(278), added: 2753, mode: `MaxEncodedLen`)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `156`
		//  Estimated: `6148`
		// Minimum execution time: 42_128_000 picoseconds.
		Weight::from_parts(42_930_000, 6148)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: `Crowdloan::Crowdloans` (r:1 w:1)
	/// Proof: `Crowdloan::Crowdloans` (`max_values`: None, `max_size`: Some(278), added: 2753, mode: `MaxEncodedLen`)
	/// Storage: `Crowdloan::Contributions` (r:1 w:1)
	/// Proof: `Crowdloan::Contributions` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(104), added: 2579, mode: `MaxEncodedLen`)
	fn contribute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `476`
		//  Estimated: `6148`
		// Minimum execution time: 43_161_000 picoseconds.
		Weight::from_parts(44_192_000, 6148)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Crowdloan::Crowdloans` (r:1 w:1)
	/// Proof: `Crowdloan::Crowdloans` (`max_values`: None, `max_size`: Some(278), added: 2753, mode: `MaxEncodedLen`)
	/// Storage: `Crowdloan::Contributions` (r:1 w:1)
	/// Proof: `Crowdloan::Contributions` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(104), added: 2579, mode: `MaxEncodedLen`)
	fn withdraw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `436`
		//  Estimated: `6148`
		// Minimum execution time: 40_235_000 picoseconds.
		Weight::from_parts(40_907_000, 6148)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Crowdloan::Crowdloans` (r:1 w:1)
	/// Proof: `Crowdloan::Crowdloans` (`max_values`: None, `max_size`: Some(278), added: 2753, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(104), added: 2579, mode: `MaxEncodedLen`)
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:0)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Crowdloan::CurrentCrowdloanId` (r:0 w:1)
	/// Proof: `Crowdloan::CurrentCrowdloanId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn finalize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `376`
		//  Estimated: `6148`
		// Minimum execution time: 40_986_000 picoseconds.
		Weight::from_parts(41_858_000, 6148)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Crowdloan::Crowdloans` (r:1 w:1)
	/// Proof: `Crowdloan::Crowdloans` (`max_values`: None, `max_size`: Some(278), added: 2753, mode: `MaxEncodedLen`)
	/// Storage: `Crowdloan::Contributions` (r:51 w:49)
	/// Proof: `Crowdloan::Contributions` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:50 w:50)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(104), added: 2579, mode: `MaxEncodedLen`)
	/// The range of component `k` is `[3, 50]`.
	fn refund(k: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `372 + k * (49 ±0)`
		//  Estimated: `3743 + k * (2579 ±0)`
		// Minimum execution time: 78_938_000 picoseconds.
		Weight::from_parts(2_729_302, 3743)
			// Standard Error: 351_422
			.saturating_add(Weight::from_parts(31_033_274, 0).saturating_mul(k.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().reads((2_u64).saturating_mul(k.into())))
			.saturating_add(RocksDbWeight::get().writes((2_u64).saturating_mul(k.into())))
			.saturating_add(Weight::from_parts(0, 2579).saturating_mul(k.into()))
	}
	/// Storage: `Crowdloan::Crowdloans` (r:1 w:1)
	/// Proof: `Crowdloan::Crowdloans` (`max_values`: None, `max_size`: Some(278), added: 2753, mode: `MaxEncodedLen`)
	/// Storage: `Crowdloan::Contributions` (r:1 w:0)
	/// Proof: `Crowdloan::Contributions` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(104), added: 2579, mode: `MaxEncodedLen`)
	fn dissolve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `450`
		//  Estimated: `6148`
		// Minimum execution time: 43_341_000 picoseconds.
		Weight::from_parts(44_402_000, 6148)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `Crowdloan::Crowdloans` (r:1 w:1)
	/// Proof: `Crowdloan::Crowdloans` (`max_values`: None, `max_size`: Some(278), added: 2753, mode: `MaxEncodedLen`)
	fn update_min_contribution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `224`
		//  Estimated: `3743`
		// Minimum execution time: 8_876_000 picoseconds.
		Weight::from_parts(9_137_000, 3743)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Crowdloan::Crowdloans` (r:1 w:1)
	/// Proof: `Crowdloan::Crowdloans` (`max_values`: None, `max_size`: Some(278), added: 2753, mode: `MaxEncodedLen`)
	fn update_end() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `224`
		//  Estimated: `3743`
		// Minimum execution time: 9_117_000 picoseconds.
		Weight::from_parts(9_438_000, 3743)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Crowdloan::Crowdloans` (r:1 w:1)
	/// Proof: `Crowdloan::Crowdloans` (`max_values`: None, `max_size`: Some(278), added: 2753, mode: `MaxEncodedLen`)
	fn update_cap() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `224`
		//  Estimated: `3743`
		// Minimum execution time: 8_766_000 picoseconds.
		Weight::from_parts(9_087_000, 3743)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}