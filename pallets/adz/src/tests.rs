use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_an_ad() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert_ok!(Adz::create(
            Origin::signed(1),
            "test".as_bytes().to_vec(),
            "test".as_bytes().to_vec(),
            vec!["test".as_bytes().to_vec()]
        ));
    });
}

#[test]
fn update_an_ad() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert_ok!(Adz::create(
            Origin::signed(1),
            "test".as_bytes().to_vec(),
            "test".as_bytes().to_vec(),
            vec!["test".as_bytes().to_vec()]
        ));

        assert_ok!(Adz::update(
            Origin::signed(1),
            0,
            "test2".as_bytes().to_vec(),
            "test2".as_bytes().to_vec(),
            vec!["test2".as_bytes().to_vec()]
        ));
    });
}
