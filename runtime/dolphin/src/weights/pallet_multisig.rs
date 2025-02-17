// Copyright 2020-2023 Manta Network.
// This file is part of Manta.
//
// Manta is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Manta is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Manta.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_multisig
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dolphin-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/manta
// benchmark
// pallet
// --chain=dolphin-dev
// --steps=50
// --repeat=20
// --pallet=pallet_multisig
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./scripts/benchmarking/frame-weights-output/pallet_multisig.rs
// --template=.github/resources/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;
use manta_primitives::constants::RocksDbWeight;

/// Weight functions needed for pallet_multisig.
pub trait WeightInfo {
    fn as_multi_threshold_1(z: u32, ) -> Weight;
    fn as_multi_create(s: u32, z: u32, ) -> Weight;
    fn as_multi_approve(s: u32, z: u32, ) -> Weight;
    fn as_multi_complete(s: u32, z: u32, ) -> Weight;
    fn approve_as_multi_create(s: u32, ) -> Weight;
    fn approve_as_multi_approve(s: u32, ) -> Weight;
    fn cancel_as_multi(s: u32, ) -> Weight;
}

/// Weights for pallet_multisig using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_multisig::WeightInfo for SubstrateWeight<T> {
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_threshold_1(z: u32, ) -> Weight {
		// Minimum execution time: 17_705 nanoseconds.
		Weight::from_ref_time(18_611_308)
			// Standard Error: 10
			.saturating_add(Weight::from_ref_time(649).saturating_mul(z.into()))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		// Minimum execution time: 52_394 nanoseconds.
		Weight::from_ref_time(42_628_113)
			// Standard Error: 7_162
			.saturating_add(Weight::from_ref_time(121_970).saturating_mul(s.into()))
			// Standard Error: 70
			.saturating_add(Weight::from_ref_time(1_865).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	/// The range of component `s` is `[3, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_approve(s: u32, z: u32, ) -> Weight {
		// Minimum execution time: 39_694 nanoseconds.
		Weight::from_ref_time(29_291_452)
			// Standard Error: 1_950
			.saturating_add(Weight::from_ref_time(122_175).saturating_mul(s.into()))
			// Standard Error: 19
			.saturating_add(Weight::from_ref_time(1_834).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_complete(s: u32, z: u32, ) -> Weight {
		// Minimum execution time: 55_881 nanoseconds.
		Weight::from_ref_time(43_628_191)
			// Standard Error: 2_030
			.saturating_add(Weight::from_ref_time(144_339).saturating_mul(s.into()))
			// Standard Error: 19
			.saturating_add(Weight::from_ref_time(1_774).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_create(s: u32, ) -> Weight {
		// Minimum execution time: 36_420 nanoseconds.
		Weight::from_ref_time(38_721_106)
			// Standard Error: 6_761
			.saturating_add(Weight::from_ref_time(156_765).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_approve(s: u32, ) -> Weight {
		// Minimum execution time: 26_025 nanoseconds.
		Weight::from_ref_time(26_889_251)
			// Standard Error: 1_653
			.saturating_add(Weight::from_ref_time(128_717).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	/// The range of component `s` is `[2, 100]`.
	fn cancel_as_multi(s: u32, ) -> Weight {
		// Minimum execution time: 36_373 nanoseconds.
		Weight::from_ref_time(38_800_022)
			// Standard Error: 2_214
			.saturating_add(Weight::from_ref_time(131_644).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_threshold_1(z: u32, ) -> Weight {
		// Minimum execution time: 17_705 nanoseconds.
		Weight::from_ref_time(18_611_308)
			// Standard Error: 10
			.saturating_add(Weight::from_ref_time(649).saturating_mul(z.into()))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		// Minimum execution time: 52_394 nanoseconds.
		Weight::from_ref_time(42_628_113)
			// Standard Error: 7_162
			.saturating_add(Weight::from_ref_time(121_970).saturating_mul(s.into()))
			// Standard Error: 70
			.saturating_add(Weight::from_ref_time(1_865).saturating_mul(z.into()))
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	/// The range of component `s` is `[3, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_approve(s: u32, z: u32, ) -> Weight {
		// Minimum execution time: 39_694 nanoseconds.
		Weight::from_ref_time(29_291_452)
			// Standard Error: 1_950
			.saturating_add(Weight::from_ref_time(122_175).saturating_mul(s.into()))
			// Standard Error: 19
			.saturating_add(Weight::from_ref_time(1_834).saturating_mul(z.into()))
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_complete(s: u32, z: u32, ) -> Weight {
		// Minimum execution time: 55_881 nanoseconds.
		Weight::from_ref_time(43_628_191)
			// Standard Error: 2_030
			.saturating_add(Weight::from_ref_time(144_339).saturating_mul(s.into()))
			// Standard Error: 19
			.saturating_add(Weight::from_ref_time(1_774).saturating_mul(z.into()))
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_create(s: u32, ) -> Weight {
		// Minimum execution time: 36_420 nanoseconds.
		Weight::from_ref_time(38_721_106)
			// Standard Error: 6_761
			.saturating_add(Weight::from_ref_time(156_765).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_approve(s: u32, ) -> Weight {
		// Minimum execution time: 26_025 nanoseconds.
		Weight::from_ref_time(26_889_251)
			// Standard Error: 1_653
			.saturating_add(Weight::from_ref_time(128_717).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	/// The range of component `s` is `[2, 100]`.
	fn cancel_as_multi(s: u32, ) -> Weight {
		// Minimum execution time: 36_373 nanoseconds.
		Weight::from_ref_time(38_800_022)
			// Standard Error: 2_214
			.saturating_add(Weight::from_ref_time(131_644).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
}
