#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

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
    use sp_std::prelude::*;

    type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;

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

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Default)]
    pub struct Comment<T: Config> {
        author: T::AccountId,
        body: Vec<u8>,
        created: u64,
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Default)]
    pub struct Ad<T: Config> {
        author: T::AccountId,
        selected_applicant: Option<T::AccountId>,
        title: Vec<u8>,
        body: Vec<u8>,
        labels: Vec<Vec<u8>>,
        created: u64,
        num_of_comments: u32,
    }

    // Storage
    #[pallet::storage]
    pub(super) type NumOfAds<T> = StorageValue<_, u32>;

    #[pallet::storage]
    #[pallet::getter(fn adz_map)]
    pub(super) type Adz<T: Config> = StorageMap<_, Identity, u32, Ad<T>>;

    #[pallet::storage]
    #[pallet::getter(fn comment_map)]
    pub(super) type Comments<T: Config> =
        StorageDoubleMap<_, Identity, u32, Identity, u32, Comment<T>>;

    // Events
    #[pallet::event]
    #[pallet::metadata(t::accountid = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        UpdateAd(T::AccountId, u32),
        CreateAd(T::AccountId, u32),
        DeleteAd(T::AccountId, u32),
        ApplicantSelected(T::AccountId, u32),

        UpdateComment(T::AccountId, u32),
        CreateComment(T::AccountId, u32),
        DeleteComment(T::AccountId, u32),
    }

    // Errors
    #[pallet::error]
    pub enum Error<T> {
        InvalidIndex,
        NotTheAuthor,
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
            labels: Vec<Vec<u8>>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            // load the user's info
            let mut num_of_ads = <NumOfAds<T>>::get().unwrap();
            // get the time from the timestamp on the block
            let created = <timestamp::Pallet<T>>::now().saturated_into::<u64>();
            // make the deposit
            let pallet = ADZ_PALLET_ID.into_account();
            let fee = T::CreateFee::get();
            T::Currency::transfer(&sender, &pallet, fee, AllowDeath)?;
            // create the ad
            let ad = Ad {
                author: sender.clone(),
                selected_applicant: None,
                title,
                body,
                labels,
                created,
                num_of_comments: 0,
            };
            // increament the number of ads made
            num_of_ads += 1;
            <NumOfAds<T>>::put(num_of_ads);
            <Adz<T>>::insert(num_of_ads, ad);

            Self::deposit_event(Event::CreateAd(sender, num_of_ads));
            Ok(())
        }

        #[pallet::weight(10_000 + <T as frame_system::Config>::DbWeight::get().writes(1))]
        pub fn update_ad(
            origin: OriginFor<T>,
            index: u32,
            title: Vec<u8>,
            body: Vec<u8>,
            labels: Vec<Vec<u8>>,
        ) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            let sender = ensure_signed(origin)?;
            match <Adz<T>>::get(index) {
                Some(mut ad) => {
                    if ad.author == sender {
                        ad.title = title;
                        ad.body = body;
                        ad.labels = labels;
                        <Adz<T>>::insert(index, ad);
                        Self::deposit_event(Event::UpdateAd(sender, index));
                        Ok(())
                    } else {
                        Err(Error::<T>::NotTheAuthor)?
                    }
                }
                None => Err(Error::<T>::InvalidIndex)?,
            }
        }

        #[pallet::weight(10_000 + <T as frame_system::Config>::DbWeight::get().writes(1))]
        pub fn delete_ad(origin: OriginFor<T>, index: u32) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            let sender = ensure_signed(origin)?;
            match <Adz<T>>::get(index) {
                Some(original) => {
                    if original.author == sender {
                        <Adz<T>>::remove(index);
                        Self::deposit_event(Event::DeleteAd(sender, index));
                        Ok(())
                    } else {
                        Err(Error::<T>::NotTheAuthor)?
                    }
                }
                None => Err(Error::<T>::InvalidIndex)?,
            }
        }

        #[pallet::weight(10_000 + <T as frame_system::Config>::DbWeight::get().writes(1))]
        pub fn select_applicant(
            origin: OriginFor<T>,
            index: u32,
            applicant: T::AccountId,
        ) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            let sender = ensure_signed(origin)?;
            match <Adz<T>>::get(index) {
                Some(mut ad) => {
                    if ad.author == sender {
                        ad.selected_applicant = Some(applicant);
                        <Adz<T>>::insert(index, ad);
                        Self::deposit_event(Event::ApplicantSelected(sender, index));
                        Ok(())
                    } else {
                        Err(Error::<T>::NotTheAuthor)?
                    }
                }
                None => Err(Error::<T>::InvalidIndex)?,
            }
        }

        /*****
        Comments
        *****/
        #[pallet::weight(10_000 + <T as frame_system::Config>::DbWeight::get().writes(1))]
        pub fn create_comment(origin: OriginFor<T>, body: Vec<u8>, ad_id: u32) -> DispatchResult {
            let author = ensure_signed(origin)?;
            // get the time from the timestamp on the block
            let created = <timestamp::Pallet<T>>::now().saturated_into::<u64>();
            // load the user's info
            let mut ad = match <Adz<T>>::get(ad_id) {
                Some(ad) => ad,
                None => Err(Error::<T>::InvalidIndex)?,
            };
            let comment = Comment {
                author,
                body,
                created,
            };
            <Comments<T>>::insert(ad_id, ad.num_of_comments, comment);
            ad.num_of_comments += 1;
            <Adz<T>>::insert(ad_id, ad);
            Ok(())
        }

        #[pallet::weight(10_000 + <T as frame_system::Config>::DbWeight::get().writes(1))]
        pub fn update_comment(
            origin: OriginFor<T>,
            ad_id: u32,
            comment_id: u32,
            body: Vec<u8>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            // load the ad's info
            let mut ad = match <Adz<T>>::get(ad_id) {
                Some(ad) => ad,
                None => Err(Error::<T>::InvalidIndex)?,
            };
            // load the ad's info
            let mut comment = match <Comments<T>>::get(ad_id, comment_id) {
                Some(comment) => comment,
                None => Err(Error::<T>::InvalidIndex)?,
            };

            if comment.author == sender {
                comment.body = body;
                <Comments<T>>::insert(ad_id, ad.num_of_comments, comment);
                ad.num_of_comments += 1;
                <Adz<T>>::insert(ad_id, ad);
                Ok(())
            } else {
                Err(Error::<T>::NotTheAuthor)?
            }
        }

        #[pallet::weight(10_000 + <T as frame_system::Config>::DbWeight::get().writes(1))]
        pub fn delete_comment(origin: OriginFor<T>, ad_id: u32, comment_id: u32) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            let sender = ensure_signed(origin)?;

            // load the ad's info
            let comment = match <Comments<T>>::get(ad_id, comment_id) {
                Some(comment) => comment,
                None => Err(Error::<T>::InvalidIndex)?,
            };

            if comment.author == sender {
                <Comments<T>>::remove(ad_id, comment_id);
                Ok(())
            } else {
                Err(Error::<T>::NotTheAuthor)?
            }
        }
    }
}
