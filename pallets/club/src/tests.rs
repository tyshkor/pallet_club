use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::do_something(RuntimeOrigin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::something(), Some(42));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::SomethingStored { something: 42, who: 1 }.into());
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			TemplateModule::cause_error(RuntimeOrigin::signed(1)),
			Error::<Test>::NoneValue
		);
	});
}

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
