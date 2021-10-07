#![cfg_attr(not(feature = "std"), no_std)]
#![feature(map_try_insert)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use sp_std::collections::{btree_map::*, btree_set::*};
use sp_std::prelude::Vec;

#[frame_support::pallet]
pub mod pallet {
    use codec::{Decode, Encode};
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement::AllowDeath},
        PalletId,
    };
    use frame_system::pallet_prelude::*;
    use pallet_timestamp as timestamp;
    use sp_arithmetic::traits::SaturatedConversion;
    use sp_runtime::traits::AccountIdConversion;
    use sp_std::{
        collections::{btree_map::*, btree_set::*},
        prelude::*,
    };

    type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;
    pub type AdId = u32;
    pub type CommentId = u32;

    const ADZ_PALLET_ID: PalletId = PalletId(*b"py/adzzz");

    #[pallet::config]
    pub trait Config: timestamp::Config + frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Currency: Currency<Self::AccountId>;
        type CreateFee: Get<BalanceOf<Self>>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[derive(Encode, Decode, PartialEq, RuntimeDebug)]
    pub struct Comment<T: Config> {
        author: T::AccountId,
        body: Vec<u8>,
        created: u64,
    }

    #[derive(Encode, Decode, PartialEq, RuntimeDebug)]
    pub struct Ad<T: Config> {
        pub author: T::AccountId,
        pub selected_applicant: Option<T::AccountId>,
        pub title: Vec<u8>,
        pub body: Vec<u8>,
        pub tags: Vec<Vec<u8>>,
        pub created: u64,
        pub num_of_comments: u32,
    }

    // Storage
    #[pallet::type_value]
    pub(super) fn NumOfAdsDefault() -> u32 {
        0
    }

    #[pallet::storage]
    pub(super) type NumOfAds<T> = StorageValue<_, u32, ValueQuery, NumOfAdsDefault>;

    // an index between Tags and Ads
    #[pallet::storage]
    pub(super) type Tags<T> = StorageValue<_, BTreeMap<Vec<u8>, BTreeSet<AdId>>, ValueQuery>;

    #[pallet::storage]
    pub(super) type Ads<T: Config> = StorageMap<_, Identity, AdId, Ad<T>>;

    #[pallet::storage]
    #[pallet::getter(fn comments_getter)]
    pub(super) type Comments<T: Config> =
        StorageDoubleMap<_, Identity, AdId, Identity, CommentId, Comment<T>>;

    // Events
    #[pallet::event]
    #[pallet::metadata(t::accountid = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        UpdateAd(T::AccountId, AdId),
        CreateAd(T::AccountId, AdId),
        DeleteAd(T::AccountId, AdId),

        UpdateComment(T::AccountId, AdId, CommentId),
        CreateComment(T::AccountId, AdId, CommentId),
        DeleteComment(T::AccountId, AdId, CommentId),

        ApplicantSelected(T::AccountId, AdId),
    }

    // Errors
    #[pallet::error]
    pub enum Error<T> {
        InvalidIndex,
        NotTheAuthor,
    }

    pub trait HasAuthor<T: Config> {
        fn get_author(&self) -> &T::AccountId;
    }

    #[macro_export]
    macro_rules! impl_get_author {
	    ($($t:ty),+ $(,)?) => ($(
	        impl<T: Config> HasAuthor<T> for $t {
                    fn get_author(&self) -> &T::AccountId {
                            &self.author
                    }
                }
	    )+)
	}

    impl_get_author!(Comment<T>, Ad<T>);

    fn check_author<T: Config, I: HasAuthor<T>>(
        origin: OriginFor<T>,
        item: &mut Option<I>,
    ) -> Result<(&mut I, T::AccountId), Error<T>> {
        let author = ensure_signed(origin).unwrap();
        match item {
            Some(ad) => {
                if *ad.get_author() == author {
                    Ok((ad, author))
                } else {
                    Err(Error::<T>::NotTheAuthor)
                }
            }
            None => Err(Error::<T>::InvalidIndex),
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + <T as frame_system::Config>::DbWeight::get().writes(1))]
        pub fn create_ad(
            origin: OriginFor<T>,
            title: Vec<u8>,
            body: Vec<u8>,
            tags: Vec<Vec<u8>>,
        ) -> DispatchResult {
            let author = ensure_signed(origin)?;
            // get the time from the timestamp on the block
            let created = <timestamp::Pallet<T>>::now().saturated_into::<u64>();
            // make the deposit
            let pallet = ADZ_PALLET_ID.into_account();
            let fee = T::CreateFee::get();
            T::Currency::transfer(&author, &pallet, fee, AllowDeath)?;
            // create the ad
            let ad = Ad {
                author: author.clone(),
                selected_applicant: None,
                title,
                body,
                tags: tags.clone(),
                created,
                num_of_comments: 0,
            };
            <NumOfAds<T>>::mutate(|num_of_ads| {
                <Ads<T>>::insert(*num_of_ads, ad);
                Self::update_tags(*num_of_ads, vec![], tags);
                Self::deposit_event(Event::CreateAd(author, *num_of_ads));
                // increament the number of ads made
                *num_of_ads += 1;
                Ok(())
            })
        }

        #[pallet::weight(10_000 + <T as frame_system::Config>::DbWeight::get().writes(1))]
        pub fn update_ad(
            origin: OriginFor<T>,
            index: AdId,
            title: Vec<u8>,
            body: Vec<u8>,
            tags: Vec<Vec<u8>>,
        ) -> DispatchResult {
            <Ads<T>>::mutate(index, |ad_op| {
                let (ad, author) = check_author(origin, ad_op)?;
                Self::update_tags(index, ad.tags.clone(), tags.clone());
                ad.title = title;
                ad.body = body;
                ad.tags = tags;
                Self::deposit_event(Event::UpdateAd(author, index));
                Ok(())
            })
        }

        #[pallet::weight(10_000 + <T as frame_system::Config>::DbWeight::get().writes(1))]
        pub fn delete_ad(origin: OriginFor<T>, index: AdId) -> DispatchResult {
            <Ads<T>>::try_mutate_exists(index, |ad_op| {
                let (ad, author) = check_author(origin, ad_op)?;
                Self::update_tags(index, ad.tags.clone(), vec![]);
                Self::deposit_event(Event::DeleteAd(author, index));
                *ad_op = None;
                Ok(())
            })
        }

        /*****
        Comments
        *****/
        #[pallet::weight(10_000 + <T as frame_system::Config>::DbWeight::get().writes(1))]
        pub fn create_comment(origin: OriginFor<T>, body: Vec<u8>, ad_id: AdId) -> DispatchResult {
            let author = ensure_signed(origin)?;
            // get the time from the timestamp on the block
            let created = <timestamp::Pallet<T>>::now().saturated_into::<u64>();
            // load the user's info
            <Ads<T>>::try_mutate(ad_id, |ad_op| match ad_op {
                Some(ad) => {
                    let comment = Comment {
                        author: author.clone(),
                        body,
                        created,
                    };
                    <Comments<T>>::insert(ad_id, ad.num_of_comments, comment);
                    Self::deposit_event(Event::CreateComment(author, ad_id, ad.num_of_comments));
                    ad.num_of_comments += 1;
                    Ok(())
                }
                None => Err(Error::<T>::InvalidIndex)?,
            })
        }

        #[pallet::weight(10_000 + <T as frame_system::Config>::DbWeight::get().writes(1))]
        pub fn update_comment(
            origin: OriginFor<T>,
            ad_id: AdId,
            comment_id: CommentId,
            body: Vec<u8>,
        ) -> DispatchResult {
            <Comments<T>>::try_mutate_exists(ad_id, comment_id, |c| {
                let (comment, author) = check_author(origin, c)?;
                comment.body = body;
                Self::deposit_event(Event::UpdateComment(author, ad_id, comment_id));
                Ok(())
            })
        }

        #[pallet::weight(10_000 + <T as frame_system::Config>::DbWeight::get().writes(1))]
        pub fn delete_comment(
            origin: OriginFor<T>,
            ad_id: AdId,
            comment_id: CommentId,
        ) -> DispatchResult {
            <Comments<T>>::try_mutate_exists(ad_id, comment_id, |comment| {
                let (_, author) = check_author(origin, comment)?;
                Self::deposit_event(Event::DeleteComment(author, ad_id, comment_id));
                *comment = None;
                Ok(())
            })
        }

        /**
        / Misc
        **/
        #[pallet::weight(10_000 + <T as frame_system::Config>::DbWeight::get().writes(1))]
        pub fn select_applicant(
            origin: OriginFor<T>,
            index: AdId,
            applicant: T::AccountId,
        ) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            <Ads<T>>::try_mutate(index, |ad_op| {
                let (ad, author) = check_author(origin, ad_op)?;
                let pallet = ADZ_PALLET_ID.into_account();
                let fee = T::CreateFee::get();
                T::Currency::transfer(&pallet, &author, fee, AllowDeath)?;
                ad.selected_applicant = Some(applicant);
                Self::deposit_event(Event::ApplicantSelected(author, index));
                Ok(())
            })
        }
    }
}

fn get_set<'a>(
    map: &'a mut BTreeMap<Vec<u8>, BTreeSet<CommentId>>,
    key: &Vec<u8>,
) -> &'a mut BTreeSet<CommentId> {
    if map.contains_key(key) {
        map.get_mut(key).unwrap()
    } else {
        map.try_insert(key.to_vec(), BTreeSet::new()).unwrap()
    }
}

impl<T: Config> Pallet<T> {
    fn update_tags(ad_id: AdId, old_tags: Vec<Vec<u8>>, new_tags: Vec<Vec<u8>>) {
        <Tags<T>>::mutate(|tags| {
            //remove old tags
            for old_tag in old_tags.iter() {
                let tag_set = get_set(tags, old_tag);
                tag_set.remove(&ad_id);
                if tag_set.is_empty() {
                    tags.remove(old_tag);
                }
            }
            // ad new tags
            for new_tag in new_tags.iter() {
                get_set(tags, new_tag).insert(ad_id);
            }
        });
    }
}
