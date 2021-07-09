//! # Migration pallet for runtime
//!
//! This pallet provides functionality for migrating a previous chain-state (possibly from a
//! stand-alone chain) to a new chain state (possbily a parachain now). This pallet is necessary due
//! to the exising boundaries that are put onto runtime upgrades from the relay-chain side.  
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::Currency;

pub use pallet::*;
pub use weights::*;

#[cfg(test)]
mod test_data;

pub mod weights;

#[cfg(test)]
pub mod tests;

#[cfg(test)]
pub mod mock;

pub mod benchmarking;

type BalanceOf<T> = <<T as pallet_vesting::Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use crate::weights::WeightInfo;
	use frame_support::pallet_prelude::*;
	use frame_support::sp_std::convert::TryInto;
	use frame_support::transactional;
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;

	// Import various types used to declare pallet in scope.
	use super::*;
	use frame_support::sp_runtime::traits::Saturating;
	use frame_support::sp_runtime::ArithmeticError;
	use frame_support::traits::VestingSchedule;
	use pallet_proxy::ProxyDefinition;
	use pallet_vesting::VestingInfo;

	pub type NumAccounts = u64;

	// Simple declaration of the `Pallet` type. It is placeholder we use to implement traits and
	// method.
	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config:
		frame_system::Config
		+ pallet_vesting::Config
		+ pallet_balances::Config
		+ pallet_proxy::Config
	{
		/// Maximum number of accounts that can be migrated at once
		#[pallet::constant]
		type MigrationMaxAccounts: Get<u64>;

		/// Maximum number of vestings that can be migrated at once
		#[pallet::constant]
		type MigrationMaxVestings: Get<u64>;

		/// Maximum number of vestings that can be migrated at once
		#[pallet::constant]
		type MigrationMaxProxies: Get<u64>;

		/// Conversion between

		/// Associated type for Event enum
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// WeightInfo
		type WeightInfo: WeightInfo;
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	/// Pallet genesis configuration type declaration.
	///
	/// It allows to build genesis storage.
	#[pallet::genesis_config]
	pub struct GenesisConfig {}

	#[cfg(feature = "std")]
	impl Default for GenesisConfig {
		fn default() -> Self {
			Self {}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Number of accounts that have been migrated
		MigratedSystemAccounts(u64),

		/// Number of vesting that have been migrated
		MigratedVestingAccounts(u64),

		/// Number of vesting that have been migrated
		/// [`OldIssuance`, `NewIssuance`]
		MigratedTotalIssuance(T::Balance, T::Balance),

		/// This is an error that must be dispatched as an Event, as we do not want to fail the whole batch
		/// when one account fails. Should also not happen, as we take them from mainnet. But...
		FailedToMigrateVestingFor(T::AccountId),

		/// Defines the vesting we migrated
		MigratedVestingFor(
			T::AccountId,
			<<T as pallet_vesting::Config>::Currency as frame_support::traits::Currency<
				<T as frame_system::Config>::AccountId,
			>>::Balance,
			<<T as pallet_vesting::Config>::Currency as frame_support::traits::Currency<
				<T as frame_system::Config>::AccountId,
			>>::Balance,
			T::BlockNumber,
		),

		/// Indicates if a migration of proxy data failed, this should NEVER happen, and can only
		/// happen due to insufficient balances during reserve
		FailedToMigrateProxyDataFor(T::AccountId),

		/// Indicates that proxy data has been migrated succesfully for
		/// [`ProxiedAccount`, `DepositOnProxiesAccount`, `NumberOfProxies`]
		MigratedProxyDataFor(
			T::AccountId,
			<<T as pallet_proxy::Config>::Currency as frame_support::traits::Currency<
				<T as frame_system::Config>::AccountId,
			>>::Balance,
			u64,
		),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Too many accounts in the vector for the call of `migrate_system_account`.
		TooManyAccounts,

		/// Too many accounts in the vector for the call of `migrate_system_account`.
		TooManyVestings,

		/// Too many accounts in the vector for the call of `migrate_system_account`.
		TooManyProxies,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Set the given fee for the key
		#[pallet::weight(<T as pallet::Config>::WeightInfo::migrate_system_account(T::MigrationMaxAccounts::get()))]
		#[transactional]
		pub fn migrate_system_account(
			origin: OriginFor<T>,
			accounts: Vec<(Vec<u8>, Vec<u8>)>,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			let num_accounts = accounts.len();
			ensure!(
				accounts.len()
					<= T::MigrationMaxAccounts::get()
						.try_into()
						.map_err(|_| ArithmeticError::Overflow)?,
				Error::<T>::TooManyAccounts
			);

			for (key, value) in accounts {
				storage::unhashed::put_raw(key.as_slice(), value.as_slice());
			}

			// TODO: TryInto
			Self::deposit_event(Event::<T>::MigratedSystemAccounts(num_accounts as u64));

			// TODO: Calculate the actual weight here with the length of the vector being submitted
			Ok(().into())
		}

		/// Calley better be sure, that the total issuance matches the actual total issuance in the system...
		#[pallet::weight(<T as pallet::Config>::WeightInfo::migrate_balances_issuance())]
		#[transactional]
		pub fn migrate_balances_issuance(
			origin: OriginFor<T>,
			additional_issuance: T::Balance,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			let current_issuance = pallet_balances::Pallet::<T>::total_issuance();
			let total_issuance = current_issuance.saturating_add(additional_issuance);

			let key = <pallet_balances::pallet::TotalIssuance<T> as frame_support::storage::generator::StorageValue<T::Balance>>::storage_value_final_key();

			storage::unhashed::put_raw(&key[..], total_issuance.encode().as_slice());

			Self::deposit_event(Event::<T>::MigratedTotalIssuance(
				current_issuance,
				total_issuance,
			));

			Ok(().into())
		}

		#[pallet::weight(<T as pallet::Config>::WeightInfo::migrate_vesting_vesting(T::MigrationMaxVestings::get()))]
		#[transactional]
		pub fn migrate_vesting_vesting(
			origin: OriginFor<T>,
			vestings: Vec<(T::AccountId, VestingInfo<BalanceOf<T>, T::BlockNumber>)>,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			ensure!(
				vestings.len()
					<= T::MigrationMaxVestings::get()
						.try_into()
						.map_err(|_| ArithmeticError::Overflow)?,
				Error::<T>::TooManyVestings
			);

			let mut trying = vestings.len() as u64;

			for (who, schedule) in vestings {
				let _not_care_here = pallet_vesting::Pallet::<T>::add_vesting_schedule(
					&who,
					schedule.locked,
					schedule.per_block,
					schedule.starting_block,
				)
				.map_err(|_| {
					Self::deposit_event(Event::<T>::FailedToMigrateVestingFor(who.clone()));
					trying -= 1;
				})
				.map(|_| {
					Self::deposit_event(Event::<T>::MigratedVestingFor(
						who,
						schedule.locked,
						schedule.per_block,
						schedule.starting_block,
					))
				});
			}

			Self::deposit_event(Event::<T>::MigratedVestingAccounts(trying));
			// TODO: Calculate the actual weight here with the length of the vector being submitted
			Ok(().into())
		}

		#[pallet::weight(<T as pallet::Config>::WeightInfo::migrate_proxy_proxies(T::MigrationMaxProxies::get()))]
		#[transactional]
		pub fn migrate_proxy_proxies(
			origin: OriginFor<T>,
			proxies: Vec<(
				T::AccountId,
				(
					BoundedVec<
						ProxyDefinition<T::AccountId, T::ProxyType, T::BlockNumber>,
						<T as pallet_proxy::Config>::MaxProxies,
					>,
					<<T as pallet_proxy::Config>::Currency as frame_support::traits::Currency<
						<T as frame_system::Config>::AccountId,
					>>::Balance,
				),
			)>,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			ensure!(
				proxies.len()
					<= <T as Config>::MigrationMaxProxies::get()
						.try_into()
						.map_err(|_| ArithmeticError::Overflow)?,
				Error::<T>::TooManyProxies
			);

			let mut trying = proxies.len() as u64;

			for (account_id, (data, deposit)) in proxies {
				let key = <pallet_proxy::pallet::Proxies<T> as frame_support::storage::generator::StorageMap<
					T::AccountId,
					(BoundedVec<
						ProxyDefinition<T::AccountId, T::ProxyType, T::BlockNumber>,
						<T as pallet_proxy::Config>::MaxProxies,
					>,
					 <<T as pallet_proxy::Config>::Currency as frame_support::traits::Currency<<T as frame_system::Config>::AccountId>>::Balance
				)>>::storage_map_final_key(&account_id);

				let _not_care_result = <<T as pallet_proxy::Config>::Currency
					as frame_support::traits::ReservableCurrency<T::AccountId>>::reserve(&account_id, deposit)
						.map_err(|_| {
							Self::deposit_event(Event::<T>::FailedToMigrateProxyDataFor(account_id.clone()));
							trying -= 1;
						})
						.map(|_| {
							Self::deposit_event(Event::<T>::MigratedProxyDataFor(
								account_id,
								deposit,
								data.len() as u64
							));
							()
						});

				storage::unhashed::put_raw(&key[..], (data, deposit).encode().as_slice());
			}

			Ok(().into())
		}
	}
}

impl<T: Config> Pallet<T> {}
