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
	use frame_support::{dispatch::{DispatchResult, DispatchResultWithPostInfo}, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Maximum length of a web4 domain name
		#[pallet::constant]
		type MaxDomainLength: Get<u32>;

		/// Maximum length of web4 content
		#[pallet::constant]
		type MaxContentLength: Get<u32>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Web4 domains storage
	#[pallet::storage]
	#[pallet::getter(fn domains)]
	pub type Domains<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxDomainLength>,
		DomainInfo<T>,
		OptionQuery,
	>;

	/// Domain information
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct DomainInfo<T: Config> {
		/// Domain owner
		pub owner: T::AccountId,
		/// Domain content hash
		pub content_hash: BoundedVec<u8, T::MaxContentLength>,
		/// Creation timestamp
		pub created: T::BlockNumber,
		/// Last update timestamp
		pub updated: T::BlockNumber,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new domain was registered
		DomainRegistered {
			domain: BoundedVec<u8, T::MaxDomainLength>,
			owner: T::AccountId,
		},
		/// Domain content was updated
		DomainUpdated {
			domain: BoundedVec<u8, T::MaxDomainLength>,
			owner: T::AccountId,
		},
		/// Domain was transferred
		DomainTransferred {
			domain: BoundedVec<u8, T::MaxDomainLength>,
			from: T::AccountId,
			to: T::AccountId,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Domain already exists
		DomainAlreadyExists,
		/// Domain does not exist
		DomainNotFound,
		/// Not authorized to perform this action
		NotAuthorized,
		/// Domain name too long
		DomainTooLong,
		/// Content too long
		ContentTooLong,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Register a new Web4 domain
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn register_domain(
			origin: OriginFor<T>,
			domain: Vec<u8>,
			content_hash: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let domain = BoundedVec::<u8, T::MaxDomainLength>::try_from(domain)
				.map_err(|_| Error::<T>::DomainTooLong)?;
			let content_hash = BoundedVec::<u8, T::MaxContentLength>::try_from(content_hash)
				.map_err(|_| Error::<T>::ContentTooLong)?;

			// Check if domain already exists
			ensure!(!Domains::<T>::contains_key(&domain), Error::<T>::DomainAlreadyExists);

			// Create domain info
			let domain_info = DomainInfo::<T> {
				owner: who.clone(),
				content_hash,
				created: frame_system::Pallet::<T>::block_number(),
				updated: frame_system::Pallet::<T>::block_number(),
			};

			// Store domain
			Domains::<T>::insert(&domain, domain_info);

			// Emit event
			Self::deposit_event(Event::DomainRegistered {
				domain: domain.clone(),
				owner: who,
			});

			Ok(())
		}

		/// Update domain content
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn update_domain(
			origin: OriginFor<T>,
			domain: Vec<u8>,
			content_hash: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let domain = BoundedVec::<u8, T::MaxDomainLength>::try_from(domain)
				.map_err(|_| Error::<T>::DomainTooLong)?;
			let content_hash = BoundedVec::<u8, T::MaxContentLength>::try_from(content_hash)
				.map_err(|_| Error::<T>::ContentTooLong)?;

			// Get existing domain
			let mut domain_info = Domains::<T>::get(&domain)
				.ok_or(Error::<T>::DomainNotFound)?;

			// Check ownership
			ensure!(domain_info.owner == who, Error::<T>::NotAuthorized);

			// Update content hash and timestamp
			domain_info.content_hash = content_hash;
			domain_info.updated = frame_system::Pallet::<T>::block_number();

			// Store updated domain
			Domains::<T>::insert(&domain, domain_info);

			// Emit event
			Self::deposit_event(Event::DomainUpdated {
				domain: domain.clone(),
				owner: who,
			});

			Ok(())
		}

		/// Transfer domain ownership
		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn transfer_domain(
			origin: OriginFor<T>,
			domain: Vec<u8>,
			to: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vector
			let domain = BoundedVec::<u8, T::MaxDomainLength>::try_from(domain)
				.map_err(|_| Error::<T>::DomainTooLong)?;

			// Get existing domain
			let mut domain_info = Domains::<T>::get(&domain)
				.ok_or(Error::<T>::DomainNotFound)?;

			// Check ownership
			ensure!(domain_info.owner == who, Error::<T>::NotAuthorized);

			// Update owner
			let old_owner = domain_info.owner.clone();
			domain_info.owner = to.clone();
			domain_info.updated = frame_system::Pallet::<T>::block_number();

			// Store updated domain
			Domains::<T>::insert(&domain, domain_info);

			// Emit event
			Self::deposit_event(Event::DomainTransferred {
				domain: domain.clone(),
				from: old_owner,
				to,
			});

			Ok(())
		}
	}
}