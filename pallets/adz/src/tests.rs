use super::*;
use crate::mock::*;

use frame_support::assert_ok;

#[test]
fn create_an_ad() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert_ok!(Adz::create_ad(
            Origin::signed(1),
            "test".as_bytes().to_vec(),
            "test".as_bytes().to_vec(),
            vec!["test".as_bytes().to_vec()]
        ));
        let ad = Ads::<Test>::get(0).unwrap();
        assert_eq!(
            ad,
            Ad {
                num_of_comments: 0,
                author: 1,
                selected_applicant: None,
                created: 0,
                title: "test".as_bytes().to_vec(),
                body: "test".as_bytes().to_vec(),
                tags: vec!["test".as_bytes().to_vec()],
            }
        );
        let num_of_ads = NumOfAds::<Test>::get();
        assert_eq!(num_of_ads, 1);
    });
}

#[test]
fn update_an_ad() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert_ok!(Adz::create_ad(
            Origin::signed(1),
            "test".as_bytes().to_vec(),
            "test".as_bytes().to_vec(),
            vec!["test".as_bytes().to_vec()]
        ));

        assert_ok!(Adz::update_ad(
            Origin::signed(1),
            0,
            "test2".as_bytes().to_vec(),
            "test2".as_bytes().to_vec(),
            vec!["test2".as_bytes().to_vec()]
        ));
    });
}
