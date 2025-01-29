#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)] 
mod tests;

#[cfg(feature = "runtime-benchmarks")] 
mod benchmarking;

pub mod weights;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{pallet_prelude::*, ensure, traits::{Currency, ReservableCurrency}};
	use frame_system::pallet_prelude::*;
	use sp_arithmetic::traits::{CheckedAdd, CheckedMul, SaturatedConversion};
	use scale_info::TypeInfo;
	use frame_support::{ PalletId,  traits::ExistenceRequirement};
	use frame_support::sp_runtime::traits::AccountIdConversion;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configuration trait
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
		type WeightInfo: WeightInfo;
		#[pallet::constant] 
		type CreationFee: Get<BalanceOf<Self>>;
		#[pallet::constant] 
		type MaxYears: Get<u32>;
		#[pallet::constant] 
		type YearDuration: Get<u64>;
	}

	pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
	type ClubId = u64;

	/// Storage: Next Club ID
	#[pallet::storage] 
	#[pallet::getter(fn next_club_id)]
	pub type NextClubId<T> = StorageValue<_, ClubId, ValueQuery>;

	/// Storage: Club information
	#[pallet::storage] 
	#[pallet::getter(fn clubs)]
	pub type Clubs<T: Config> = StorageMap<
		_, Blake2_128Concat, ClubId, ClubInfo<T>, OptionQuery
	>;

	/// Storage: Club memberships
	#[pallet::storage] 
	#[pallet::getter(fn members)]
	pub type Members<T: Config> = StorageDoubleMap<
		_, Blake2_128Concat, ClubId, Blake2_128Concat, T::AccountId, T::Moment
	>;

	/// Club information structure
	#[derive(Clone, Encode, Decode, PartialEq, TypeInfo, RuntimeDebug, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct ClubInfo<T: Config> {
		pub owner: T::AccountId,
		pub annual_fee: BalanceOf<T>,
	}
	

	/// Events
	#[pallet::event] 
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClubCreated { club_id: ClubId, owner: T::AccountId, annual_fee: BalanceOf<T> },
		OwnershipTransferred { club_id: ClubId, new_owner: T::AccountId },
		AnnualFeeSet { club_id: ClubId, new_fee: BalanceOf<T> },
		MemberJoined { club_id: ClubId, member: T::AccountId, expiry: T::Moment },
	}

	#[pallet::error]
	pub enum Error<T> {
		NotRoot,
		InsufficientFunds,
		ClubDoesNotExist,
		NotClubOwner,
		YearsExceedMax,
		YearsZero,
		TransferToSelf,
		Overflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight({ <T as Config>::WeightInfo::create_club() })]
		pub fn create_club(
			origin: OriginFor<T>,
			owner: T::AccountId,
			annual_fee: BalanceOf<T>,
		) -> DispatchResult {
			ensure_root(origin)?;
			
			let creation_fee = T::CreationFee::get();
			T::Currency::reserve(&owner, creation_fee)
				.map_err(|_| Error::<T>::InsufficientFunds)?;

			let club_id = NextClubId::<T>::get();
			Clubs::<T>::insert(club_id, ClubInfo { owner: owner.clone(), annual_fee });
			NextClubId::<T>::put(club_id + 1);

			Self::deposit_event(Event::ClubCreated { club_id, owner, annual_fee });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight({ <T as Config>::WeightInfo::transfer_ownership() })]
		pub fn transfer_ownership(
			origin: OriginFor<T>,
			club_id: ClubId,
			new_owner: T::AccountId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			
			let mut club = Clubs::<T>::get(club_id).ok_or(Error::<T>::ClubDoesNotExist)?;
			ensure!(sender == club.owner, Error::<T>::NotClubOwner);
			ensure!(new_owner != club.owner, Error::<T>::TransferToSelf);

			club.owner = new_owner.clone();
			Clubs::<T>::insert(club_id, club);

			Self::deposit_event(Event::OwnershipTransferred { club_id, new_owner });
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight({ <T as Config>::WeightInfo::set_annual_fee() })]
		pub fn set_annual_fee(
			origin: OriginFor<T>,
			club_id: ClubId,
			new_fee: BalanceOf<T>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			
			let mut club = Clubs::<T>::get(club_id).ok_or(Error::<T>::ClubDoesNotExist)?;
			ensure!(sender == club.owner, Error::<T>::NotClubOwner);

			club.annual_fee = new_fee;
			Clubs::<T>::insert(club_id, club);

			Self::deposit_event(Event::AnnualFeeSet { club_id, new_fee });
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight({ <T as Config>::WeightInfo::join_club() })]
		pub fn join_club(
			origin: OriginFor<T>,
			club_id: ClubId,
			years: u32,
		) -> DispatchResult {
			let member = ensure_signed(origin)?;
			let club = Clubs::<T>::get(club_id).ok_or(Error::<T>::ClubDoesNotExist)?;
			
			ensure!(years <= T::MaxYears::get(), Error::<T>::YearsExceedMax);
			ensure!(years > 0, Error::<T>::YearsZero);

			let total_cost = club.annual_fee.checked_mul(&years.into())
				.ok_or(Error::<T>::Overflow)?;

			T::Currency::transfer(
				&member,
				&Self::account_id(),
				total_cost,
				ExistenceRequirement::KeepAlive,
			)?;
			let now = pallet_timestamp::Pallet::<T>::get();
			let duration_per_year = T::YearDuration::get();
			let durations = duration_per_year.checked_mul(years as u64)
				.ok_or(Error::<T>::Overflow)?;
			let duration = T::Moment::saturated_from(durations);
			let new_expiry = now.checked_add(&duration)
				.ok_or(Error::<T>::Overflow)?;

			Members::<T>::mutate(club_id, &member, |expiry| {
				let current_expiry = expiry.unwrap_or(T::Moment::saturated_from(0u64));
				*expiry = if current_expiry > now {
					Some(current_expiry.checked_add(&duration)
						.ok_or(Error::<T>::Overflow)?)
				} else {
					Some(new_expiry)
				};
				Ok::<(), Error<T>>(()) 
			})?;

			Self::deposit_event(Event::MemberJoined { club_id, member, expiry: new_expiry });
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn account_id() -> T::AccountId {
			let pallet_id = PalletId(*b"ClubMngr");
			pallet_id.into_account_truncating()
		}
	}
}