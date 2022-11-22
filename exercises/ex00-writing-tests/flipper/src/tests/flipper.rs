use super::mock::*;
use crate::{pallet, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn set_value_ok() {
	new_test_ext().execute_with(|| {
		assert_ok!(Flipper::set_value(Origin::signed(ALICE), false));
		assert_eq!(Flipper::value(), Some(false));
	});
}

#[test]
fn set_value_err_already_set() {
	new_test_ext().execute_with(|| {
		// Sign extrinsic with LUIS key and set value to true
		assert_ok!(Flipper::set_value(Origin::signed(LUIS), true));
		// Assert that trying to do same again is noop (doesn't touch storage) +
		// returns the error we expect
		assert_noop!(
			Flipper::set_value(Origin::signed(LUIS), true),
			Error::<TestRuntime>::AlreadySet
		);
	});
}

#[test]
fn flip_value_ok() {
	new_test_ext()
		.execute_with(|| {
			// Flip once by setting to true
			assert_ok!(Flipper::set_value(Origin::signed(LUIS), true));
			// Check val
			assert_eq!(Flipper::value(), Some(true));
			// Flip (sets to false)
			assert_ok!(Flipper::flip_value(Origin::signed(LUIS)));
			// Check val
			assert_eq!(Flipper::value(), Some(false));
		});
}

#[test]
fn flip_value_ko() {
	new_test_ext()
		.execute_with(|| {
			assert_noop!(
				Flipper::flip_value(Origin::signed(LUIS)),
				Error::<TestRuntime>::NoneValue
			);
		});
}
