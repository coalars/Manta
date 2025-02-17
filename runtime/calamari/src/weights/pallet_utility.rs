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

//! Autogenerated weights for pallet_utility
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("calamari-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/manta
// benchmark
// pallet
// --chain=calamari-dev
// --steps=50
// --repeat=20
// --pallet=pallet_utility
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./scripts/benchmarking/frame-weights-output/pallet_utility.rs
// --template=.github/resources/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;
use manta_primitives::constants::RocksDbWeight;

/// Weight functions needed for pallet_utility.
pub trait WeightInfo {
    fn batch(c: u32, ) -> Weight;
    fn as_derivative() -> Weight;
    fn batch_all(c: u32, ) -> Weight;
    fn dispatch_as() -> Weight;
    fn force_batch(c: u32, ) -> Weight;
}

/// Weights for pallet_utility using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for SubstrateWeight<T> {
	/// The range of component `c` is `[0, 1000]`.
	fn batch(c: u32, ) -> Weight {
		// Minimum execution time: 12_646 nanoseconds.
		Weight::from_ref_time(44_551_907)
			// Standard Error: 6_853
			.saturating_add(Weight::from_ref_time(4_504_303).saturating_mul(c.into()))
	}
	fn as_derivative() -> Weight {
		// Minimum execution time: 7_642 nanoseconds.
		Weight::from_ref_time(7_863_000)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn batch_all(c: u32, ) -> Weight {
		// Minimum execution time: 14_100 nanoseconds.
		Weight::from_ref_time(56_552_464)
			// Standard Error: 4_706
			.saturating_add(Weight::from_ref_time(4_794_210).saturating_mul(c.into()))
	}
	fn dispatch_as() -> Weight {
		// Minimum execution time: 16_253 nanoseconds.
		Weight::from_ref_time(16_745_000)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn force_batch(c: u32, ) -> Weight {
		// Minimum execution time: 14_094 nanoseconds.
		Weight::from_ref_time(44_345_993)
			// Standard Error: 4_888
			.saturating_add(Weight::from_ref_time(4_635_190).saturating_mul(c.into()))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// The range of component `c` is `[0, 1000]`.
	fn batch(c: u32, ) -> Weight {
		// Minimum execution time: 12_646 nanoseconds.
		Weight::from_ref_time(44_551_907)
			// Standard Error: 6_853
			.saturating_add(Weight::from_ref_time(4_504_303).saturating_mul(c.into()))
	}
	fn as_derivative() -> Weight {
		// Minimum execution time: 7_642 nanoseconds.
		Weight::from_ref_time(7_863_000)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn batch_all(c: u32, ) -> Weight {
		// Minimum execution time: 14_100 nanoseconds.
		Weight::from_ref_time(56_552_464)
			// Standard Error: 4_706
			.saturating_add(Weight::from_ref_time(4_794_210).saturating_mul(c.into()))
	}
	fn dispatch_as() -> Weight {
		// Minimum execution time: 16_253 nanoseconds.
		Weight::from_ref_time(16_745_000)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn force_batch(c: u32, ) -> Weight {
		// Minimum execution time: 14_094 nanoseconds.
		Weight::from_ref_time(44_345_993)
			// Standard Error: 4_888
			.saturating_add(Weight::from_ref_time(4_635_190).saturating_mul(c.into()))
	}
}
