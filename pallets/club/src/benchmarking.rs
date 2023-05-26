//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use sp_std::collections::btree_map::BTreeMap;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn do_something() {
		let value = 100u32.into();
		let caller: T::AccountId = whitelisted_caller();
		#[extrinsic_call]
		do_something(RawOrigin::Signed(caller), value);

		assert_eq!(Something::<T>::get(), Some(value));
	}

	#[benchmark]
	fn cause_error() {
		Something::<T>::put(100u32);
		let caller: T::AccountId = whitelisted_caller();
		#[extrinsic_call]
		cause_error(RawOrigin::Signed(caller));

		assert_eq!(Something::<T>::get(), Some(101u32));
	}

	#[benchmark]
	fn create_club() {
		let caller2: T::AccountId = whitelisted_caller();
		#[extrinsic_call]
		create_club(RawOrigin::Root, caller2, 5, 1u32);
	}

	#[benchmark]
	fn add_member() {
		let member: T::AccountId = whitelisted_caller();
		let owner: T::AccountId = whitelisted_caller();

		let club_id = 5;

		PalletStorage::<T>::insert(club_id, Club {
			owner: owner.clone(),
			members: Default::default(),
			annual_expenses: 1u32,
		});

		#[extrinsic_call]
		add_member(RawOrigin::Signed(owner), club_id, member);	
	}

	#[benchmark]
	fn transfer_ownership() {
		let owner: T::AccountId = whitelisted_caller();
		let new_owner: T::AccountId = whitelisted_caller();
		let club_id = 5;

		PalletStorage::<T>::insert(club_id, Club {
			owner: owner.clone(),
			members: Default::default(),
			annual_expenses: 1u32,
		});

		#[extrinsic_call]
		transfer_ownership(RawOrigin::Signed(owner), new_owner, club_id);
	}

	#[benchmark]
	fn set_annual_expense() {
		let owner: T::AccountId = whitelisted_caller();

		let club_id = 5;

		PalletStorage::<T>::insert(club_id, Club {
			owner: owner.clone(),
			members: Default::default(),
			annual_expenses: 1u32,
		});

		#[extrinsic_call]
		set_annual_expense(RawOrigin::Signed(owner), club_id, 2u32);	
	}

	#[benchmark]
	fn pay_membership_expense() {
		let member: T::AccountId = whitelisted_caller();
		let owner: T::AccountId = whitelisted_caller();

		let club_id = 5;

		let mut members = BTreeMap::new();

		members.insert(member.clone(), T::Moment::from(89u32));

		PalletStorage::<T>::insert(club_id, Club {
			owner: owner.clone(),
			members,
			annual_expenses: 1u32,
		});

		#[extrinsic_call]
		pay_membership_expense(RawOrigin::Signed(member), club_id, 2u32);	
	}
}
