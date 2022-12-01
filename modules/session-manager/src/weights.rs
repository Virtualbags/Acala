// This file is part of Acala.

// Copyright (C) 2020-2022 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for module_session_manager
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-07-12, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/acala
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=module_session_manager
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./modules/session-manager/src/weights.rs
// --template=./templates/module-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for module_session_manager.
pub trait WeightInfo {
	fn schedule_session_duration() -> Weight;
	fn on_initialize_skip() -> Weight;
	fn on_initialize() -> Weight;
	fn estimate_current_session_progress() -> Weight;
	fn estimate_next_session_rotation() -> Weight;
}

/// Weights for module_session_manager using the Acala node and recommended hardware.
pub struct AcalaWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AcalaWeight<T> {
	fn schedule_session_duration() -> Weight {
		Weight::from_ref_time(32_968_000)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn on_initialize_skip() -> Weight {
		Weight::from_ref_time(5_399_000)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn on_initialize() -> Weight {
		Weight::from_ref_time(8_030_000)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	fn estimate_current_session_progress() -> Weight {
		Weight::from_ref_time(6_449_000)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
	}
	fn estimate_next_session_rotation() -> Weight {
		Weight::from_ref_time(6_530_000)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn schedule_session_duration() -> Weight {
		Weight::from_ref_time(32_968_000)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn on_initialize_skip() -> Weight {
		Weight::from_ref_time(5_399_000)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn on_initialize() -> Weight {
		Weight::from_ref_time(8_030_000)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	fn estimate_current_session_progress() -> Weight {
		Weight::from_ref_time(6_449_000)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
	}
	fn estimate_next_session_rotation() -> Weight {
		Weight::from_ref_time(6_530_000)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
	}
}
