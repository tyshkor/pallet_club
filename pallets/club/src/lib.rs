#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
use pallet_timestamp::{self as timestamp};
use frame_support::traits::Currency;
use frame_support::traits::ExistenceRequirement::AllowDeath;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

const SECOND_IN_YEAR: u32 = 31622400;
const MAX_YEARS: u32 = 100;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use sp_std::collections::btree_map::BTreeMap;
	use frame_support::pallet_prelude::{*, StorageMap};
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;

    pub type ClubId = u32;
    pub type AccountId = u32;

    #[pallet::storage]
    pub type PalletStorage<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        ClubId,
        Club<T::AccountId, <pallet_timestamp::Pallet<T> as frame_support::traits::Time>::Moment>
    >;

	#[pallet::storage]
	pub type Payouts<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::AccountId,
		Vec<T::AccountId>,
		ValueQuery,
	>;

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct Club<A, M> {
        pub owner: A,
        pub annual_expenses: u32,
        pub members: BTreeMap<A, M>,
    }

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + timestamp::Config  {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;

		/// The currency used for deposits.
		type Currency: Currency<Self::AccountId>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored { something: u32, who: T::AccountId },
		ClubCreated { club_id: ClubId },
		MemberAdded { member: T::AccountId, club_id: ClubId },
		NewOwner { new_owner: T::AccountId, club_id: ClubId },
		MembershipExpencesPayed { member: T::AccountId, club_id: ClubId },
		AnnualExpencesSet { club_id: ClubId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		NotOwner,
		NotMember,
		TooManyTokens,
		ClubDoesNotExist,
	}

	fn ensure_owner<T: Config>(origin: OriginFor<T>, club_id: ClubId) -> DispatchResult {
		let candidate = ensure_signed(origin)?;
		let club = PalletStorage::<T>::get(club_id).ok_or(Error::<T>::ClubDoesNotExist)?;
		if club.owner == candidate {
			Ok(())
		} else {
			Err(Error::<T>::NotOwner.into())
		}
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::do_something())]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored { something, who });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(1)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::cause_error())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}

        #[pallet::call_index(2)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::create_club())]
        pub fn create_club(origin: OriginFor<T>, owner: T::AccountId, club_id: ClubId, annual_expenses: u32) -> DispatchResult {
            ensure_root(origin)?;
        
            PalletStorage::<T>::insert(club_id, Club {
                owner,
                members: Default::default(),
                annual_expenses,
            });
    
            Ok(())
        }

		#[pallet::call_index(3)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::transfer_ownership())]
        pub fn transfer_ownership(origin: OriginFor<T>, new_owner: T::AccountId, club_id: ClubId) -> DispatchResult {
            ensure_owner::<T>(origin, club_id)?;
    
            let mut club = PalletStorage::<T>::get(club_id).unwrap();
			club.owner = new_owner.clone();
    
            PalletStorage::<T>::insert(club_id, club);
			Self::deposit_event(Event::NewOwner { new_owner, club_id });
            Ok(())
        }

		/// Set the annual expense for club membership.
		#[pallet::call_index(4)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::set_annual_expense())]
		pub fn set_annual_expense(origin: OriginFor<T>, club_id: ClubId, expense: u32) -> DispatchResult {
			ensure_owner::<T>(origin, club_id)?;

			let mut club = PalletStorage::<T>::get(club_id).ok_or(Error::<T>::ClubDoesNotExist)?;
			club.annual_expenses = expense;

			PalletStorage::<T>::insert(club_id, club);
			Self::deposit_event(Event::AnnualExpencesSet { club_id });
			Ok(())
		}

		#[pallet::call_index(5)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::pay_membership_expense())]
		pub fn pay_membership_expense(origin: OriginFor<T>, club_id: ClubId, expense: u32) -> DispatchResult {
			let caller = ensure_signed(origin)?;

			let mut club = PalletStorage::<T>::get(club_id).ok_or(Error::<T>::ClubDoesNotExist)?;
			club.members.get(&caller).ok_or(Error::<T>::ClubDoesNotExist)?;
			
			if club.annual_expenses * MAX_YEARS >= expense {
				return Err(Error::<T>::TooManyTokens.into());
			}
						
			T::Currency::transfer(&caller, &club.owner, club.annual_expenses.into(), AllowDeath)?;

			let current_membership_end_moment = club.members.get(&caller).unwrap();

			let new_membership_end_moment = *current_membership_end_moment + T::Moment::from(expense / club.annual_expenses * SECOND_IN_YEAR);
		
			club.members.insert(caller.clone(), new_membership_end_moment);

            PalletStorage::<T>::insert(club_id, club);
			Self::deposit_event(Event::MembershipExpencesPayed { member: caller, club_id });
			Ok(())
		}

        #[pallet::call_index(6)]
        #[pallet::weight(<T as pallet::Config>::WeightInfo::add_member())]
        pub fn add_member(origin: OriginFor<T>, club_id: ClubId, member: T::AccountId) -> DispatchResult {
			let owner = ensure_signed(origin.clone())?;
            ensure_owner::<T>(origin, club_id)?;
            let mut club = PalletStorage::<T>::get(club_id).unwrap(); // need to fix the wrapping

			T::Currency::transfer(&member, &owner, 1u32.into(), AllowDeath)?;

			let now = <timestamp::Pallet<T>>::get();
            club.members.insert(member.clone(), now);

            PalletStorage::<T>::insert(club_id, club);
			Self::deposit_event(Event::MemberAdded { member, club_id });
            Ok(())
        }
	}
}