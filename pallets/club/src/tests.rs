use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::BadOrigin;

#[test]
fn create_club_test() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::create_club(RuntimeOrigin::root(), 56, 5, 1u32));
	});
}

#[test]
fn add_member_test() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::create_club(RuntimeOrigin::root(), 56, 5, 1u32));
		assert_ok!(TemplateModule::add_member(RuntimeOrigin::signed(56), 5, 57));
		
	});
}

#[test]
fn add_member_non_owner_test() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::create_club(RuntimeOrigin::root(), 56, 5, 1u32));
		assert_noop!(
			TemplateModule::add_member(RuntimeOrigin::root(), 5, 57),
			BadOrigin,
		);
	});
}

#[test]
fn pay_membership_expense_test() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::create_club(RuntimeOrigin::root(), 56, 5, 1u32));
		assert_ok!(TemplateModule::add_member(RuntimeOrigin::signed(56), 5, 57));
		assert_ok!(TemplateModule::pay_membership_expense(RuntimeOrigin::signed(57), 5, 57));
	});
}

#[test]
fn transfer_ownership_test() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::create_club(RuntimeOrigin::root(), 56, 5, 1u32));
		assert_ok!(TemplateModule::transfer_ownership(RuntimeOrigin::signed(56), 57, 5));
	});
}

#[test]
fn set_annual_expense_test() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::create_club(RuntimeOrigin::root(), 56, 5, 1u32));
		assert_ok!(TemplateModule::set_annual_expense(RuntimeOrigin::signed(56), 5, 200u32));
	});
}
