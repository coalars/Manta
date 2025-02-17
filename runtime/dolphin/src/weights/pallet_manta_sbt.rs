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

//! Autogenerated weights for pallet_manta_sbt
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
// --pallet=pallet_manta_sbt
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./scripts/benchmarking/frame-weights-output/pallet_manta_sbt.rs
// --template=.github/resources/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;
use manta_primitives::constants::RocksDbWeight;

/// Weight functions needed for pallet_manta_sbt.
pub trait WeightInfo {
    fn to_private() -> Weight;
    fn reserve_sbt() -> Weight;
    fn change_allowlist_account() -> Weight;
    fn allowlist_evm_account() -> Weight;
    fn set_mint_chain_info() -> Weight;
    fn mint_sbt_eth() -> Weight;
}

/// Weights for pallet_manta_sbt using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_manta_sbt::WeightInfo for SubstrateWeight<T> {
	// Storage: MantaSbt ReservedIds (r:1 w:1)
	// Storage: MantaSbt UtxoSet (r:1 w:1)
	// Storage: MantaSbt ShardTrees (r:1 w:1)
	// Storage: MantaSbt UtxoAccumulatorOutputs (r:0 w:1)
	// Storage: MantaSbt SbtMetadata (r:0 w:1)
	// Storage: MantaSbt Shards (r:0 w:1)
	fn to_private() -> Weight {
		// Minimum execution time: 39_306_220 nanoseconds.
		Weight::from_ref_time(39_327_198_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: MantaSbt NextSbtId (r:1 w:1)
	// Storage: MantaSbt ReservedIds (r:0 w:1)
	fn reserve_sbt() -> Weight {
		// Minimum execution time: 96_792 nanoseconds.
		Weight::from_ref_time(98_693_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: MantaSbt AllowlistAccount (r:0 w:1)
	fn change_allowlist_account() -> Weight {
		// Minimum execution time: 20_853 nanoseconds.
		Weight::from_ref_time(21_511_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: MantaSbt MintChainInfos (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: MantaSbt AllowlistAccount (r:1 w:0)
	// Storage: MantaSbt EvmAddressAllowlist (r:1 w:1)
	// Storage: MantaSbt NextSbtId (r:1 w:1)
	fn allowlist_evm_account() -> Weight {
		// Minimum execution time: 29_502 nanoseconds.
		Weight::from_ref_time(30_448_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: MantaSbt MintChainInfos (r:0 w:1)
	fn set_mint_chain_info() -> Weight {
		// Minimum execution time: 16_745 nanoseconds.
		Weight::from_ref_time(17_237_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: MantaSbt MintChainInfos (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System BlockHash (r:1 w:0)
	// Storage: MantaSbt EvmAddressAllowlist (r:1 w:1)
	// Storage: MantaSbt UtxoSet (r:1 w:1)
	// Storage: MantaSbt ShardTrees (r:1 w:1)
	// Storage: MantaSbt UtxoAccumulatorOutputs (r:0 w:1)
	// Storage: MantaSbt SbtMetadata (r:0 w:1)
	// Storage: MantaSbt Shards (r:0 w:1)
	fn mint_sbt_eth() -> Weight {
		// Minimum execution time: 39_373_883 nanoseconds.
		Weight::from_ref_time(39_416_396_000)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: MantaSbt ReservedIds (r:1 w:1)
	// Storage: MantaSbt UtxoSet (r:1 w:1)
	// Storage: MantaSbt ShardTrees (r:1 w:1)
	// Storage: MantaSbt UtxoAccumulatorOutputs (r:0 w:1)
	// Storage: MantaSbt SbtMetadata (r:0 w:1)
	// Storage: MantaSbt Shards (r:0 w:1)
	fn to_private() -> Weight {
		// Minimum execution time: 39_306_220 nanoseconds.
		Weight::from_ref_time(39_327_198_000)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(6))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: MantaSbt NextSbtId (r:1 w:1)
	// Storage: MantaSbt ReservedIds (r:0 w:1)
	fn reserve_sbt() -> Weight {
		// Minimum execution time: 96_792 nanoseconds.
		Weight::from_ref_time(98_693_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: MantaSbt AllowlistAccount (r:0 w:1)
	fn change_allowlist_account() -> Weight {
		// Minimum execution time: 20_853 nanoseconds.
		Weight::from_ref_time(21_511_000)
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: MantaSbt MintChainInfos (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: MantaSbt AllowlistAccount (r:1 w:0)
	// Storage: MantaSbt EvmAddressAllowlist (r:1 w:1)
	// Storage: MantaSbt NextSbtId (r:1 w:1)
	fn allowlist_evm_account() -> Weight {
		// Minimum execution time: 29_502 nanoseconds.
		Weight::from_ref_time(30_448_000)
			.saturating_add(RocksDbWeight::get().reads(5))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
	// Storage: MantaSbt MintChainInfos (r:0 w:1)
	fn set_mint_chain_info() -> Weight {
		// Minimum execution time: 16_745 nanoseconds.
		Weight::from_ref_time(17_237_000)
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: MantaSbt MintChainInfos (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: System BlockHash (r:1 w:0)
	// Storage: MantaSbt EvmAddressAllowlist (r:1 w:1)
	// Storage: MantaSbt UtxoSet (r:1 w:1)
	// Storage: MantaSbt ShardTrees (r:1 w:1)
	// Storage: MantaSbt UtxoAccumulatorOutputs (r:0 w:1)
	// Storage: MantaSbt SbtMetadata (r:0 w:1)
	// Storage: MantaSbt Shards (r:0 w:1)
	fn mint_sbt_eth() -> Weight {
		// Minimum execution time: 39_373_883 nanoseconds.
		Weight::from_ref_time(39_416_396_000)
			.saturating_add(RocksDbWeight::get().reads(6))
			.saturating_add(RocksDbWeight::get().writes(6))
	}
}
