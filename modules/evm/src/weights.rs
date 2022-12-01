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

//! Autogenerated weights for module_evm
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-28, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bd19e103a8cb`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/acala
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=module_evm
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./modules/evm/src/weights.rs
// --template=./templates/module-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for module_evm.
pub trait WeightInfo {
	fn create() -> Weight;
	fn create2() -> Weight;
	fn create_nft_contract() -> Weight;
	fn create_predeploy_contract() -> Weight;
	fn call() -> Weight;
	fn transfer_maintainer() -> Weight;
	fn publish_contract() -> Weight;
	fn publish_free() -> Weight;
	fn enable_contract_development() -> Weight;
	fn disable_contract_development() -> Weight;
	fn set_code(c: u32, ) -> Weight;
	fn selfdestruct() -> Weight;
}

/// Weights for module_evm using the Acala node and recommended hardware.
pub struct AcalaWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AcalaWeight<T> {
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: EvmAccounts Accounts (r:2 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Reserves (r:2 w:2)
	// Storage: EVM Accounts (r:2 w:2)
	// Storage: EVM Codes (r:1 w:1)
	// Storage: EVM CodeInfos (r:1 w:1)
	// Storage: EVM ContractStorageSizes (r:1 w:1)
	fn create() -> Weight {
		// Minimum execution time: 212_993 nanoseconds.
		Weight::from_ref_time(217_192_000)
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: EvmAccounts Accounts (r:2 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Reserves (r:2 w:2)
	// Storage: EVM Accounts (r:2 w:2)
	// Storage: EVM Codes (r:1 w:1)
	// Storage: EVM CodeInfos (r:1 w:1)
	// Storage: EVM ContractStorageSizes (r:1 w:1)
	fn create2() -> Weight {
		// Minimum execution time: 206_178 nanoseconds.
		Weight::from_ref_time(210_604_000)
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: EvmAccounts Accounts (r:2 w:0)
	// Storage: EVM NetworkContractIndex (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Reserves (r:2 w:2)
	// Storage: EVM Accounts (r:2 w:2)
	// Storage: EVM Codes (r:1 w:1)
	// Storage: EVM CodeInfos (r:1 w:1)
	// Storage: EVM ContractStorageSizes (r:1 w:1)
	fn create_nft_contract() -> Weight {
		// Minimum execution time: 236_885 nanoseconds.
		Weight::from_ref_time(246_172_000)
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(10))
	}
	// Storage: EVM Accounts (r:2 w:2)
	// Storage: EvmAccounts Accounts (r:2 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Reserves (r:2 w:2)
	// Storage: EVM Codes (r:1 w:1)
	// Storage: EVM CodeInfos (r:1 w:1)
	// Storage: EVM ContractStorageSizes (r:1 w:1)
	fn create_predeploy_contract() -> Weight {
		// Minimum execution time: 241_100 nanoseconds.
		Weight::from_ref_time(247_057_000)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: EVM Accounts (r:2 w:1)
	// Storage: EvmAccounts Accounts (r:2 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Reserves (r:2 w:2)
	// Storage: EVM Codes (r:1 w:0)
	// Storage: EVM ContractStorageSizes (r:1 w:1)
	fn call() -> Weight {
		// Minimum execution time: 189_010 nanoseconds.
		Weight::from_ref_time(193_110_000)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: EVM Accounts (r:1 w:1)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	fn transfer_maintainer() -> Weight {
		// Minimum execution time: 119_154 nanoseconds.
		Weight::from_ref_time(121_664_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: EVM Accounts (r:1 w:1)
	fn publish_contract() -> Weight {
		// Minimum execution time: 149_378 nanoseconds.
		Weight::from_ref_time(152_344_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: EVM Accounts (r:1 w:1)
	fn publish_free() -> Weight {
		// Minimum execution time: 37_552 nanoseconds.
		Weight::from_ref_time(39_235_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Balances Reserves (r:1 w:1)
	fn enable_contract_development() -> Weight {
		// Minimum execution time: 127_031 nanoseconds.
		Weight::from_ref_time(128_982_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Balances Reserves (r:1 w:1)
	fn disable_contract_development() -> Weight {
		// Minimum execution time: 130_008 nanoseconds.
		Weight::from_ref_time(131_580_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: EVM Accounts (r:1 w:1)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: EVM CodeInfos (r:2 w:2)
	// Storage: EvmAccounts Accounts (r:2 w:0)
	// Storage: Balances Reserves (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: EVM ContractStorageSizes (r:1 w:1)
	// Storage: EVM Codes (r:0 w:2)
	/// The range of component `c` is `[0, 61440]`.
	fn set_code(c: u32, ) -> Weight {
		// Minimum execution time: 225_636 nanoseconds.
		Weight::from_ref_time(224_505_124)
			// Standard Error: 19
			.saturating_add(Weight::from_ref_time(5_794).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: EVM Accounts (r:1 w:1)
	// Storage: EvmAccounts Accounts (r:1 w:0)
	// Storage: EVM CodeInfos (r:1 w:1)
	// Storage: EVM ContractStorageSizes (r:1 w:1)
	// Storage: IdleScheduler NextTaskId (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: IdleScheduler Tasks (r:0 w:1)
	// Storage: EVM Codes (r:0 w:1)
	fn selfdestruct() -> Weight {
		// Minimum execution time: 157_494 nanoseconds.
		Weight::from_ref_time(160_658_000)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(7))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: EvmAccounts Accounts (r:2 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Reserves (r:2 w:2)
	// Storage: EVM Accounts (r:2 w:2)
	// Storage: EVM Codes (r:1 w:1)
	// Storage: EVM CodeInfos (r:1 w:1)
	// Storage: EVM ContractStorageSizes (r:1 w:1)
	fn create() -> Weight {
		// Minimum execution time: 212_993 nanoseconds.
		Weight::from_ref_time(217_192_000)
			.saturating_add(RocksDbWeight::get().reads(12))
			.saturating_add(RocksDbWeight::get().writes(9))
	}
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: EvmAccounts Accounts (r:2 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Reserves (r:2 w:2)
	// Storage: EVM Accounts (r:2 w:2)
	// Storage: EVM Codes (r:1 w:1)
	// Storage: EVM CodeInfos (r:1 w:1)
	// Storage: EVM ContractStorageSizes (r:1 w:1)
	fn create2() -> Weight {
		// Minimum execution time: 206_178 nanoseconds.
		Weight::from_ref_time(210_604_000)
			.saturating_add(RocksDbWeight::get().reads(12))
			.saturating_add(RocksDbWeight::get().writes(9))
	}
	// Storage: EvmAccounts Accounts (r:2 w:0)
	// Storage: EVM NetworkContractIndex (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Reserves (r:2 w:2)
	// Storage: EVM Accounts (r:2 w:2)
	// Storage: EVM Codes (r:1 w:1)
	// Storage: EVM CodeInfos (r:1 w:1)
	// Storage: EVM ContractStorageSizes (r:1 w:1)
	fn create_nft_contract() -> Weight {
		// Minimum execution time: 236_885 nanoseconds.
		Weight::from_ref_time(246_172_000)
			.saturating_add(RocksDbWeight::get().reads(12))
			.saturating_add(RocksDbWeight::get().writes(10))
	}
	// Storage: EVM Accounts (r:2 w:2)
	// Storage: EvmAccounts Accounts (r:2 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Reserves (r:2 w:2)
	// Storage: EVM Codes (r:1 w:1)
	// Storage: EVM CodeInfos (r:1 w:1)
	// Storage: EVM ContractStorageSizes (r:1 w:1)
	fn create_predeploy_contract() -> Weight {
		// Minimum execution time: 241_100 nanoseconds.
		Weight::from_ref_time(247_057_000)
			.saturating_add(RocksDbWeight::get().reads(11))
			.saturating_add(RocksDbWeight::get().writes(9))
	}
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: EVM Accounts (r:2 w:1)
	// Storage: EvmAccounts Accounts (r:2 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Reserves (r:2 w:2)
	// Storage: EVM Codes (r:1 w:0)
	// Storage: EVM ContractStorageSizes (r:1 w:1)
	fn call() -> Weight {
		// Minimum execution time: 189_010 nanoseconds.
		Weight::from_ref_time(193_110_000)
			.saturating_add(RocksDbWeight::get().reads(11))
			.saturating_add(RocksDbWeight::get().writes(6))
	}
	// Storage: EVM Accounts (r:1 w:1)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	fn transfer_maintainer() -> Weight {
		// Minimum execution time: 119_154 nanoseconds.
		Weight::from_ref_time(121_664_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: EVM Accounts (r:1 w:1)
	fn publish_contract() -> Weight {
		// Minimum execution time: 149_378 nanoseconds.
		Weight::from_ref_time(152_344_000)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: EVM Accounts (r:1 w:1)
	fn publish_free() -> Weight {
		// Minimum execution time: 37_552 nanoseconds.
		Weight::from_ref_time(39_235_000)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Balances Reserves (r:1 w:1)
	fn enable_contract_development() -> Weight {
		// Minimum execution time: 127_031 nanoseconds.
		Weight::from_ref_time(128_982_000)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Balances Reserves (r:1 w:1)
	fn disable_contract_development() -> Weight {
		// Minimum execution time: 130_008 nanoseconds.
		Weight::from_ref_time(131_580_000)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: EVM Accounts (r:1 w:1)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: EVM CodeInfos (r:2 w:2)
	// Storage: EvmAccounts Accounts (r:2 w:0)
	// Storage: Balances Reserves (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: EVM ContractStorageSizes (r:1 w:1)
	// Storage: EVM Codes (r:0 w:2)
	/// The range of component `c` is `[0, 61440]`.
	fn set_code(c: u32, ) -> Weight {
		// Minimum execution time: 225_636 nanoseconds.
		Weight::from_ref_time(224_505_124)
			// Standard Error: 19
			.saturating_add(Weight::from_ref_time(5_794).saturating_mul(c.into()))
			.saturating_add(RocksDbWeight::get().reads(10))
			.saturating_add(RocksDbWeight::get().writes(9))
	}
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: EVM Accounts (r:1 w:1)
	// Storage: EvmAccounts Accounts (r:1 w:0)
	// Storage: EVM CodeInfos (r:1 w:1)
	// Storage: EVM ContractStorageSizes (r:1 w:1)
	// Storage: IdleScheduler NextTaskId (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: IdleScheduler Tasks (r:0 w:1)
	// Storage: EVM Codes (r:0 w:1)
	fn selfdestruct() -> Weight {
		// Minimum execution time: 157_494 nanoseconds.
		Weight::from_ref_time(160_658_000)
			.saturating_add(RocksDbWeight::get().reads(7))
			.saturating_add(RocksDbWeight::get().writes(7))
	}
}
