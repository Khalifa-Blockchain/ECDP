
// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
//
// This file is part of Ethical DeFi.
//
// Copyright (C) 2019-Present Setheum Labs.
// SPDX-License-Identifier: BUSL-1.1 (Business Source License 1.1)

//! Autogenerated weights for cdp_treasury
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-02-26, STEPS: [50, ], REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/setheum-node
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=cdp_treasury
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./lib-serml/cdp-treasury/src/weights.rs
// --template=./templates/module-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for cdp_treasury.
pub trait WeightInfo {
	fn extract_surplus_to_serp() -> Weight;
	fn auction_collateral(b: u32) -> Weight;
	fn exchange_collateral_to_stable() -> Weight;
	fn set_expected_collateral_auction_size() -> Weight;
}

/// Weights for cdp_treasury using the Setheum node and recommended hardware.
pub struct SetheumWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SetheumWeight<T> {
	fn extract_surplus_to_serp() -> Weight {
		(124_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn auction_collateral(b: u32, ) -> Weight {
		(2_672_000 as Weight)
			// Standard Error: 326_000
			.saturating_add((32_334_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(b as Weight)))
	}
	fn exchange_collateral_to_stable() -> Weight {
		(176_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	fn set_expected_collateral_auction_size() -> Weight {
		(25_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn extract_surplus_to_serp() -> Weight {
		(124_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn auction_collateral(b: u32, ) -> Weight {
		(2_672_000 as Weight)
			.saturating_add((32_334_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes((3 as Weight).saturating_mul(b as Weight)))
	}
	fn exchange_collateral_to_stable() -> Weight {
		(176_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(9 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	fn set_expected_collateral_auction_size() -> Weight {
		(25_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
