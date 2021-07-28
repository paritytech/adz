#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
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
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use pallet_timestamp as timestamp;
    use sp_arithmetic::traits::SaturatedConversion;
    use sp_std::prelude::*;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: timestamp::Config + frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[derive(Encode, Decode, Clone)]
    pub struct Comment {
        body: Vec<u8>,
        created: u64,
    }

    #[derive(Encode, Decode, Clone)]
    pub struct Ad {
        title: Vec<u8>,
        body: Vec<u8>,
        labels: Vec<Vec<u8>>,
        created: u64,
        comments: Vec<Comment>,
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn adz_map)]
    pub(super) type Adz<T: Config> =
        StorageMap<_, Blake2_128Concat, <T as frame_system::Config>::AccountId, Vec<Ad>>;

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        UpdateAd(u32, <T as frame_system::Config>::AccountId),
        CreateAd(u32, <T as frame_system::Config>::AccountId),
        DeleteAd(u32, <T as frame_system::Config>::AccountId),
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
            // Something::put(something);
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

        #[pallet::weight(10_000 + <T as frame_system::Config>::DbWeight::get().writes(1))]
        pub fn get_all(origin: OriginFor<T>) -> DispatchResult {
            Ok(())
        }
    }
}
