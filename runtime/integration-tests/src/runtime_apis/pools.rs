// Copyright 2021 Centrifuge Foundation (centrifuge.io).
//
// This file is part of the Centrifuge chain project.
// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).
// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

use cfg_primitives::{AccountId, Moment, PoolId};
use cfg_traits::TrancheCurrency as _;
use cfg_types::{
	permissions::{PermissionScope, PermissionScope::Currency, PoolRole, Role},
	tokens::{CurrencyId, TrancheCurrency},
};
use cfg_utils::set_block_number_timestamp;
use codec::Encode;
use development_runtime::apis::PoolsApi;
use frame_support::{
	assert_ok,
	dispatch::UnfilteredDispatchable,
	traits::{OnInitialize, UnixTime},
};
use frame_system::Origin;
use fudge::primitives::Chain;
use pallet_loans::math;
use sp_core::Pair;
use sp_runtime::{app_crypto::sr25519, traits::IdentifyAccount};
use tokio::runtime::Handle;

use super::{ApiEnv, PARA_ID};
use crate::{
	chain,
	chain::{
		centrifuge,
		centrifuge::{Runtime, RuntimeOrigin},
	},
	pools::utils::{
		accounts::Keyring,
		env::{test_env_default, TestEnv},
		loans::{borrow_call, init_loans_for_pool, issue_default_loan, LoanId, NftManager},
		pools::default_pool_calls,
		time::secs::{SECONDS_PER_DAY, SECONDS_PER_YEAR},
		tokens::DECIMAL_BASE_12,
	},
};

const POOL_ID: PoolId = 1;

#[tokio::test]
async fn test() {
	ApiEnv::new(Handle::current())
		.startup(|| {
			let pool_admin: AccountId = Keyring::Alice.into();
			let borrower: AccountId = Keyring::Bob.into();

			let mut nft_manager = NftManager::new();
			let set_default_pool =
				default_pool_calls(pool_admin.clone(), POOL_ID, &mut nft_manager);
			let issue_default_loans = issue_default_loan(
				pool_admin.clone(),
				POOL_ID,
				100_000 * DECIMAL_BASE_12,
				2 * SECONDS_PER_YEAR,
				&mut nft_manager,
			);

			for call in set_default_pool {
				let res = UnfilteredDispatchable::dispatch_bypass_filter(
					call,
					RuntimeOrigin::signed(pool_admin.clone()),
				);
				assert_ok!(res);
			}

			// set timestamp to around 1 week and update interest accrual
			let now = development_runtime::Timestamp::now();
			let after_one_year = now + 7 * SECONDS_PER_DAY;
			pallet_timestamp::Now::<Runtime>::set(after_one_year.into());
			centrifuge::InterestAccrual::on_initialize(0);

			// TODO: This should be a utility function. All of the above, with just invest(account, pool, tranche_index, amount)
			//       AS with the others the utility should generate calls than can be submitted
			//
			//       NOTE: The UnfilteredDispatchable::dispatch_bypass_filter(...) Should also be a method somewhere in utils
			let tranche_id = {
				let pool = pallet_pool_system::Pool::<Runtime>::get(POOL_ID).unwrap();
				pool.tranches.ids_residual_top().get(0).unwrap().clone()
			};
			for id in 1..11 {
				centrifuge::Investments::update_invest_order(
					RuntimeOrigin::signed(Keyring::TrancheInvestor(id).into()),
					TrancheCurrency::generate(POOL_ID, tranche_id),
					1_000 * DECIMAL_BASE_12,
				)
				.unwrap()
			}
			centrifuge::Loans::update_nav(RuntimeOrigin::signed(pool_admin.clone()), POOL_ID)
				.unwrap();
			centrifuge::PoolSystem::close_epoch(RuntimeOrigin::signed(pool_admin.clone()), POOL_ID)
				.unwrap();
			// TODO: Utility till here

			for call in issue_default_loans {
				let res = UnfilteredDispatchable::dispatch_bypass_filter(
					call,
					RuntimeOrigin::signed(pool_admin.clone()),
				);
				assert_ok!(res);
			}

			let borrow_call = UnfilteredDispatchable::dispatch_bypass_filter(
				borrow_call(POOL_ID, LoanId::from(1_u16), 10_000 * DECIMAL_BASE_12),
				RuntimeOrigin::signed(pool_admin.clone()),
			);

			assert_ok!(borrow_call);

			// set timestamp to around 1 year
			let now = development_runtime::Timestamp::now();
			let after_one_year = now + SECONDS_PER_YEAR;
			pallet_timestamp::Now::<Runtime>::set(after_one_year.into());

			// let max_borrow_amount = development_runtime::Loans::get_max_borrow_amount(0, LoanId::from(0_u16));
			// assert_eq!(max_borrow_amount, Ok(0));
		})
		.with_api(|api, latest| {
			let valuation = api.portfolio_valuation(&latest, POOL_ID).unwrap();
			assert_eq!(valuation, Some(11836407882999939));

			// None existing loan is None
			let max_borrow_amount = api
				.max_borrow_amount(&latest, POOL_ID, LoanId::from(0_u16))
				.unwrap();
			assert_eq!(max_borrow_amount, None);

			// Existing and borrowed loan has Some()
			let max_borrow_amount = api
				.max_borrow_amount(&latest, POOL_ID, LoanId::from(1_u16))
				.unwrap();
			assert_eq!(max_borrow_amount, Some(80_000 * DECIMAL_BASE_12));
		});
}
