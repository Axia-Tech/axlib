// This file is part of Axlib.

// Copyright (C) 2021 AXIA Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_vesting
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-08-10, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/axlib
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_vesting
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/vesting/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_vesting.
pub trait WeightInfo {
	fn vest_locked(l: u32, s: u32, ) -> Weight;
	fn vest_unlocked(l: u32, s: u32, ) -> Weight;
	fn vest_other_locked(l: u32, s: u32, ) -> Weight;
	fn vest_other_unlocked(l: u32, s: u32, ) -> Weight;
	fn vested_transfer(l: u32, s: u32, ) -> Weight;
	fn force_vested_transfer(l: u32, s: u32, ) -> Weight;
	fn not_unlocking_merge_schedules(l: u32, s: u32, ) -> Weight;
	fn unlocking_merge_schedules(l: u32, s: u32, ) -> Weight;
}

/// Weights for pallet_vesting using the Axlib node and recommended hardware.
pub struct AxlibWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AxlibWeight<T> {
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vest_locked(l: u32, s: u32, ) -> Weight {
		(50_642_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((144_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 3_000
			.saturating_add((177_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vest_unlocked(l: u32, s: u32, ) -> Weight {
		(50_830_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((115_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 3_000
			.saturating_add((112_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn vest_other_locked(l: u32, s: u32, ) -> Weight {
		(52_151_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((130_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 3_000
			.saturating_add((162_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn vest_other_unlocked(l: u32, s: u32, ) -> Weight {
		(51_009_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((123_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 9_000
			.saturating_add((118_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vested_transfer(l: u32, s: u32, ) -> Weight {
		(89_517_000 as Weight)
			// Standard Error: 5_000
			.saturating_add((114_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 10_000
			.saturating_add((23_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Locks (r:1 w:1)
	fn force_vested_transfer(l: u32, s: u32, ) -> Weight {
		(87_903_000 as Weight)
			// Standard Error: 6_000
			.saturating_add((121_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 12_000
			.saturating_add((56_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn not_unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		(54_463_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((123_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 5_000
			.saturating_add((149_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		(53_674_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((137_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 4_000
			.saturating_add((152_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vest_locked(l: u32, s: u32, ) -> Weight {
		(50_642_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((144_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 3_000
			.saturating_add((177_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vest_unlocked(l: u32, s: u32, ) -> Weight {
		(50_830_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((115_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 3_000
			.saturating_add((112_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn vest_other_locked(l: u32, s: u32, ) -> Weight {
		(52_151_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((130_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 3_000
			.saturating_add((162_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn vest_other_unlocked(l: u32, s: u32, ) -> Weight {
		(51_009_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((123_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 9_000
			.saturating_add((118_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn vested_transfer(l: u32, s: u32, ) -> Weight {
		(89_517_000 as Weight)
			// Standard Error: 5_000
			.saturating_add((114_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 10_000
			.saturating_add((23_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Locks (r:1 w:1)
	fn force_vested_transfer(l: u32, s: u32, ) -> Weight {
		(87_903_000 as Weight)
			// Standard Error: 6_000
			.saturating_add((121_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 12_000
			.saturating_add((56_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn not_unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		(54_463_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((123_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 5_000
			.saturating_add((149_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		(53_674_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((137_000 as Weight).saturating_mul(l as Weight))
			// Standard Error: 4_000
			.saturating_add((152_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
}
