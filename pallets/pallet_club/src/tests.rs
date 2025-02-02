use crate::{mock::*, Error, Clubs, ClubInfo};
use frame_support::{assert_noop, assert_ok};
use frame_support::traits::fungible::Mutate;


#[test]
fn create_club_works() {
	new_test_ext().execute_with(|| {
		// Test root can create club
		Balances::set_balance(&1, 10000);
		assert_ok!(Club::create_club(RuntimeOrigin::root(), 1, 100));
		assert_eq!(Club::next_club_id(), 1);
		
		// Check storage
		let club = Club::clubs(0).unwrap();
		assert_eq!(club.owner, 1);
		assert_eq!(club.annual_fee, 100);
	});
}

#[test]
fn transfer_ownership_works() {
	new_test_ext().execute_with(|| {
		Clubs::<Test>::insert(0, ClubInfo { owner: 1, annual_fee: 100 });
		
		// Owner can transfer
		assert_ok!(Club::transfer_ownership(RuntimeOrigin::signed(1), 0, 2));
		let club = Club::clubs(0).unwrap();
		assert_eq!(club.owner, 2);
		
		// Non-owner cannot transfer
		assert_noop!(
			Club::transfer_ownership(RuntimeOrigin::signed(3), 0, 4),
			Error::<Test>::NotClubOwner
		);
	});
}

#[test]
fn set_annual_fee_works() {
	new_test_ext().execute_with(|| {
		// Setup: Create a club with owner 1 and initial fee 100
		Clubs::<Test>::insert(0, ClubInfo { owner: 1, annual_fee: 100 });
		
		// Test successful fee update
		assert_ok!(Club::set_annual_fee(RuntimeOrigin::signed(1), 0, 200));
		let club = Club::clubs(0).unwrap();
		assert_eq!(club.annual_fee, 200);
		
		// Test errors
		// Non-owner cannot set fee
		assert_noop!(
			Club::set_annual_fee(RuntimeOrigin::signed(2), 0, 300),
			Error::<Test>::NotClubOwner
		);
		
		// Cannot set fee for non-existent club
		assert_noop!(
			Club::set_annual_fee(RuntimeOrigin::signed(1), 1, 300),
			Error::<Test>::ClubDoesNotExist
		);
	});
}

#[test]
fn join_club_works() {
	new_test_ext().execute_with(|| {
		Clubs::<Test>::insert(0, ClubInfo { owner: 1, annual_fee: 100 });
		Balances::set_balance(&2, 1000);

		assert_ok!(Club::join_club(RuntimeOrigin::signed(2), 0, 1));
		assert_eq!(Club::members(0, &2), Some(31_536_000_000)); // 1 year
	});
}