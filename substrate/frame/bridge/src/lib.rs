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

		/// Maximum length of chain identifier
		#[pallet::constant]
		type MaxChainIdLength: Get<u32>;

		/// Maximum length of asset identifier
		#[pallet::constant]
		type MaxAssetIdLength: Get<u32>;

		/// Maximum number of validators per bridge
		#[pallet::constant]
		type MaxValidatorsPerBridge: Get<u32>;

		/// Minimum validators required for bridge operation
		#[pallet::constant]
		type MinValidators: Get<u32>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Bridge configurations storage
	#[pallet::storage]
	#[pallet::getter(fn bridge_configs)]
	pub type BridgeConfigs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxChainIdLength>,
		BridgeConfig<T>,
		OptionQuery,
	>;

	/// Asset mappings between chains
	#[pallet::storage]
	#[pallet::getter(fn asset_mappings)]
	pub type AssetMappings<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxChainIdLength>, // Source chain
		Blake2_128Concat,
		BoundedVec<u8, T::MaxAssetIdLength>, // Source asset
		AssetMapping<T>,
		OptionQuery,
	>;

	/// Pending cross-chain transfers
	#[pallet::storage]
	#[pallet::getter(fn pending_transfers)]
	pub type PendingTransfers<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxAssetIdLength>, // Transfer ID
		CrossChainTransfer<T>,
		OptionQuery,
	>;

	/// Bridge validators
	#[pallet::storage]
	#[pallet::getter(fn bridge_validators)]
	pub type BridgeValidators<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxChainIdLength>,
		BoundedVec<T::AccountId, T::MaxValidatorsPerBridge>,
		OptionQuery,
	>;

	/// Bridge configuration
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct BridgeConfig<T: Config> {
		/// Bridge name/identifier
		pub name: BoundedVec<u8, T::MaxChainIdLength>,
		/// Bridge operator
		pub operator: T::AccountId,
		/// Target chain ID
		pub target_chain: BoundedVec<u8, T::MaxChainIdLength>,
		/// Bridge status
		pub active: bool,
		/// Relayer threshold (minimum signatures required)
		pub threshold: u32,
		/// Fee structure
		pub fee_percentage: u32,
		/// Creation timestamp
		pub created: T::BlockNumber,
	}

	/// Asset mapping information
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct AssetMapping<T: Config> {
		/// Source chain asset ID
		pub source_asset: BoundedVec<u8, T::MaxAssetIdLength>,
		/// Target chain asset ID
		pub target_asset: BoundedVec<u8, T::MaxAssetIdLength>,
		/// Conversion rate (source to target)
		pub conversion_rate: u128,
		/// Bridge fee
		pub bridge_fee: u128,
		/// Minimum transfer amount
		pub min_transfer: u128,
		/// Maximum transfer amount
		pub max_transfer: u128,
		/// Active status
		pub active: bool,
	}

	/// Cross-chain transfer information
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct CrossChainTransfer<T: Config> {
		/// Source chain
		pub source_chain: BoundedVec<u8, T::MaxChainIdLength>,
		/// Target chain
		pub target_chain: BoundedVec<u8, T::MaxChainIdLength>,
		/// Source asset
		pub source_asset: BoundedVec<u8, T::MaxAssetIdLength>,
		/// Target asset
		pub target_asset: BoundedVec<u8, T::MaxAssetIdLength>,
		/// Sender address
		pub sender: T::AccountId,
		/// Recipient address (on target chain)
		pub recipient: BoundedVec<u8, T::MaxAssetIdLength>,
		/// Transfer amount
		pub amount: u128,
		/// Fee amount
		pub fee: u128,
		/// Transfer status
		pub status: TransferStatus,
		/// Initiated timestamp
		pub initiated_at: T::BlockNumber,
		/// Completed timestamp
		pub completed_at: Option<T::BlockNumber>,
	}

	/// Transfer status enumeration
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum TransferStatus {
		/// Transfer initiated
		Initiated,
		/// Transfer confirmed by validators
		Confirmed,
		/// Transfer completed
		Completed,
		/// Transfer failed
		Failed,
		/// Transfer cancelled
		Cancelled,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new bridge was created
		BridgeCreated {
			bridge_id: BoundedVec<u8, T::MaxChainIdLength>,
			operator: T::AccountId,
			target_chain: BoundedVec<u8, T::MaxChainIdLength>,
		},
		/// Bridge configuration was updated
		BridgeUpdated {
			bridge_id: BoundedVec<u8, T::MaxChainIdLength>,
			operator: T::AccountId,
		},
		/// Asset mapping was created
		AssetMappingCreated {
			source_chain: BoundedVec<u8, T::MaxChainIdLength>,
			source_asset: BoundedVec<u8, T::MaxAssetIdLength>,
			target_asset: BoundedVec<u8, T::MaxAssetIdLength>,
		},
		/// Cross-chain transfer initiated
		CrossChainTransferInitiated {
			transfer_id: BoundedVec<u8, T::MaxAssetIdLength>,
			sender: T::AccountId,
			source_chain: BoundedVec<u8, T::MaxChainIdLength>,
			target_chain: BoundedVec<u8, T::MaxChainIdLength>,
			amount: u128,
		},
		/// Cross-chain transfer confirmed
		CrossChainTransferConfirmed {
			transfer_id: BoundedVec<u8, T::MaxAssetIdLength>,
			validator: T::AccountId,
		},
		/// Cross-chain transfer completed
		CrossChainTransferCompleted {
			transfer_id: BoundedVec<u8, T::MaxAssetIdLength>,
			recipient: BoundedVec<u8, T::MaxAssetIdLength>,
		},
		/// Validator added to bridge
		ValidatorAdded {
			bridge_id: BoundedVec<u8, T::MaxChainIdLength>,
			validator: T::AccountId,
		},
		/// Validator removed from bridge
		ValidatorRemoved {
			bridge_id: BoundedVec<u8, T::MaxChainIdLength>,
			validator: T::AccountId,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Bridge already exists
		BridgeAlreadyExists,
		/// Bridge does not exist
		BridgeNotFound,
		/// Asset mapping already exists
		AssetMappingAlreadyExists,
		/// Asset mapping does not exist
		AssetMappingNotFound,
		/// Transfer does not exist
		TransferNotFound,
		/// Not authorized to perform this action
		NotAuthorized,
		/// Bridge is not active
		BridgeInactive,
		/// Insufficient validators
		InsufficientValidators,
		/// Invalid transfer amount
		InvalidAmount,
		/// Transfer already processed
		TransferAlreadyProcessed,
		/// Chain ID too long
		ChainIdTooLong,
		/// Asset ID too long
		AssetIdTooLong,
		/// Maximum validators reached
		MaxValidatorsReached,
		/// Validator not found
		ValidatorNotFound,
		/// Threshold too high
		ThresholdTooHigh,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new cross-chain bridge
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_bridge(
			origin: OriginFor<T>,
			bridge_id: Vec<u8>,
			target_chain: Vec<u8>,
			threshold: u32,
			fee_percentage: u32,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let bridge_id = BoundedVec::<u8, T::MaxChainIdLength>::try_from(bridge_id.clone())
				.map_err(|_| Error::<T>::ChainIdTooLong)?;
			let target_chain = BoundedVec::<u8, T::MaxChainIdLength>::try_from(target_chain)
				.map_err(|_| Error::<T>::ChainIdTooLong)?;

			// Validate threshold
			ensure!(threshold >= T::MinValidators::get(), Error::<T>::InsufficientValidators);
			ensure!(threshold <= T::MaxValidatorsPerBridge::get(), Error::<T>::ThresholdTooHigh);

			// Check if bridge already exists
			ensure!(!BridgeConfigs::<T>::contains_key(&bridge_id), Error::<T>::BridgeAlreadyExists);

			// Create bridge configuration
			let bridge_config = BridgeConfig::<T> {
				name: bridge_id.clone(),
				operator: who.clone(),
				target_chain: target_chain.clone(),
				active: true,
				threshold,
				fee_percentage,
				created: frame_system::Pallet::<T>::block_number(),
			};

			// Store bridge configuration
			BridgeConfigs::<T>::insert(&bridge_id, bridge_config);

			// Initialize empty validator list
			let empty_validators = BoundedVec::<T::AccountId, T::MaxValidatorsPerBridge>::new();
			BridgeValidators::<T>::insert(&bridge_id, empty_validators);

			// Emit event
			Self::deposit_event(Event::BridgeCreated {
				bridge_id: bridge_id.clone(),
				operator: who,
				target_chain: target_chain.clone(),
			});

			Ok(())
		}

		/// Add validator to bridge
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn add_bridge_validator(
			origin: OriginFor<T>,
			bridge_id: Vec<u8>,
			validator: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vector
			let bridge_id = BoundedVec::<u8, T::MaxChainIdLength>::try_from(bridge_id)
				.map_err(|_| Error::<T>::ChainIdTooLong)?;

			// Get bridge configuration
			let bridge_config = BridgeConfigs::<T>::get(&bridge_id)
				.ok_or(Error::<T>::BridgeNotFound)?;

			// Check authorization (only bridge operator)
			ensure!(bridge_config.operator == who, Error::<T>::NotAuthorized);

			// Get current validators
			let mut validators = BridgeValidators::<T>::get(&bridge_id)
				.unwrap_or_default();

			// Check if validator already exists
			ensure!(!validators.contains(&validator), Error::<T>::ValidatorNotFound);

			// Check maximum validators
			ensure!(validators.len() < T::MaxValidatorsPerBridge::get() as usize, Error::<T>::MaxValidatorsReached);

			// Add validator
			validators.try_push(validator.clone())
				.map_err(|_| Error::<T>::MaxValidatorsReached)?;

			// Store updated validators
			BridgeValidators::<T>::insert(&bridge_id, validators);

			// Emit event
			Self::deposit_event(Event::ValidatorAdded {
				bridge_id: bridge_id.clone(),
				validator,
			});

			Ok(())
		}

		/// Create asset mapping between chains
		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_asset_mapping(
			origin: OriginFor<T>,
			source_chain: Vec<u8>,
			source_asset: Vec<u8>,
			target_asset: Vec<u8>,
			conversion_rate: u128,
			bridge_fee: u128,
			min_transfer: u128,
			max_transfer: u128,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let source_chain = BoundedVec::<u8, T::MaxChainIdLength>::try_from(source_chain)
				.map_err(|_| Error::<T>::ChainIdTooLong)?;
			let source_asset = BoundedVec::<u8, T::MaxAssetIdLength>::try_from(source_asset.clone())
				.map_err(|_| Error::<T>::AssetIdTooLong)?;
			let target_asset = BoundedVec::<u8, T::MaxAssetIdLength>::try_from(target_asset)
				.map_err(|_| Error::<T>::AssetIdTooLong)?;

			// Check if mapping already exists
			ensure!(!AssetMappings::<T>::contains_key(&source_chain, &source_asset), Error::<T>::AssetMappingAlreadyExists);

			// Create asset mapping
			let asset_mapping = AssetMapping::<T> {
				source_asset: source_asset.clone(),
				target_asset: target_asset.clone(),
				conversion_rate,
				bridge_fee,
				min_transfer,
				max_transfer,
				active: true,
			};

			// Store asset mapping
			AssetMappings::<T>::insert(&source_chain, &source_asset, asset_mapping);

			// Emit event
			Self::deposit_event(Event::AssetMappingCreated {
				source_chain: source_chain.clone(),
				source_asset: source_asset.clone(),
				target_asset: target_asset.clone(),
			});

			Ok(())
		}

		/// Initiate cross-chain transfer
		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2, 2).ref_time())]
		pub fn initiate_cross_chain_transfer(
			origin: OriginFor<T>,
			bridge_id: Vec<u8>,
			source_asset: Vec<u8>,
			target_chain: Vec<u8>,
			target_asset: Vec<u8>,
			recipient: Vec<u8>,
			amount: u128,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let bridge_id = BoundedVec::<u8, T::MaxChainIdLength>::try_from(bridge_id.clone())
				.map_err(|_| Error::<T>::ChainIdTooLong)?;
			let source_asset = BoundedVec::<u8, T::MaxAssetIdLength>::try_from(source_asset.clone())
				.map_err(|_| Error::<T>::AssetIdTooLong)?;
			let target_chain = BoundedVec::<u8, T::MaxChainIdLength>::try_from(target_chain)
				.map_err(|_| Error::<T>::ChainIdTooLong)?;
			let target_asset = BoundedVec::<u8, T::MaxAssetIdLength>::try_from(target_asset)
				.map_err(|_| Error::<T>::AssetIdTooLong)?;
			let recipient = BoundedVec::<u8, T::MaxAssetIdLength>::try_from(recipient)
				.map_err(|_| Error::<T>::AssetIdTooLong)?;

			// Get bridge configuration
			let bridge_config = BridgeConfigs::<T>::get(&bridge_id)
				.ok_or(Error::<T>::BridgeNotFound)?;

			// Check if bridge is active
			ensure!(bridge_config.active, Error::<T>::BridgeInactive);

			// Get asset mapping
			let asset_mapping = AssetMappings::<T>::get(&bridge_id, &source_asset)
				.ok_or(Error::<T>::AssetMappingNotFound)?;

			// Validate transfer amount
			ensure!(amount >= asset_mapping.min_transfer, Error::<T>::InvalidAmount);
			ensure!(amount <= asset_mapping.max_transfer, Error::<T>::InvalidAmount);

			// Calculate fee
			let fee = (amount * bridge_config.fee_percentage as u128) / 10000;
			let transfer_amount = amount - fee;

			// Generate transfer ID (could use block number + hash for uniqueness)
			let transfer_id_data = (bridge_id.clone(), source_asset.clone(), who.clone(), amount).encode();
			let transfer_id_hash = sp_runtime::traits::Hash::hash(&transfer_id_data.encode());
			let mut transfer_id = BoundedVec::<u8, T::MaxAssetIdLength>::new();
			transfer_id.try_extend_from_slice(&transfer_id_hash.as_ref()[0..16])
				.map_err(|_| Error::<T>::AssetIdTooLong)?;

			// Create transfer record
			let transfer = CrossChainTransfer::<T> {
				source_chain: bridge_id.clone(),
				target_chain: target_chain.clone(),
				source_asset: source_asset.clone(),
				target_asset: target_asset.clone(),
				sender: who.clone(),
				recipient: recipient.clone(),
				amount: transfer_amount,
				fee,
				status: TransferStatus::Initiated,
				initiated_at: frame_system::Pallet::<T>::block_number(),
				completed_at: None,
			};

			// Store transfer
			PendingTransfers::<T>::insert(&transfer_id, transfer);

			// Emit event
			Self::deposit_event(Event::CrossChainTransferInitiated {
				transfer_id: transfer_id.clone(),
				sender: who,
				source_chain: bridge_id.clone(),
				target_chain: target_chain.clone(),
				amount: transfer_amount,
			});

			Ok(())
		}

		/// Confirm cross-chain transfer (validator signature)
		#[pallet::call_index(4)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn confirm_cross_chain_transfer(
			origin: OriginFor<T>,
			transfer_id: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vector
			let transfer_id = BoundedVec::<u8, T::MaxAssetIdLength>::try_from(transfer_id)
				.map_err(|_| Error::<T>::AssetIdTooLong)?;

			// Get transfer
			let mut transfer = PendingTransfers::<T>::get(&transfer_id)
				.ok_or(Error::<T>::TransferNotFound)?;

			// Check transfer status
			ensure!(transfer.status == TransferStatus::Initiated, Error::<T>::TransferAlreadyProcessed);

			// Get bridge validators
			let validators = BridgeValidators::<T>::get(&transfer.source_chain)
				.unwrap_or_default();

			// Check if caller is authorized validator
			ensure!(validators.contains(&who), Error::<T>::NotAuthorized);

			// Update transfer status to confirmed
			transfer.status = TransferStatus::Confirmed;
			transfer.completed_at = Some(frame_system::Pallet::<T>::block_number());

			// Store updated transfer
			PendingTransfers::<T>::insert(&transfer_id, transfer);

			// Emit event
			Self::deposit_event(Event::CrossChainTransferConfirmed {
				transfer_id: transfer_id.clone(),
				validator: who,
			});

			Ok(())
		}

		/// Complete cross-chain transfer
		#[pallet::call_index(5)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn complete_cross_chain_transfer(
			origin: OriginFor<T>,
			transfer_id: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vector
			let transfer_id = BoundedVec::<u8, T::MaxAssetIdLength>::try_from(transfer_id)
				.map_err(|_| Error::<T>::AssetIdTooLong)?;

			// Get transfer
			let mut transfer = PendingTransfers::<T>::get(&transfer_id)
				.ok_or(Error::<T>::TransferNotFound)?;

			// Check transfer status
			ensure!(transfer.status == TransferStatus::Confirmed, Error::<T>::TransferAlreadyProcessed);

			// Get bridge configuration
			let bridge_config = BridgeConfigs::<T>::get(&transfer.source_chain)
				.ok_or(Error::<T>::BridgeNotFound)?;

			// Check authorization (bridge operator or validator)
			let validators = BridgeValidators::<T>::get(&transfer.source_chain).unwrap_or_default();
			ensure!(bridge_config.operator == who || validators.contains(&who), Error::<T>::NotAuthorized);

			// Check minimum validator confirmations
			let confirmation_count = self::Pallet::<T>::get_transfer_confirmations(&transfer_id);
			ensure!(confirmation_count >= bridge_config.threshold, Error::<T>::InsufficientValidators);

			// Complete transfer
			transfer.status = TransferStatus::Completed;
			transfer.completed_at = Some(frame_system::Pallet::<T>::block_number());

			// Store completed transfer
			PendingTransfers::<T>::insert(&transfer_id, transfer);

			// Emit event
			Self::deposit_event(Event::CrossChainTransferCompleted {
				transfer_id: transfer_id.clone(),
				recipient: transfer.recipient.clone(),
			});

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Get the number of confirmations for a transfer
		pub fn get_transfer_confirmations(transfer_id: &BoundedVec<u8, T::MaxAssetIdLength>) -> u32 {
			// In a real implementation, this would track validator signatures
			// For now, return a placeholder
			1
		}
	}
}