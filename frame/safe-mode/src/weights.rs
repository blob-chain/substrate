// This file is part of Substrate.

// Copyright (C) 2023 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_safe_mode
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-01, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `i9`, CPU: `13th Gen Intel(R) Core(TM) i9-13900K`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet-safe-mode
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/safe-mode/src/weights.rs
// --header=./HEADER-APACHE2
// --template=.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_safe_mode.
pub trait WeightInfo {
	fn on_initialize_exit() -> Weight;
	fn on_initialize_noop() -> Weight;
	fn enter() -> Weight;
	fn force_enter() -> Weight;
	fn extend() -> Weight;
	fn force_extend() -> Weight;
	fn force_exit() -> Weight;
	fn release_stake() -> Weight;
	fn force_release_stake() -> Weight;
	fn force_slash_stake() -> Weight;
}

/// Weights for pallet_safe_mode using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: SafeMode EnteredUntil (r:1 w:1)
	/// Proof: SafeMode EnteredUntil (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn on_initialize_exit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1141`
		//  Estimated: `499`
		// Minimum execution time: 10_678 nanoseconds.
		Weight::from_parts(11_199_000, 499)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SafeMode EnteredUntil (r:1 w:0)
	/// Proof: SafeMode EnteredUntil (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn on_initialize_noop() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `676`
		//  Estimated: `499`
		// Minimum execution time: 2_574 nanoseconds.
		Weight::from_parts(2_747_000, 499)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: SafeMode EnteredUntil (r:1 w:1)
	/// Proof: SafeMode EnteredUntil (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Balances Reserves (r:1 w:1)
	/// Proof: Balances Reserves (max_values: None, max_size: Some(1049), added: 3524, mode: MaxEncodedLen)
	/// Storage: SafeMode Reservations (r:1 w:1)
	/// Proof: SafeMode Reservations (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn enter() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1895`
		//  Estimated: `6566`
		// Minimum execution time: 29_221 nanoseconds.
		Weight::from_parts(31_037_000, 6566)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: SafeMode EnteredUntil (r:1 w:1)
	/// Proof: SafeMode EnteredUntil (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn force_enter() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1258`
		//  Estimated: `499`
		// Minimum execution time: 12_753 nanoseconds.
		Weight::from_parts(12_939_000, 499)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SafeMode EnteredUntil (r:1 w:1)
	/// Proof: SafeMode EnteredUntil (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Balances Reserves (r:1 w:1)
	/// Proof: Balances Reserves (max_values: None, max_size: Some(1049), added: 3524, mode: MaxEncodedLen)
	/// Storage: SafeMode Reservations (r:1 w:1)
	/// Proof: SafeMode Reservations (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn extend() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2228`
		//  Estimated: `6566`
		// Minimum execution time: 33_757 nanoseconds.
		Weight::from_parts(35_096_000, 6566)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: SafeMode EnteredUntil (r:1 w:1)
	/// Proof: SafeMode EnteredUntil (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn force_extend() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1444`
		//  Estimated: `499`
		// Minimum execution time: 15_521 nanoseconds.
		Weight::from_parts(16_387_000, 499)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SafeMode EnteredUntil (r:1 w:1)
	/// Proof: SafeMode EnteredUntil (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn force_exit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1319`
		//  Estimated: `499`
		// Minimum execution time: 13_464 nanoseconds.
		Weight::from_parts(14_153_000, 499)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: SafeMode Reservations (r:1 w:1)
	/// Proof: SafeMode Reservations (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	/// Storage: SafeMode EnteredUntil (r:1 w:0)
	/// Proof: SafeMode EnteredUntil (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Balances Reserves (r:1 w:1)
	/// Proof: Balances Reserves (max_values: None, max_size: Some(1049), added: 3524, mode: MaxEncodedLen)
	fn release_stake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2080`
		//  Estimated: `6566`
		// Minimum execution time: 30_614 nanoseconds.
		Weight::from_parts(32_080_000, 6566)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: SafeMode Reservations (r:1 w:1)
	/// Proof: SafeMode Reservations (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	/// Storage: Balances Reserves (r:1 w:1)
	/// Proof: Balances Reserves (max_values: None, max_size: Some(1049), added: 3524, mode: MaxEncodedLen)
	fn force_release_stake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2080`
		//  Estimated: `6067`
		// Minimum execution time: 28_752 nanoseconds.
		Weight::from_parts(30_362_000, 6067)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: SafeMode Reservations (r:1 w:1)
	/// Proof: SafeMode Reservations (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	/// Storage: Balances Reserves (r:1 w:1)
	/// Proof: Balances Reserves (max_values: None, max_size: Some(1049), added: 3524, mode: MaxEncodedLen)
	fn force_slash_stake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2083`
		//  Estimated: `6067`
		// Minimum execution time: 33_964 nanoseconds.
		Weight::from_parts(35_626_000, 6067)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: SafeMode EnteredUntil (r:1 w:1)
	/// Proof: SafeMode EnteredUntil (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn on_initialize_exit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1141`
		//  Estimated: `499`
		// Minimum execution time: 10_678 nanoseconds.
		Weight::from_parts(11_199_000, 499)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SafeMode EnteredUntil (r:1 w:0)
	/// Proof: SafeMode EnteredUntil (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn on_initialize_noop() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `676`
		//  Estimated: `499`
		// Minimum execution time: 2_574 nanoseconds.
		Weight::from_parts(2_747_000, 499)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: SafeMode EnteredUntil (r:1 w:1)
	/// Proof: SafeMode EnteredUntil (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Balances Reserves (r:1 w:1)
	/// Proof: Balances Reserves (max_values: None, max_size: Some(1049), added: 3524, mode: MaxEncodedLen)
	/// Storage: SafeMode Reservations (r:1 w:1)
	/// Proof: SafeMode Reservations (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn enter() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1895`
		//  Estimated: `6566`
		// Minimum execution time: 29_221 nanoseconds.
		Weight::from_parts(31_037_000, 6566)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: SafeMode EnteredUntil (r:1 w:1)
	/// Proof: SafeMode EnteredUntil (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn force_enter() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1258`
		//  Estimated: `499`
		// Minimum execution time: 12_753 nanoseconds.
		Weight::from_parts(12_939_000, 499)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SafeMode EnteredUntil (r:1 w:1)
	/// Proof: SafeMode EnteredUntil (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Balances Reserves (r:1 w:1)
	/// Proof: Balances Reserves (max_values: None, max_size: Some(1049), added: 3524, mode: MaxEncodedLen)
	/// Storage: SafeMode Reservations (r:1 w:1)
	/// Proof: SafeMode Reservations (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn extend() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2228`
		//  Estimated: `6566`
		// Minimum execution time: 33_757 nanoseconds.
		Weight::from_parts(35_096_000, 6566)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: SafeMode EnteredUntil (r:1 w:1)
	/// Proof: SafeMode EnteredUntil (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn force_extend() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1444`
		//  Estimated: `499`
		// Minimum execution time: 15_521 nanoseconds.
		Weight::from_parts(16_387_000, 499)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SafeMode EnteredUntil (r:1 w:1)
	/// Proof: SafeMode EnteredUntil (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn force_exit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1319`
		//  Estimated: `499`
		// Minimum execution time: 13_464 nanoseconds.
		Weight::from_parts(14_153_000, 499)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: SafeMode Reservations (r:1 w:1)
	/// Proof: SafeMode Reservations (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	/// Storage: SafeMode EnteredUntil (r:1 w:0)
	/// Proof: SafeMode EnteredUntil (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Balances Reserves (r:1 w:1)
	/// Proof: Balances Reserves (max_values: None, max_size: Some(1049), added: 3524, mode: MaxEncodedLen)
	fn release_stake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2080`
		//  Estimated: `6566`
		// Minimum execution time: 30_614 nanoseconds.
		Weight::from_parts(32_080_000, 6566)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: SafeMode Reservations (r:1 w:1)
	/// Proof: SafeMode Reservations (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	/// Storage: Balances Reserves (r:1 w:1)
	/// Proof: Balances Reserves (max_values: None, max_size: Some(1049), added: 3524, mode: MaxEncodedLen)
	fn force_release_stake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2080`
		//  Estimated: `6067`
		// Minimum execution time: 28_752 nanoseconds.
		Weight::from_parts(30_362_000, 6067)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: SafeMode Reservations (r:1 w:1)
	/// Proof: SafeMode Reservations (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	/// Storage: Balances Reserves (r:1 w:1)
	/// Proof: Balances Reserves (max_values: None, max_size: Some(1049), added: 3524, mode: MaxEncodedLen)
	fn force_slash_stake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2083`
		//  Estimated: `6067`
		// Minimum execution time: 33_964 nanoseconds.
		Weight::from_parts(35_626_000, 6067)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}
