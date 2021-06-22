// Copyright 2021 Centrifuge GmbH (centrifuge.io).
// This file is part of Centrifuge chain project.

// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).

// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// # Crowdloan reward pallet's transactions benchmarking
#![cfg(feature = "runtime-benchmarks")]

use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use sp_runtime::Perbill;

use crate::*;

benchmarks! {
    initialize {
        let conversion_rate = 100;
        let direct_payout_ratio = Perbill::from_percent(20);
        let vesting_period = 200,
        let vesting_start = 100,
    }: initialize(RawOrigin::Root, conversion_rate, direct_payout_ratio, vesting_period, vesting_start)
    verify {
        assert_eq!(CrowdloanReward::conversion_rate(), 100);
        assert_eq!(CrowdloanReward::direct_payout_ratio(), Perbill::from_percent(20));
        assert_eq!(CrowdloanReward::vesting_period(), Some(200));
        assert_eq!(CrowdloanReward::vesting_start(), Some(100));
    };

    set_vesting_start {
        let start = 50;
    }: set_vesting_start(RawOrigin::Root, start)
    verify {
        assert_eq!(CrowdloanReward::vesting_start(), Some(50));
    };

    set_vesting_period {
        let period = 50;
    }: set_vesting_period(RawOrigin::Root, period)
    verify {
        assert_eq!(CrowdloanReward::vesting_period(), Some(50));
    };

    set_conversion_rate {
        let rate = 50;
    }: set_conversion_rate(RawOrigin::Root, rate)
    verify {
        assert_eq!(CrowdloanReward::conversion_rate(), 50);
    }

    set_direct_payout_ratio {
        let ratio = 50;
    }: set_direct_payout_ratio(RawOrigin::Root, ratio)
    verify {
        assert_eq!(CrowdloanReward::direct_payout_ratio(), Perbill::from_percent(50));
    }

    reward {
        let rew_balance = Balances::free_balance(&4);
        let who = 2;
        let contribution = 50;
    }: reward(who, contribution)
    verify {
        assert_eq!(Vesting::vesting_balance(&4), Some(80));
        assert_eq!(Balances::usable_balance(&4), rew_balance + 20);
    }
}

impl_benchmark_test_suite!(
    Pallet,
    crate::tests::build(|| {}),
    crate::tests::MockRuntime,
);
