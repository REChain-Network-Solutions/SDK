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

		/// Maximum length of proof identifier
		#[pallet::constant]
		type MaxProofIdLength: Get<u32>;

		/// Maximum length of verification key
		#[pallet::constant]
		type MaxVerificationKeyLength: Get<u32>;

		/// Maximum length of proof data
		#[pallet::constant]
		type MaxProofDataLength: Get<u32>;

		/// Maximum length of public inputs
		#[pallet::constant]
		type MaxPublicInputsLength: Get<u32>;

		/// Gas limit for proof verification
		#[pallet::constant]
		type ProofVerificationGasLimit: Get<u64>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// ZKP verification keys storage
	#[pallet::storage]
	#[pallet::getter(fn verification_keys)]
	pub type VerificationKeys<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxProofIdLength>,
		VerificationKeyInfo<T>,
		OptionQuery,
	>;

	/// ZKP proofs storage
	#[pallet::storage]
	#[pallet::getter(fn verified_proofs)]
	pub type VerifiedProofs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxProofIdLength>,
		VerifiedProof<T>,
		OptionQuery,
	>;

	/// Verification key information
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct VerificationKeyInfo<T: Config> {
		/// Proof identifier
		pub proof_id: BoundedVec<u8, T::MaxProofIdLength>,
		/// Verification key data
		pub verification_key: BoundedVec<u8, T::MaxVerificationKeyLength>,
		/// Key owner
		pub owner: T::AccountId,
		/// Circuit description
		pub description: BoundedVec<u8, T::MaxProofIdLength>,
		/// Supported curve type
		pub curve_type: CurveType,
		/// Creation timestamp
		pub created: T::BlockNumber,
		/// Active status
		pub active: bool,
	}

	/// Curve types supported
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum CurveType {
		/// BLS12-381 curve
		BLS12381,
		/// BN254 curve
		BN254,
		/// BW6-761 curve
		BW6761,
		/// Custom curve
		Custom,
	}

	/// Verified proof information
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct VerifiedProof<T: Config> {
		/// Proof identifier
		pub proof_id: BoundedVec<u8, T::MaxProofIdLength>,
		/// Proof submitter
		pub submitter: T::AccountId,
		/// Proof data
		pub proof_data: BoundedVec<u8, T::MaxProofDataLength>,
		/// Public inputs
		pub public_inputs: BoundedVec<u8, T::MaxPublicInputsLength>,
		/// Verification timestamp
		pub verified_at: T::BlockNumber,
		/// Verification gas used
		pub gas_used: u64,
		/// Verification result
		pub verified: bool,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New verification key registered
		VerificationKeyRegistered {
			proof_id: BoundedVec<u8, T::MaxProofIdLength>,
			owner: T::AccountId,
			curve_type: CurveType,
		},
		/// ZKP submitted for verification
		ZKPSubmitted {
			proof_id: BoundedVec<u8, T::MaxProofIdLength>,
			submitter: T::AccountId,
		},
		/// ZKP verification completed
		ZKPVerified {
			proof_id: BoundedVec<u8, T::MaxProofIdLength>,
			submitter: T::AccountId,
			verified: bool,
			gas_used: u64,
		},
		/// Verification key updated
		VerificationKeyUpdated {
			proof_id: BoundedVec<u8, T::MaxProofIdLength>,
			owner: T::AccountId,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Verification key already exists
		VerificationKeyAlreadyExists,
		/// Verification key not found
		VerificationKeyNotFound,
		/// Proof already exists
		ProofAlreadyExists,
		/// Proof not found
		ProofNotFound,
		/// Not authorized to perform this action
		NotAuthorized,
		/// Proof identifier too long
		ProofIdTooLong,
		/// Verification key too long
		VerificationKeyTooLong,
		/// Proof data too long
		ProofDataTooLong,
		/// Public inputs too long
		PublicInputsTooLong,
		/// Verification failed
		VerificationFailed,
		/// Gas limit exceeded
		GasLimitExceeded,
		/// Unsupported curve type
		UnsupportedCurve,
		/// Invalid proof format
		InvalidProofFormat,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Register new verification key
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn register_verification_key(
			origin: OriginFor<T>,
			proof_id: Vec<u8>,
			verification_key: Vec<u8>,
			description: Vec<u8>,
			curve_type: CurveType,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let proof_id = BoundedVec::<u8, T::MaxProofIdLength>::try_from(proof_id.clone())
				.map_err(|_| Error::<T>::ProofIdTooLong)?;
			let verification_key = BoundedVec::<u8, T::MaxVerificationKeyLength>::try_from(verification_key)
				.map_err(|_| Error::<T>::VerificationKeyTooLong)?;
			let description = BoundedVec::<u8, T::MaxProofIdLength>::try_from(description)
				.map_err(|_| Error::<T>::ProofIdTooLong)?;

			// Check if verification key already exists
			ensure!(!VerificationKeys::<T>::contains_key(&proof_id), Error::<T>::VerificationKeyAlreadyExists);

			// Create verification key info
			let vk_info = VerificationKeyInfo::<T> {
				proof_id: proof_id.clone(),
				verification_key,
				owner: who.clone(),
				description,
				curve_type: curve_type.clone(),
				created: frame_system::Pallet::<T>::block_number(),
				active: true,
			};

			// Store verification key
			VerificationKeys::<T>::insert(&proof_id, vk_info);

			// Emit event
			Self::deposit_event(Event::VerificationKeyRegistered {
				proof_id: proof_id.clone(),
				owner: who,
				curve_type,
			});

			Ok(())
		}

		/// Submit ZKP for verification
		#[pallet::call_index(1)]
		#[pallet::weight(T::ProofVerificationGasLimit::get().ref_time())]
		pub fn submit_zkp(
			origin: OriginFor<T>,
			proof_id: Vec<u8>,
			proof_data: Vec<u8>,
			public_inputs: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let proof_id = BoundedVec::<u8, T::MaxProofIdLength>::try_from(proof_id.clone())
				.map_err(|_| Error::<T>::ProofIdTooLong)?;
			let proof_data = BoundedVec::<u8, T::MaxProofDataLength>::try_from(proof_data.clone())
				.map_err(|_| Error::<T>::ProofDataTooLong)?;
			let public_inputs = BoundedVec::<u8, T::MaxPublicInputsLength>::try_from(public_inputs)
				.map_err(|_| Error::<T>::PublicInputsTooLong)?;

			// Get verification key
			let vk_info = VerificationKeys::<T>::get(&proof_id)
				.ok_or(Error::<T>::VerificationKeyNotFound)?;

			// Check if verification key is active
			ensure!(vk_info.active, Error::<T>::VerificationKeyNotFound);

			// Check if proof already submitted
			ensure!(!VerifiedProofs::<T>::contains_key(&proof_id), Error::<T>::ProofAlreadyExists);

			// Perform ZKP verification (simplified for this example)
			let verification_result = Self::verify_proof(&proof_id, &proof_data, &public_inputs, &vk_info.curve_type)?;

			// Create verified proof record
			let verified_proof = VerifiedProof::<T> {
				proof_id: proof_id.clone(),
				submitter: who.clone(),
				proof_data,
				public_inputs,
				verified_at: frame_system::Pallet::<T>::block_number(),
				gas_used: T::ProofVerificationGasLimit::get(),
				verified: verification_result,
			};

			// Store verified proof
			VerifiedProofs::<T>::insert(&proof_id, verified_proof);

			// Emit event
			Self::deposit_event(Event::ZKPSubmitted {
				proof_id: proof_id.clone(),
				submitter: who.clone(),
			});

			Self::deposit_event(Event::ZKPVerified {
				proof_id: proof_id.clone(),
				submitter: who,
				verified: verification_result,
				gas_used: T::ProofVerificationGasLimit::get(),
			});

			Ok(())
		}

		/// Update verification key
		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn update_verification_key(
			origin: OriginFor<T>,
			proof_id: Vec<u8>,
			new_verification_key: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let proof_id = BoundedVec::<u8, T::MaxProofIdLength>::try_from(proof_id.clone())
				.map_err(|_| Error::<T>::ProofIdTooLong)?;
			let new_verification_key = BoundedVec::<u8, T::MaxVerificationKeyLength>::try_from(new_verification_key)
				.map_err(|_| Error::<T>::VerificationKeyTooLong)?;

			// Get existing verification key
			let mut vk_info = VerificationKeys::<T>::get(&proof_id)
				.ok_or(Error::<T>::VerificationKeyNotFound)?;

			// Check authorization (only owner)
			ensure!(vk_info.owner == who, Error::<T>::NotAuthorized);

			// Update verification key
			vk_info.verification_key = new_verification_key;
			vk_info.active = true; // Reactivate if it was deactivated

			// Store updated verification key
			VerificationKeys::<T>::insert(&proof_id, vk_info);

			// Emit event
			Self::deposit_event(Event::VerificationKeyUpdated {
				proof_id: proof_id.clone(),
				owner: who,
			});

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Verify zero-knowledge proof (simplified implementation)
		pub fn verify_proof(
			_proof_id: &BoundedVec<u8, T::MaxProofIdLength>,
			_proof_data: &BoundedVec<u8, T::MaxProofDataLength>,
			_public_inputs: &BoundedVec<u8, T::MaxPublicInputsLength>,
			curve_type: &CurveType,
		) -> Result<bool, Error<T>> {
			// In a real implementation, this would call the appropriate ZKP verification library
			// For this example, we'll simulate verification based on curve type

			match curve_type {
				CurveType::BLS12381 => {
					// Simulate BLS12-381 verification
					// In practice, this would call a BLS12-381 verification function
					Ok(true) // Simplified: assume verification succeeds
				},
				CurveType::BN254 => {
					// Simulate BN254 verification
					// In practice, this would call a BN254 verification function
					Ok(true) // Simplified: assume verification succeeds
				},
				CurveType::BW6761 => {
					// Simulate BW6-761 verification
					// In practice, this would call a BW6-761 verification function
					Ok(true) // Simplified: assume verification succeeds
				},
				CurveType::Custom => {
					// For custom curves, verification logic would be implementation-specific
					Ok(true) // Simplified: assume verification succeeds
				},
			}
		}

		/// Get verification status for a proof
		pub fn get_proof_status(proof_id: &BoundedVec<u8, T::MaxProofIdLength>) -> Option<bool> {
			VerifiedProofs::<T>::get(proof_id).map(|proof| proof.verified)
		}

		/// Check if verification key exists and is active
		pub fn is_verification_key_active(proof_id: &BoundedVec<u8, T::MaxProofIdLength>) -> bool {
			if let Some(vk_info) = VerificationKeys::<T>::get(proof_id) {
				vk_info.active
			} else {
				false
			}
		}
	}
}