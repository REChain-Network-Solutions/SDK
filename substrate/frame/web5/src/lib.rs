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

		/// Maximum length of a DID identifier
		#[pallet::constant]
		type MaxDidLength: Get<u32>;

		/// Maximum length of identity metadata
		#[pallet::constant]
		type MaxIdentityMetadataLength: Get<u32>;

		/// Maximum number of credentials per identity
		#[pallet::constant]
		type MaxCredentialsPerIdentity: Get<u32>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Decentralized Identity storage
	#[pallet::storage]
	#[pallet::getter(fn decentralized_ids)]
	pub type DecentralizedIds<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxDidLength>,
		IdentityInfo<T>,
		OptionQuery,
	>;

	/// Verifiable Credentials storage
	#[pallet::storage]
	#[pallet::getter(fn verifiable_credentials)]
	pub type VerifiableCredentials<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxDidLength>,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxIdentityMetadataLength>,
		CredentialInfo<T>,
		OptionQuery,
	>;

	/// Identity information structure
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct IdentityInfo<T: Config> {
		/// Identity controller
		pub controller: T::AccountId,
		/// Public key references
		pub public_keys: BoundedVec<BoundedVec<u8, T::MaxIdentityMetadataLength>, ConstU32<10>>,
		/// Services (endpoints)
		pub services: BoundedVec<ServiceInfo<T>, ConstU32<10>>,
		/// Creation timestamp
		pub created: T::BlockNumber,
		/// Last update timestamp
		pub updated: T::BlockNumber,
		/// Credential count
		pub credential_count: u32,
	}

	/// Service information
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct ServiceInfo<T: Config> {
		/// Service ID
		pub id: BoundedVec<u8, T::MaxIdentityMetadataLength>,
		/// Service type
		pub service_type: BoundedVec<u8, T::MaxIdentityMetadataLength>,
		/// Service endpoint
		pub service_endpoint: BoundedVec<u8, T::MaxIdentityMetadataLength>,
	}

	/// Verifiable Credential information
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct CredentialInfo<T: Config> {
		/// Credential issuer
		pub issuer: T::AccountId,
		/// Credential subject (DID)
		pub subject: BoundedVec<u8, T::MaxDidLength>,
		/// Credential type
		pub credential_type: BoundedVec<u8, T::MaxIdentityMetadataLength>,
		/// Credential data hash
		pub data_hash: BoundedVec<u8, T::MaxIdentityMetadataLength>,
		/// Issuance timestamp
		pub issued_at: T::BlockNumber,
		/// Expiration timestamp (if any)
		pub expires_at: Option<T::BlockNumber>,
		/// Revoked status
		pub revoked: bool,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new DID was created
		DidCreated {
			did: BoundedVec<u8, T::MaxDidLength>,
			controller: T::AccountId,
		},
		/// DID was updated
		DidUpdated {
			did: BoundedVec<u8, T::MaxDidLength>,
			controller: T::AccountId,
		},
		/// Verifiable credential was issued
		CredentialIssued {
			did: BoundedVec<u8, T::MaxDidLength>,
			credential_id: BoundedVec<u8, T::MaxIdentityMetadataLength>,
			issuer: T::AccountId,
		},
		/// Credential was revoked
		CredentialRevoked {
			did: BoundedVec<u8, T::MaxDidLength>,
			credential_id: BoundedVec<u8, T::MaxIdentityMetadataLength>,
			revoker: T::AccountId,
		},
		/// Service was added to DID
		ServiceAdded {
			did: BoundedVec<u8, T::MaxDidLength>,
			service_id: BoundedVec<u8, T::MaxIdentityMetadataLength>,
			controller: T::AccountId,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// DID already exists
		DidAlreadyExists,
		/// DID does not exist
		DidNotFound,
		/// Credential already exists
		CredentialAlreadyExists,
		/// Credential does not exist
		CredentialNotFound,
		/// Not authorized to perform this action
		NotAuthorized,
		/// DID identifier too long
		DidTooLong,
		/// Identity metadata too long
		IdentityMetadataTooLong,
		/// Maximum credentials per identity reached
		MaxCredentialsReached,
		/// Credential expired
		CredentialExpired,
		/// Credential already revoked
		CredentialAlreadyRevoked,
		/// Invalid service information
		InvalidService,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new decentralized identity (DID)
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_did(
			origin: OriginFor<T>,
			did: Vec<u8>,
			public_keys: Vec<Vec<u8>>,
			services: Vec<(Vec<u8>, Vec<u8>, Vec<u8>)>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let did = BoundedVec::<u8, T::MaxDidLength>::try_from(did)
				.map_err(|_| Error::<T>::DidTooLong)?;

			// Check if DID already exists
			ensure!(!DecentralizedIds::<T>::contains_key(&did), Error::<T>::DidAlreadyExists);

			// Convert public keys
			let mut public_keys_bounded = BoundedVec::<BoundedVec<u8, T::MaxIdentityMetadataLength>, ConstU32<10>>::new();
			for key in public_keys {
				let key_bounded = BoundedVec::<u8, T::MaxIdentityMetadataLength>::try_from(key)
					.map_err(|_| Error::<T>::IdentityMetadataTooLong)?;
				public_keys_bounded.try_push(key_bounded)
					.map_err(|_| Error::<T>::MaxCredentialsReached)?;
			}

			// Convert services
			let mut services_bounded = BoundedVec::<ServiceInfo<T>, ConstU32<10>>::new();
			for (id, service_type, endpoint) in services {
				let id_bounded = BoundedVec::<u8, T::MaxIdentityMetadataLength>::try_from(id)
					.map_err(|_| Error::<T>::IdentityMetadataTooLong)?;
				let service_type_bounded = BoundedVec::<u8, T::MaxIdentityMetadataLength>::try_from(service_type)
					.map_err(|_| Error::<T>::IdentityMetadataTooLong)?;
				let endpoint_bounded = BoundedVec::<u8, T::MaxIdentityMetadataLength>::try_from(endpoint)
					.map_err(|_| Error::<T>::IdentityMetadataTooLong)?;

				services_bounded.try_push(ServiceInfo::<T> {
					id: id_bounded,
					service_type: service_type_bounded,
					service_endpoint: endpoint_bounded,
				}).map_err(|_| Error::<T>::MaxCredentialsReached)?;
			}

			// Create identity info
			let identity_info = IdentityInfo::<T> {
				controller: who.clone(),
				public_keys: public_keys_bounded,
				services: services_bounded,
				created: frame_system::Pallet::<T>::block_number(),
				updated: frame_system::Pallet::<T>::block_number(),
				credential_count: 0,
			};

			// Store DID
			DecentralizedIds::<T>::insert(&did, identity_info);

			// Emit event
			Self::deposit_event(Event::DidCreated {
				did: did.clone(),
				controller: who,
			});

			Ok(())
		}

		/// Issue a verifiable credential
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
		pub fn issue_credential(
			origin: OriginFor<T>,
			subject_did: Vec<u8>,
			credential_id: Vec<u8>,
			credential_type: Vec<u8>,
			data_hash: Vec<u8>,
			expires_at: Option<T::BlockNumber>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let subject_did = BoundedVec::<u8, T::MaxDidLength>::try_from(subject_did)
				.map_err(|_| Error::<T>::DidTooLong)?;
			let credential_id = BoundedVec::<u8, T::MaxIdentityMetadataLength>::try_from(credential_id)
				.map_err(|_| Error::<T>::IdentityMetadataTooLong)?;
			let credential_type = BoundedVec::<u8, T::MaxIdentityMetadataLength>::try_from(credential_type)
				.map_err(|_| Error::<T>::IdentityMetadataTooLong)?;
			let data_hash = BoundedVec::<u8, T::MaxIdentityMetadataLength>::try_from(data_hash)
				.map_err(|_| Error::<T>::IdentityMetadataTooLong)?;

			// Get subject identity
			let mut subject_identity = DecentralizedIds::<T>::get(&subject_did)
				.ok_or(Error::<T>::DidNotFound)?;

			// Check authorization (issuer must be authorized or controller)
			ensure!(who == subject_identity.controller, Error::<T>::NotAuthorized);

			// Check credential limit
			ensure!(subject_identity.credential_count < T::MaxCredentialsPerIdentity::get(), Error::<T>::MaxCredentialsReached);

			// Check if credential already exists
			ensure!(!VerifiableCredentials::<T>::contains_key(&subject_did, &credential_id), Error::<T>::CredentialAlreadyExists);

			// Create credential info
			let credential_info = CredentialInfo::<T> {
				issuer: who.clone(),
				subject: subject_did.clone(),
				credential_type,
				data_hash,
				issued_at: frame_system::Pallet::<T>::block_number(),
				expires_at,
				revoked: false,
			};

			// Store credential
			VerifiableCredentials::<T>::insert(&subject_did, &credential_id, credential_info);

			// Update identity credential count
			subject_identity.credential_count += 1;
			subject_identity.updated = frame_system::Pallet::<T>::block_number();
			DecentralizedIds::<T>::insert(&subject_did, subject_identity);

			// Emit event
			Self::deposit_event(Event::CredentialIssued {
				did: subject_did.clone(),
				credential_id: credential_id.clone(),
				issuer: who,
			});

			Ok(())
		}

		/// Revoke a verifiable credential
		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn revoke_credential(
			origin: OriginFor<T>,
			subject_did: Vec<u8>,
			credential_id: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let subject_did = BoundedVec::<u8, T::MaxDidLength>::try_from(subject_did)
				.map_err(|_| Error::<T>::DidTooLong)?;
			let credential_id = BoundedVec::<u8, T::MaxIdentityMetadataLength>::try_from(credential_id)
				.map_err(|_| Error::<T>::IdentityMetadataTooLong)?;

			// Get credential
			let mut credential_info = VerifiableCredentials::<T>::get(&subject_did, &credential_id)
				.ok_or(Error::<T>::CredentialNotFound)?;

			// Check authorization (only issuer can revoke)
			ensure!(credential_info.issuer == who, Error::<T>::NotAuthorized);

			// Check if already revoked
			ensure!(!credential_info.revoked, Error::<T>::CredentialAlreadyRevoked);

			// Check expiration
			if let Some(expiry) = credential_info.expires_at {
				let current_block = frame_system::Pallet::<T>::block_number();
				ensure!(current_block < expiry, Error::<T>::CredentialExpired);
			}

			// Revoke credential
			credential_info.revoked = true;
			VerifiableCredentials::<T>::insert(&subject_did, &credential_id, credential_info);

			// Emit event
			Self::deposit_event(Event::CredentialRevoked {
				did: subject_did.clone(),
				credential_id: credential_id.clone(),
				revoker: who,
			});

			Ok(())
		}

		/// Add a service to a DID
		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn add_service(
			origin: OriginFor<T>,
			did: Vec<u8>,
			service_id: Vec<u8>,
			service_type: Vec<u8>,
			service_endpoint: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let did = BoundedVec::<u8, T::MaxDidLength>::try_from(did)
				.map_err(|_| Error::<T>::DidTooLong)?;
			let service_id = BoundedVec::<u8, T::MaxIdentityMetadataLength>::try_from(service_id)
				.map_err(|_| Error::<T>::IdentityMetadataTooLong)?;
			let service_type = BoundedVec::<u8, T::MaxIdentityMetadataLength>::try_from(service_type)
				.map_err(|_| Error::<T>::IdentityMetadataTooLong)?;
			let service_endpoint = BoundedVec::<u8, T::MaxIdentityMetadataLength>::try_from(service_endpoint)
				.map_err(|_| Error::<T>::IdentityMetadataTooLong)?;

			// Get identity
			let mut identity_info = DecentralizedIds::<T>::get(&did)
				.ok_or(Error::<T>::DidNotFound)?;

			// Check authorization (only controller)
			ensure!(identity_info.controller == who, Error::<T>::NotAuthorized);

			// Create service info
			let service_info = ServiceInfo::<T> {
				id: service_id.clone(),
				service_type,
				service_endpoint,
			};

			// Add service (check if already exists)
			let service_exists = identity_info.services.iter().any(|s| s.id == service_id);
			ensure!(!service_exists, Error::<T>::InvalidService);

			// Add service to identity
			identity_info.services.try_push(service_info)
				.map_err(|_| Error::<T>::MaxCredentialsReached)?;
			identity_info.updated = frame_system::Pallet::<T>::block_number();

			// Store updated identity
			DecentralizedIds::<T>::insert(&did, identity_info);

			// Emit event
			Self::deposit_event(Event::ServiceAdded {
				did: did.clone(),
				service_id: service_id.clone(),
				controller: who,
			});

			Ok(())
		}
	}
}