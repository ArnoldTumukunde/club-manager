#![cfg(feature = "runtime-benchmarks")]
use super::*;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use frame_support::{
    traits::{Currency, Get},
    sp_runtime::traits::SaturatedConversion,
};

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn create_club() {
        let fee = T::CreationFee::get();
        let caller: T::AccountId = whitelisted_caller();
        T::Currency::make_free_balance_be(&caller, fee * 100u32.into());
        
        #[extrinsic_call]
        create_club(RawOrigin::Root, caller.clone(), 100u32.into());
    }

    #[benchmark]
    fn transfer_ownership() {
        let owner: T::AccountId = whitelisted_caller();
        let new_owner: T::AccountId = account("new_owner", 0, 0);
        Clubs::<T>::insert(1u64, ClubInfo { owner: owner.clone(), annual_fee: 100u32.into() });
        
        #[extrinsic_call]
        transfer_ownership(RawOrigin::Signed(owner), 1u64, new_owner);
    }

    #[benchmark]
    fn set_annual_fee() {
        let owner: T::AccountId = whitelisted_caller();
        Clubs::<T>::insert(1u64, ClubInfo { owner: owner.clone(), annual_fee: 100u32.into() });
        
        #[extrinsic_call]
        set_annual_fee(RawOrigin::Signed(owner), 1u64, 200u32.into());
    }

    #[benchmark]
    fn join_club() {
        let member: T::AccountId = whitelisted_caller();
        let fee = 100u32;
        Clubs::<T>::insert(1u64, ClubInfo { owner: member.clone(), annual_fee: fee.into() });
        T::Currency::make_free_balance_be(&member, (fee * 1000).into());
        
        let current_timestamp: T::Moment = 10_000u64.saturated_into();
        pallet_timestamp::Now::<T>::put(current_timestamp);

        #[extrinsic_call]
        join_club(RawOrigin::Signed(member), 1u64, 10u32,current_timestamp);
    }

    impl_benchmark_test_suite!(
        Pallet,
        crate::mock::new_test_ext(),
        crate::mock::Test
    );
}