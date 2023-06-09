
//! Autogenerated weights for pallet_club
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-06, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Alexs-MacBook-Pro-2.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ../../target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_club
// --extrinsic
// *
// --steps=50
// --repeat=20
// --execution=wasm
// --wasm-execution=compiled
// --output
// pallets/template/src/weights.rs
// --template
// ../../.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_club.
pub trait WeightInfo {
	fn create_club() -> Weight;
	fn add_member() -> Weight;
	fn transfer_ownership() -> Weight;
	fn set_annual_expense() -> Weight;
	fn pay_membership_expense() -> Weight;
}

/// Weights for pallet_club using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn create_club() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32`
		//  Estimated: `1489`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(6_000_000, 1489)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}

	fn add_member() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32`
		//  Estimated: `1489`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(6_000_000, 1489)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}

	fn transfer_ownership() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32`
		//  Estimated: `1489`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(6_000_000, 1489)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}

	fn set_annual_expense() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32`
		//  Estimated: `1489`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(6_000_000, 1489)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}

	fn pay_membership_expense() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32`
		//  Estimated: `1489`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(6_000_000, 1489)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn create_club() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32`
		//  Estimated: `1489`
		// Minimum execution time: 7_000_000 picoseconds.
		Weight::from_parts(6_000_000, 1489)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}

	fn add_member() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32`
		//  Estimated: `1489`
		// Minimum execution time: 7_000_000 picoseconds.
		Weight::from_parts(6_000_000, 1489)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}

	fn transfer_ownership() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32`
		//  Estimated: `1489`
		// Minimum execution time: 7_000_000 picoseconds.
		Weight::from_parts(6_000_000, 1489)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}

	fn set_annual_expense() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32`
		//  Estimated: `1489`
		// Minimum execution time: 7_000_000 picoseconds.
		Weight::from_parts(6_000_000, 1489)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}

	fn pay_membership_expense() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32`
		//  Estimated: `1489`
		// Minimum execution time: 7_000_000 picoseconds.
		Weight::from_parts(6_000_000, 1489)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
