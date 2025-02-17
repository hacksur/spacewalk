
//! Autogenerated weights for nomination
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-26, STEPS: `100`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `MacBook-Pro`, CPU: `<UNKNOWN>`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/spacewalk-standalone
// benchmark
// pallet
// --chain=dev
// --pallet=nomination
// --extrinsic=*
// --steps=100
// --repeat=10
// --wasm-execution=compiled
// --output=pallets/nomination/src/default_weights.rs
// --template=.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for nomination.
pub trait WeightInfo {
	fn set_nomination_enabled() -> Weight;
	fn opt_in_to_nomination() -> Weight;
	fn opt_out_of_nomination() -> Weight;
	fn deposit_collateral() -> Weight;
	fn withdraw_collateral() -> Weight;
}

/// Weights for nomination using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Nomination NominationEnabled (r:0 w:1)
	fn set_nomination_enabled() -> Weight {
		// Minimum execution time: 3_000 nanoseconds.
		Weight::from_ref_time(3_000_000u64)
			.saturating_add(T::DbWeight::get().writes(1u64))
	}
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: Nomination NominationEnabled (r:1 w:0)
	// Storage: VaultRegistry Vaults (r:1 w:0)
	// Storage: Nomination Vaults (r:1 w:1)
	fn opt_in_to_nomination() -> Weight {
		// Minimum execution time: 18_000 nanoseconds.
		Weight::from_ref_time(19_000_000u64)
			.saturating_add(T::DbWeight::get().reads(4u64))
			.saturating_add(T::DbWeight::get().writes(1u64))
	}
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: Nomination Vaults (r:1 w:1)
	// Storage: VaultStaking Nonce (r:1 w:1)
	// Storage: VaultStaking TotalCurrentStake (r:2 w:2)
	// Storage: VaultStaking Stake (r:2 w:2)
	// Storage: VaultStaking SlashPerToken (r:2 w:0)
	// Storage: VaultStaking SlashTally (r:2 w:2)
	// Storage: VaultRegistry Vaults (r:1 w:0)
	// Storage: VaultRegistry SecureCollateralThreshold (r:1 w:0)
	// Storage: VaultStaking TotalStake (r:2 w:2)
	// Storage: VaultStaking RewardTally (r:4 w:4)
	// Storage: VaultStaking RewardPerToken (r:4 w:0)
	// Storage: VaultRegistry TotalUserVaultCollateral (r:1 w:1)
	fn opt_out_of_nomination() -> Weight {
		// Minimum execution time: 90_000 nanoseconds.
		Weight::from_ref_time(93_000_000u64)
			.saturating_add(T::DbWeight::get().reads(24u64))
			.saturating_add(T::DbWeight::get().writes(15u64))
	}
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: Nomination NominationEnabled (r:1 w:0)
	// Storage: Nomination Vaults (r:1 w:0)
	// Storage: VaultStaking Nonce (r:1 w:0)
	// Storage: VaultStaking TotalCurrentStake (r:1 w:1)
	// Storage: VaultStaking Stake (r:2 w:1)
	// Storage: VaultStaking SlashPerToken (r:1 w:0)
	// Storage: VaultStaking SlashTally (r:2 w:1)
	// Storage: VaultRegistry SecureCollateralThreshold (r:1 w:0)
	// Storage: VaultRegistry PremiumRedeemThreshold (r:1 w:0)
	// Storage: VaultRewards Stake (r:1 w:0)
	// Storage: VaultRewards RewardPerToken (r:2 w:0)
	// Storage: VaultRewards RewardTally (r:2 w:2)
	// Storage: VaultRewards TotalRewards (r:2 w:2)
	// Storage: VaultStaking RewardPerToken (r:2 w:2)
	// Storage: VaultStaking TotalStake (r:1 w:1)
	// Storage: VaultStaking RewardTally (r:2 w:2)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:0)
	// Storage: VaultRegistry TotalUserVaultCollateral (r:1 w:1)
	// Storage: VaultRegistry SystemCollateralCeiling (r:1 w:0)
	fn deposit_collateral() -> Weight {
		// Minimum execution time: 101_000 nanoseconds.
		Weight::from_ref_time(104_000_000u64)
			.saturating_add(T::DbWeight::get().reads(29u64))
			.saturating_add(T::DbWeight::get().writes(15u64))
	}
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: VaultStaking Nonce (r:1 w:0)
	// Storage: VaultRegistry Vaults (r:1 w:0)
	// Storage: VaultStaking TotalCurrentStake (r:1 w:1)
	// Storage: VaultRegistry SecureCollateralThreshold (r:1 w:0)
	// Storage: Nomination NominationEnabled (r:1 w:0)
	// Storage: Nomination Vaults (r:1 w:0)
	// Storage: VaultRegistry TotalUserVaultCollateral (r:1 w:1)
	// Storage: VaultRewards Stake (r:1 w:0)
	// Storage: VaultRewards RewardPerToken (r:2 w:0)
	// Storage: VaultRewards RewardTally (r:2 w:2)
	// Storage: VaultRewards TotalRewards (r:2 w:2)
	// Storage: VaultStaking RewardPerToken (r:2 w:2)
	// Storage: VaultStaking Stake (r:1 w:1)
	// Storage: VaultStaking SlashPerToken (r:1 w:0)
	// Storage: VaultStaking SlashTally (r:1 w:1)
	// Storage: VaultStaking TotalStake (r:1 w:1)
	// Storage: VaultStaking RewardTally (r:2 w:2)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:0)
	fn withdraw_collateral() -> Weight {
		// Minimum execution time: 110_000 nanoseconds.
		Weight::from_ref_time(111_000_000u64)
			.saturating_add(T::DbWeight::get().reads(26u64))
			.saturating_add(T::DbWeight::get().writes(15u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Nomination NominationEnabled (r:0 w:1)
	fn set_nomination_enabled() -> Weight {
		// Minimum execution time: 3_000 nanoseconds.
		Weight::from_ref_time(3_000_000u64)
			.saturating_add(RocksDbWeight::get().writes(1u64))
	}
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: Nomination NominationEnabled (r:1 w:0)
	// Storage: VaultRegistry Vaults (r:1 w:0)
	// Storage: Nomination Vaults (r:1 w:1)
	fn opt_in_to_nomination() -> Weight {
		// Minimum execution time: 18_000 nanoseconds.
		Weight::from_ref_time(19_000_000u64)
			.saturating_add(RocksDbWeight::get().reads(4u64))
			.saturating_add(RocksDbWeight::get().writes(1u64))
	}
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: Nomination Vaults (r:1 w:1)
	// Storage: VaultStaking Nonce (r:1 w:1)
	// Storage: VaultStaking TotalCurrentStake (r:2 w:2)
	// Storage: VaultStaking Stake (r:2 w:2)
	// Storage: VaultStaking SlashPerToken (r:2 w:0)
	// Storage: VaultStaking SlashTally (r:2 w:2)
	// Storage: VaultRegistry Vaults (r:1 w:0)
	// Storage: VaultRegistry SecureCollateralThreshold (r:1 w:0)
	// Storage: VaultStaking TotalStake (r:2 w:2)
	// Storage: VaultStaking RewardTally (r:4 w:4)
	// Storage: VaultStaking RewardPerToken (r:4 w:0)
	// Storage: VaultRegistry TotalUserVaultCollateral (r:1 w:1)
	fn opt_out_of_nomination() -> Weight {
		// Minimum execution time: 90_000 nanoseconds.
		Weight::from_ref_time(93_000_000u64)
			.saturating_add(RocksDbWeight::get().reads(24u64))
			.saturating_add(RocksDbWeight::get().writes(15u64))
	}
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: Nomination NominationEnabled (r:1 w:0)
	// Storage: Nomination Vaults (r:1 w:0)
	// Storage: VaultStaking Nonce (r:1 w:0)
	// Storage: VaultStaking TotalCurrentStake (r:1 w:1)
	// Storage: VaultStaking Stake (r:2 w:1)
	// Storage: VaultStaking SlashPerToken (r:1 w:0)
	// Storage: VaultStaking SlashTally (r:2 w:1)
	// Storage: VaultRegistry SecureCollateralThreshold (r:1 w:0)
	// Storage: VaultRegistry PremiumRedeemThreshold (r:1 w:0)
	// Storage: VaultRewards Stake (r:1 w:0)
	// Storage: VaultRewards RewardPerToken (r:2 w:0)
	// Storage: VaultRewards RewardTally (r:2 w:2)
	// Storage: VaultRewards TotalRewards (r:2 w:2)
	// Storage: VaultStaking RewardPerToken (r:2 w:2)
	// Storage: VaultStaking TotalStake (r:1 w:1)
	// Storage: VaultStaking RewardTally (r:2 w:2)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:0)
	// Storage: VaultRegistry TotalUserVaultCollateral (r:1 w:1)
	// Storage: VaultRegistry SystemCollateralCeiling (r:1 w:0)
	fn deposit_collateral() -> Weight {
		// Minimum execution time: 101_000 nanoseconds.
		Weight::from_ref_time(104_000_000u64)
			.saturating_add(RocksDbWeight::get().reads(29u64))
			.saturating_add(RocksDbWeight::get().writes(15u64))
	}
	// Storage: Security ParachainStatus (r:1 w:0)
	// Storage: VaultStaking Nonce (r:1 w:0)
	// Storage: VaultRegistry Vaults (r:1 w:0)
	// Storage: VaultStaking TotalCurrentStake (r:1 w:1)
	// Storage: VaultRegistry SecureCollateralThreshold (r:1 w:0)
	// Storage: Nomination NominationEnabled (r:1 w:0)
	// Storage: Nomination Vaults (r:1 w:0)
	// Storage: VaultRegistry TotalUserVaultCollateral (r:1 w:1)
	// Storage: VaultRewards Stake (r:1 w:0)
	// Storage: VaultRewards RewardPerToken (r:2 w:0)
	// Storage: VaultRewards RewardTally (r:2 w:2)
	// Storage: VaultRewards TotalRewards (r:2 w:2)
	// Storage: VaultStaking RewardPerToken (r:2 w:2)
	// Storage: VaultStaking Stake (r:1 w:1)
	// Storage: VaultStaking SlashPerToken (r:1 w:0)
	// Storage: VaultStaking SlashTally (r:1 w:1)
	// Storage: VaultStaking TotalStake (r:1 w:1)
	// Storage: VaultStaking RewardTally (r:2 w:2)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:0)
	fn withdraw_collateral() -> Weight {
		// Minimum execution time: 110_000 nanoseconds.
		Weight::from_ref_time(111_000_000u64)
			.saturating_add(RocksDbWeight::get().reads(26u64))
			.saturating_add(RocksDbWeight::get().writes(15u64))
	}
}