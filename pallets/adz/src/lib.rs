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
        StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, u64, Ad>;

    #[pallet::storage]
    #[pallet::getter(fn comment_map)]
    pub(super) type Comments<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, u64, Comment>;

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        UpdateAd(T::AccountId, u64),
        CreateAd(T::AccountId, u64),
        DeleteAd(T::AccountId, u64),
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
            let pallet = ADZ_PALLET_ID.into_account();
            let fee = T::CreateFee::get();
            T::Currency::transfer(&sender, &pallet, fee, AllowDeath)?;

            let ad = Ad {
                title,
                body,
                labels,
                created: now,
                comments: vec![],
            };
            <Adz<T>>::insert(&sender, now, ad);
            Self::deposit_event(Event::CreateAd(sender, now));
            Ok(())
        }

        #[pallet::weight(10_000 + <T as frame_system::Config>::DbWeight::get().writes(1))]
        pub fn update(
            origin: OriginFor<T>,
            index: u64,
            title: Vec<u8>,
            body: Vec<u8>,
            labels: Vec<Vec<u8>>,
        ) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            let sender = ensure_signed(origin)?;
            match <Adz<T>>::get(&sender, index) {
                Some(_) => {
                    let ad = Ad {
                        title,
                        body,
                        labels,
                        created: index,
                        comments: vec![],
                    };
                    <Adz<T>>::insert(&sender, index, ad);
                    Self::deposit_event(Event::UpdateAd(sender, index));
                    Ok(())
                }
                None => Err(Error::<T>::InvalidIndex)?,
            }
        }

        #[pallet::weight(10_000 + <T as frame_system::Config>::DbWeight::get().writes(1))]
        pub fn delete(origin: OriginFor<T>, index: u64) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            let sender = ensure_signed(origin)?;
            <Adz<T>>::remove(&sender, index);
            Self::deposit_event(Event::DeleteAd(sender, index));
            Ok(())
        }
    }
}
