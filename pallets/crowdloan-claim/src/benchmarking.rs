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

//! Crodloan claim pallet's benchmarking.

#![cfg(feature = "runtime-benchmarks")]
use super::*;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use sp_runtime::traits::Hash;

benchmarks! {
  claim_reward {
  }: _(RawOrigin::Root, fee_key, fee)
  verify {
  }

  initialize {
        let start = 50
        let period = 100;
        let contributions_root = H256 = [
            25, 102, 189, 46, 86, 242, 48, 217, 254, 16, 20, 211, 98, 206, 125, 92, 167, 175, 70,
            161, 35, 135, 33, 80, 225, 247, 4, 240, 138, 86, 167, 142,
        ].into();
        let locked_at = 10;
        let trie_index = 0;
  }: _(RawOrigin::Root, contributions_root, locked_at, trie_index, start, period)
  verify {
        let contributions_root = H256 = [
            25, 102, 189, 46, 86, 242, 48, 217, 254, 16, 20, 211, 98, 206, 125, 92, 167, 175, 70,
            161, 35, 135, 33, 80, 225, 247, 4, 240, 138, 86, 167, 142,
        ].into();

        assert_eq!(CrowdloanClaim::lease_start(), 50);
        assert_eq!(CrowdloanClaim::lease_period(), 100);
        assert_eq!(CrowdloanClaim::contributions_root(), contributions_root);
        assert_eq!(CrowdloanClaim::locked_at(), 10);
        assert_eq!(CrowdloanClaim::crowdloan_trie_index(), 0);
  }

  set_lease_start{
        let start = 50;;
  }: _(RawOrigin::Root, start)
  verify {
        assert_eq!(CrowdloanClaim::lease_start(), 50);
  }

  set_lease_period{
        let period = 100;
  }: _(RawOrigin::Root, period)
  verify {
        assert_eq!(CrowdloanClaim::lease_period(), 100);
  }

  set_contributions_root {
        let contributions_root = H256 = [
            25, 102, 189, 46, 86, 242, 48, 217, 254, 16, 20, 211, 98, 206, 125, 92, 167, 175, 70,
            161, 35, 135, 33, 80, 225, 247, 4, 240, 138, 86, 167, 142,
        ].into();
  }: _(RawOrigin::Root, fee_key, fee)
  verify {
        let contributions_root = H256 = [
            25, 102, 189, 46, 86, 242, 48, 217, 254, 16, 20, 211, 98, 206, 125, 92, 167, 175, 70,
            161, 35, 135, 33, 80, 225, 247, 4, 240, 138, 86, 167, 142,
        ].into();

        assert_eq!(CrowdloanClaim::contributions_root(), contributions_root);
  }

  set_locked_at {
        let locked_at = 10;
  }: _(RawOrigin::Root, locked_at)
  verify {
       assert_eq!(CrowdloanClaim::locked_at(), 10);
  }

  set_crowdloan_trie_index {
        let trie_index = 0;
  }: _(RawOrigin::Root, trie_index)
  verify {
        assert_eq!(CrowdloanClaim::crowdloan_trie_index(), 0);
  }
}

impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test,);
