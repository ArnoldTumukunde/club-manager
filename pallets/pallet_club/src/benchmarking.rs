#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use frame_support::traits::{Currency, Get};
use frame_support::sp_runtime::Saturating;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn create_club() {
        let creation_fee: BalanceOf<T> = T::CreationFee::get();
        let owner: T::AccountId = whitelisted_caller();

        let deposit_amount = creation_fee
            .saturating_mul(10_000_000u32.into());
        T::Currency::make_free_balance_be(&owner, deposit_amount);

        let annual_fee: BalanceOf<T> = 100u32.into();

        #[extrinsic_call]
        create_club(RawOrigin::Root, owner.clone(), annual_fee);
    }

    #[benchmark]
    fn transfer_ownership() {
        let owner: T::AccountId = whitelisted_caller();
        let new_owner: T::AccountId = account("new_owner", 0, 0);

        Clubs::<T>::insert(
            1u64,
            ClubInfo {
                owner: owner.clone(),
                annual_fee: 100u32.into(),
            }
        );

        #[extrinsic_call]
        transfer_ownership(RawOrigin::Signed(owner), 1u64, new_owner);
    }

    #[benchmark]
    fn set_annual_fee() {
        let owner: T::AccountId = whitelisted_caller();
        Clubs::<T>::insert(
            1u64,
            ClubInfo {
                owner: owner.clone(),
                annual_fee: 100u32.into(),
            }
        );

        #[extrinsic_call]
        set_annual_fee(RawOrigin::Signed(owner), 1u64, 200u32.into());
    }

    #[benchmark]
    fn join_club() {
        let member: T::AccountId = whitelisted_caller();
        let owner: T::AccountId = account("owner", 0, 0);
        let annual_fee: BalanceOf<T> = 100u32.into();
        let years: u32 = 10;
    
        // Insert club
        Clubs::<T>::insert(
            1u64,
            ClubInfo {
                owner: owner.clone(),
                annual_fee,
            }
        );
    
        // Calculate required funds
        let existential_deposit = T::Currency::minimum_balance();
        let cost = annual_fee.saturating_mul(years.into());
        let deposit_amount = cost.saturating_mul(100_000_000u32.into());
    
        // Fund all accounts including the pallet account
        T::Currency::make_free_balance_be(&member, deposit_amount);
        T::Currency::make_free_balance_be(&owner, existential_deposit.saturating_mul(1_000u32.into()));
        T::Currency::make_free_balance_be(
            &Pallet::<T>::account_id(),
            existential_deposit.saturating_mul(1_000u32.into())
        );
    
        #[extrinsic_call]
        join_club(RawOrigin::Signed(member), 1u64, years);
    }

    impl_benchmark_test_suite!(
        Pallet,
        crate::mock::new_test_ext(),
        crate::mock::Test
    );
}