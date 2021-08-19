use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn creat_an_ad() {
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
