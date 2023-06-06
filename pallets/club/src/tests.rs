use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::BadOrigin;

#[test]
fn create_club_test() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		let club_id: u32 = 5;
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::create_club(RuntimeOrigin::root(), 56, club_id, 1u32));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::ClubCreated { club_id }.into());
	});
}

#[test]
fn add_member_test() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		let club_id: u32 = 5;
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::create_club(RuntimeOrigin::root(), 56, club_id, 1u32));
		let member: u64 = 57;
		assert_ok!(TemplateModule::add_member(RuntimeOrigin::signed(56), club_id, member));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::MemberAdded { member, club_id }.into());
	});
}

#[test]
fn add_member_non_owner_test() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::create_club(RuntimeOrigin::root(), 56, 5, 1u32));
		assert_noop!(TemplateModule::add_member(RuntimeOrigin::root(), 5, 57), BadOrigin,);
	});
}

#[test]
fn pay_membership_expense_test() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		let club_id: u32 = 5;
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::create_club(RuntimeOrigin::root(), 56, club_id, 1u32));
		let member: u64 = 57;
		assert_ok!(TemplateModule::add_member(RuntimeOrigin::signed(56), 5, member));
		assert_ok!(TemplateModule::pay_membership_expense(
			RuntimeOrigin::signed(member),
			club_id,
			57
		));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::MembershipExpencesPayed { member, club_id }.into());
	});
}

#[test]
fn pay_membership_expense_too_many_tokens_test() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::create_club(RuntimeOrigin::root(), 56, 5, 1u32));
		assert_ok!(TemplateModule::add_member(RuntimeOrigin::signed(56), 5, 57));
		assert_noop!(
			TemplateModule::pay_membership_expense(RuntimeOrigin::signed(57), 5, 102),
			Error::<Test>::TooManyTokens,
		);
	});
}

#[test]
fn transfer_ownership_test() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		let club_id: u32 = 5;
		let new_owner: u64 = 57;
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::create_club(RuntimeOrigin::root(), 56, club_id, 1u32));
		assert_ok!(TemplateModule::transfer_ownership(
			RuntimeOrigin::signed(56),
			new_owner,
			club_id
		));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::NewOwner { new_owner, club_id }.into());
	});
}

#[test]
fn set_annual_expense_test() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		let club_id: u32 = 5;
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::create_club(RuntimeOrigin::root(), 56, club_id, 1u32));
		assert_ok!(TemplateModule::set_annual_expense(RuntimeOrigin::signed(56), club_id, 200u32));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::AnnualExpencesSet { club_id }.into());
	});
}
