use super::*;
use crate::mock::*;
use frame_support::assert_ok;
use frame_system::ensure_signed;

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

        let ad = Ads::<Test>::get(0).unwrap();
        assert_eq!(
            ad,
            Ad {
                num_of_comments: 0,
                author: 1,
                selected_applicant: None,
                created: 0,
                title: "test2".as_bytes().to_vec(),
                body: "test2".as_bytes().to_vec(),
                tags: vec!["test2".as_bytes().to_vec()],
            }
        );

        let tag_map = Tags::<Test>::get();
        let mut tag_map_test = BTreeMap::new();
        let mut ad_set = BTreeSet::new();
        ad_set.insert(0);
        tag_map_test.insert("test2".as_bytes().to_vec(), ad_set);
        assert_eq!(tag_map, tag_map_test);
    });
}

#[test]
fn create_an_comment() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert_ok!(Adz::create_ad(
            Origin::signed(1),
            "test".as_bytes().to_vec(),
            "test".as_bytes().to_vec(),
            vec!["test".as_bytes().to_vec()]
        ));

        assert_ok!(Adz::create_comment(
            Origin::signed(2),
            "test".as_bytes().to_vec(),
            0
        ));

        // select an apllicant
        let selected = ensure_signed(Origin::signed(2)).unwrap();
        assert_ok!(Adz::select_applicant(Origin::signed(1), 0, selected));

        assert_eq!(
            Ads::<Test>::get(0).unwrap(),
            Ad {
                num_of_comments: 1,
                author: 1,
                selected_applicant: Some(selected),
                created: 0,
                title: "test".as_bytes().to_vec(),
                body: "test".as_bytes().to_vec(),
                tags: vec!["test".as_bytes().to_vec()],
            }
        );
    });
}
