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

    #[pallet::config]
    pub trait Config: timestamp::Config + frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Currency: Currency<Self::AccountId>;
        type PalletId: Get<PalletId>;
        type CreateFee: Get<BalanceOf<Self>>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Default)]
    pub struct Comment {
        body: Vec<u8>,
        created: u64,
    }

    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, Default)]
    pub struct Ad {
        title: Vec<u8>,
        body: Vec<u8>,
        labels: Vec<Vec<u8>>,
        created: u64,
        comments: Vec<Comment>,
    }

    #[pallet::storage]
    #[pallet::getter(fn adz_map)]
    pub(super) type Adz<T: Config> =
        StorageMap<_, Blake2_128Concat, <T as frame_system::Config>::AccountId, Vec<Ad>>;

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        UpdateAd(u32, T::AccountId),
        CreateAd(u32, T::AccountId),
        DeleteAd(u32, T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        InvalidIndex,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + <T as frame_system::Config>::DbWeight::get().writes(1))]
        pub fn create(
            origin: OriginFor<T>,
            title: Vec<u8>,
            body: Vec<u8>,
            labels: Vec<Vec<u8>>,
        ) -> DispatchResult {
            // get the time from the timestamp on the block
            let now = <timestamp::Pallet<T>>::now().saturated_into::<u64>();
            // Check that the extrinsic was signed and get the signer.
            let sender = ensure_signed(origin)?;
            let pallet: T::AccountId = T::PalletId::get().into_account();
            let fee = T::CreateFee::get();
            T::Currency::transfer(&sender, &pallet, fee, AllowDeath)?;
            let mut ads = match <Adz<T>>::get(&sender) {
                Some(inner) => inner,
                None => Vec::new(),
            };
            let ads_len = ads.len() as u32;
            ads.push(Ad {
                title,
                body,
                labels,
                created: now,
                comments: vec![],
            });
            <Adz<T>>::insert(&sender, ads);
            Self::deposit_event(Event::CreateAd(ads_len, sender));
            Ok(())
        }

        #[pallet::weight(10_000 + <T as frame_system::Config>::DbWeight::get().writes(1))]
        pub fn update(
            origin: OriginFor<T>,
            index: u8,
            title: Vec<u8>,
            body: Vec<u8>,
            labels: Vec<Vec<u8>>,
        ) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            let sender = ensure_signed(origin)?;
            let mut ads = match <Adz<T>>::get(&sender) {
                Some(inner) => inner,
                None => Vec::new(),
            };
            match ads.get_mut(index as usize) {
                Some(ad) => {
                    *ad = Ad {
                        title,
                        body,
                        labels,
                        created: ad.created,
                        comments: ad.comments.clone(),
                    };
                    Self::deposit_event(Event::UpdateAd(index as u32, sender));
                    Ok(())
                }
                None => Err(Error::<T>::InvalidIndex)?,
            }
        }
    }
}
