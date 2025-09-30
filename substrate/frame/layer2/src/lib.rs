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
	use frame_support::{dispatch::{DispatchResult, DispatchResultWithPostInfo}, pallet_prelude::*, traits::fungible};
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;
	use sp_runtime::traits::{CheckedAdd, CheckedSub, Zero};
	use sp_arithmetic::fixed_point::FixedU128;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Native currency for Layer 2 operations
		type Currency: fungible::Inspect<Self::AccountId> + fungible::Mutate<Self::AccountId>;

		/// Maximum length of Layer 2 chain identifier
		#[pallet::constant]
		type MaxChainIdLength: Get<u32>;

		/// Maximum length of rollup name
		#[pallet::constant]
		type MaxRollupNameLength: Get<u32>;

		/// Maximum number of operators per Layer 2 solution
		#[pallet::constant]
		type MaxOperatorsPerLayer2: Get<u32>;

		/// Minimum stake required for operators
		#[pallet::constant]
		type MinOperatorStake: Get<u128>;

		/// Challenge period for fraud proofs
		#[pallet::constant]
		type ChallengePeriod: Get<Self::BlockNumber>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Layer 2 chains storage
	#[pallet::storage]
	#[pallet::getter(fn layer2_chains)]
	pub type Layer2Chains<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxChainIdLength>,
		Layer2Chain<T>,
		OptionQuery,
	>;

	/// Rollup configurations storage
	#[pallet::storage]
	#[pallet::getter(fn rollup_configs)]
	pub type RollupConfigs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxRollupNameLength>,
		RollupConfig<T>,
		OptionQuery,
	>;

	/// State channel storage
	#[pallet::storage]
	#[pallet::getter(fn state_channels)]
	pub type StateChannels<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxChainIdLength>,
		StateChannel<T>,
		OptionQuery,
	>;

	/// Layer 2 operators storage
	#[pallet::storage]
	#[pallet::getter(fn layer2_operators)]
	pub type Layer2Operators<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxChainIdLength>,
		BoundedVec<T::AccountId, T::MaxOperatorsPerLayer2>,
		OptionQuery,
	>;

	/// Operator stakes storage
	#[pallet::storage]
	#[pallet::getter(fn operator_stakes)]
	pub type OperatorStakes<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxChainIdLength>,
		u128,
		OptionQuery,
	>;

	/// Layer 2 chain information
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct Layer2Chain<T: Config> {
		/// Chain identifier
		pub chain_id: BoundedVec<u8, T::MaxChainIdLength>,
		/// Chain name
		pub name: BoundedVec<u8, T::MaxChainIdLength>,
		/// Chain type
		pub chain_type: Layer2Type,
		/// Chain operator
		pub operator: T::AccountId,
		/// Total value locked
		pub tvl: u128,
		/// Active status
		pub active: bool,
		/// Creation timestamp
		pub created: T::BlockNumber,
		/// Last update
		pub last_update: T::BlockNumber,
	}

	/// Layer 2 types
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum Layer2Type {
		/// Optimistic rollup
		OptimisticRollup,
		/// Zero-knowledge rollup
		ZKRollup,
		/// Sidechain
		Sidechain,
		/// State channel
		StateChannel,
		/// Plasma chain
		Plasma,
		/// Validium
		Validium,
	}

	/// Rollup configuration
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct RollupConfig<T: Config> {
		/// Rollup name
		pub name: BoundedVec<u8, T::MaxRollupNameLength>,
		/// Associated Layer 2 chain
		pub layer2_chain: BoundedVec<u8, T::MaxChainIdLength>,
		/// Batch size for rollup
		pub batch_size: u64,
		/// Challenge period
		pub challenge_period: T::BlockNumber,
		/// Finality period
		pub finality_period: T::BlockNumber,
		/// Gas limit per batch
		pub gas_limit: u64,
		/// Compression enabled
		pub compression_enabled: bool,
		/// Active status
		pub active: bool,
	}

	/// State channel information
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct StateChannel<T: Config> {
		/// Channel identifier
		pub channel_id: BoundedVec<u8, T::MaxChainIdLength>,
		/// Participants
		pub participants: BoundedVec<T::AccountId, ConstU32<10>>,
		/// Channel state
		pub state: BoundedVec<u8, T::MaxChainIdLength>,
		/// State nonce
		pub nonce: u64,
		/// Total channel value
		pub total_value: u128,
		/// Channel status
		pub status: ChannelStatus,
		/// Creation timestamp
		pub created: T::BlockNumber,
		/// Last update
		pub last_update: T::BlockNumber,
	}

	/// Channel status
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum ChannelStatus {
		/// Channel is open
		Open,
		/// Channel is closed
		Closed,
		/// Channel is disputed
		Disputed,
		/// Channel is finalized
		Finalized,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New Layer 2 chain created
		Layer2ChainCreated {
			chain_id: BoundedVec<u8, T::MaxChainIdLength>,
			chain_type: Layer2Type,
			operator: T::AccountId,
		},
		/// Layer 2 operator added
		Layer2OperatorAdded {
			chain_id: BoundedVec<u8, T::MaxChainIdLength>,
			operator: T::AccountId,
		},
		/// Rollup batch submitted
		RollupBatchSubmitted {
			rollup_name: BoundedVec<u8, T::MaxRollupNameLength>,
			batch_hash: BoundedVec<u8, T::MaxChainIdLength>,
			operator: T::AccountId,
		},
		/// Rollup batch finalized
		RollupBatchFinalized {
			rollup_name: BoundedVec<u8, T::MaxRollupNameLength>,
			batch_hash: BoundedVec<u8, T::MaxChainIdLength>,
		},
		/// State channel opened
		StateChannelOpened {
			channel_id: BoundedVec<u8, T::MaxChainIdLength>,
			participants: BoundedVec<T::AccountId, ConstU32<10>>,
		},
		/// State channel state updated
		StateChannelUpdated {
			channel_id: BoundedVec<u8, T::MaxChainIdLength>,
			nonce: u64,
		},
		/// State channel closed
		StateChannelClosed {
			channel_id: BoundedVec<u8, T::MaxChainIdLength>,
			final_state: BoundedVec<u8, T::MaxChainIdLength>,
		},
		/// Fraud challenge submitted
		FraudChallengeSubmitted {
			rollup_name: BoundedVec<u8, T::MaxRollupNameLength>,
			challenger: T::AccountId,
			batch_hash: BoundedVec<u8, T::MaxChainIdLength>,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Layer 2 chain already exists
		Layer2ChainAlreadyExists,
		/// Layer 2 chain not found
		Layer2ChainNotFound,
		/// Rollup configuration not found
		RollupConfigNotFound,
		/// State channel not found
		StateChannelNotFound,
		/// Operator already exists
		OperatorAlreadyExists,
		/// Operator not found
		OperatorNotFound,
		/// Insufficient stake
		InsufficientStake,
		/// Not authorized to perform this action
		NotAuthorized,
		/// Chain ID too long
		ChainIdTooLong,
		/// Rollup name too long
		RollupNameTooLong,
		/// Maximum operators reached
		MaxOperatorsReached,
		/// Channel already exists
		ChannelAlreadyExists,
		/// Invalid channel participants
		InvalidChannelParticipants,
		/// Challenge period not expired
		ChallengePeriodNotExpired,
		/// Invalid batch data
		InvalidBatchData,
		/// Insufficient funds for operation
		InsufficientFunds,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create new Layer 2 chain
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_layer2_chain(
			origin: OriginFor<T>,
			chain_id: Vec<u8>,
			name: Vec<u8>,
			chain_type: Layer2Type,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let chain_id = BoundedVec::<u8, T::MaxChainIdLength>::try_from(chain_id.clone())
				.map_err(|_| Error::<T>::ChainIdTooLong)?;
			let name = BoundedVec::<u8, T::MaxChainIdLength>::try_from(name)
				.map_err(|_| Error::<T>::ChainIdTooLong)?;

			// Check if chain already exists
			ensure!(!Layer2Chains::<T>::contains_key(&chain_id), Error::<T>::Layer2ChainAlreadyExists);

			// Create Layer 2 chain
			let layer2_chain = Layer2Chain::<T> {
				chain_id: chain_id.clone(),
				name,
				chain_type: chain_type.clone(),
				operator: who.clone(),
				tvl: 0,
				active: true,
				created: frame_system::Pallet::<T>::block_number(),
				last_update: frame_system::Pallet::<T>::block_number(),
			};

			// Store Layer 2 chain
			Layer2Chains::<T>::insert(&chain_id, layer2_chain);

			// Initialize empty operator list
			let empty_operators = BoundedVec::<T::AccountId, T::MaxOperatorsPerLayer2>::new();
			Layer2Operators::<T>::insert(&chain_id, empty_operators);

			// Emit event
			Self::deposit_event(Event::Layer2ChainCreated {
				chain_id: chain_id.clone(),
				chain_type,
				operator: who,
			});

			Ok(())
		}

		/// Add operator to Layer 2 chain
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn add_layer2_operator(
			origin: OriginFor<T>,
			chain_id: Vec<u8>,
			operator: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vector
			let chain_id = BoundedVec::<u8, T::MaxChainIdLength>::try_from(chain_id.clone())
				.map_err(|_| Error::<T>::ChainIdTooLong)?;

			// Get Layer 2 chain
			let layer2_chain = Layer2Chains::<T>::get(&chain_id)
				.ok_or(Error::<T>::Layer2ChainNotFound)?;

			// Check authorization (only chain operator)
			ensure!(layer2_chain.operator == who, Error::<T>::NotAuthorized);

			// Get current operators
			let mut operators = Layer2Operators::<T>::get(&chain_id)
				.unwrap_or_default();

			// Check if operator already exists
			ensure!(!operators.contains(&operator), Error::<T>::OperatorAlreadyExists);

			// Check maximum operators
			ensure!(operators.len() < T::MaxOperatorsPerLayer2::get() as usize, Error::<T>::MaxOperatorsReached);

			// Add operator
			operators.try_push(operator.clone())
				.map_err(|_| Error::<T>::MaxOperatorsReached)?;

			// Store updated operators
			Layer2Operators::<T>::insert(&chain_id, operators);

			// Emit event
			Self::deposit_event(Event::Layer2OperatorAdded {
				chain_id: chain_id.clone(),
				operator: operator.clone(),
			});

			Ok(())
		}

		/// Submit rollup batch
		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn submit_rollup_batch(
			origin: OriginFor<T>,
			rollup_name: Vec<u8>,
			batch_data: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let rollup_name = BoundedVec::<u8, T::MaxRollupNameLength>::try_from(rollup_name.clone())
				.map_err(|_| Error::<T>::RollupNameTooLong)?;
			let batch_hash = BoundedVec::<u8, T::MaxChainIdLength>::try_from(
				sp_runtime::traits::Hash::hash(&batch_data).as_ref().to_vec()
			).map_err(|_| Error::<T>::ChainIdTooLong)?;

			// Get rollup configuration
			let rollup_config = RollupConfigs::<T>::get(&rollup_name)
				.ok_or(Error::<T>::RollupConfigNotFound)?;

			// Check if rollup is active
			ensure!(rollup_config.active, Error::<T>::RollupConfigNotFound);

			// Get chain operators
			let operators = Layer2Operators::<T>::get(&rollup_config.layer2_chain)
				.unwrap_or_default();

			// Check if caller is authorized operator
			ensure!(operators.contains(&who), Error::<T>::NotAuthorized);

			// Emit event
			Self::deposit_event(Event::RollupBatchSubmitted {
				rollup_name: rollup_name.clone(),
				batch_hash: batch_hash.clone(),
				operator: who.clone(),
			});

			Ok(())
		}

		/// Open state channel
		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn open_state_channel(
			origin: OriginFor<T>,
			channel_id: Vec<u8>,
			participants: Vec<T::AccountId>,
			initial_state: Vec<u8>,
			total_value: u128,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let channel_id = BoundedVec::<u8, T::MaxChainIdLength>::try_from(channel_id.clone())
				.map_err(|_| Error::<T>::ChainIdTooLong)?;
			let initial_state = BoundedVec::<u8, T::MaxChainIdLength>::try_from(initial_state)
				.map_err(|_| Error::<T>::ChainIdTooLong)?;

			// Validate participants
			ensure!(participants.len() >= 2, Error::<T>::InvalidChannelParticipants);
			ensure!(participants.len() <= 10, Error::<T>::InvalidChannelParticipants);
			let participants_bounded = BoundedVec::<T::AccountId, ConstU32<10>>::try_from(participants.clone())
				.map_err(|_| Error::<T>::InvalidChannelParticipants)?;

			// Check if channel already exists
			ensure!(!StateChannels::<T>::contains_key(&channel_id), Error::<T>::ChannelAlreadyExists);

			// Verify all participants have sufficient balance
			for participant in &participants {
				ensure!(T::Currency::reducible_balance(participant, false) >= total_value / participants.len() as u128, Error::<T>::InsufficientFunds);
			}

			// Create state channel
			let state_channel = StateChannel::<T> {
				channel_id: channel_id.clone(),
				participants: participants_bounded,
				state: initial_state,
				nonce: 0,
				total_value,
				status: ChannelStatus::Open,
				created: frame_system::Pallet::<T>::block_number(),
				last_update: frame_system::Pallet::<T>::block_number(),
			};

			// Store state channel
			StateChannels::<T>::insert(&channel_id, state_channel);

			// Emit event
			Self::deposit_event(Event::StateChannelOpened {
				channel_id: channel_id.clone(),
				participants: participants_bounded,
			});

			Ok(())
		}

		/// Update state channel state
		#[pallet::call_index(4)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn update_state_channel(
			origin: OriginFor<T>,
			channel_id: Vec<u8>,
			new_state: Vec<u8>,
			nonce: u64,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let channel_id = BoundedVec::<u8, T::MaxChainIdLength>::try_from(channel_id.clone())
				.map_err(|_| Error::<T>::ChainIdTooLong)?;
			let new_state = BoundedVec::<u8, T::MaxChainIdLength>::try_from(new_state)
				.map_err(|_| Error::<T>::ChainIdTooLong)?;

			// Get state channel
			let mut state_channel = StateChannels::<T>::get(&channel_id)
				.ok_or(Error::<T>::StateChannelNotFound)?;

			// Check if channel is open
			ensure!(state_channel.status == ChannelStatus::Open, Error::<T>::InvalidChannelStatus);

			// Check if caller is participant
			ensure!(state_channel.participants.contains(&who), Error::<T>::NotAuthorized);

			// Check nonce
			ensure!(nonce > state_channel.nonce, Error::<T>::InvalidNonce);

			// Update state
			state_channel.state = new_state;
			state_channel.nonce = nonce;
			state_channel.last_update = frame_system::Pallet::<T>::block_number();

			// Store updated state channel
			StateChannels::<T>::insert(&channel_id, state_channel);

			// Emit event
			Self::deposit_event(Event::StateChannelUpdated {
				channel_id: channel_id.clone(),
				nonce,
			});

			Ok(())
		}

		/// Close state channel
		#[pallet::call_index(5)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn close_state_channel(
			origin: OriginFor<T>,
			channel_id: Vec<u8>,
			final_state: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let channel_id = BoundedVec::<u8, T::MaxChainIdLength>::try_from(channel_id.clone())
				.map_err(|_| Error::<T>::ChainIdTooLong)?;
			let final_state = BoundedVec::<u8, T::MaxChainIdLength>::try_from(final_state)
				.map_err(|_| Error::<T>::ChainIdTooLong)?;

			// Get state channel
			let mut state_channel = StateChannels::<T>::get(&channel_id)
				.ok_or(Error::<T>::StateChannelNotFound)?;

			// Check if caller is participant
			ensure!(state_channel.participants.contains(&who), Error::<T>::NotAuthorized);

			// Check if channel is open
			ensure!(state_channel.status == ChannelStatus::Open, Error::<T>::InvalidChannelStatus);

			// Close channel
			state_channel.status = ChannelStatus::Closed;
			state_channel.state = final_state.clone();
			state_channel.last_update = frame_system::Pallet::<T>::block_number();

			// Store updated state channel
			StateChannels::<T>::insert(&channel_id, state_channel);

			// Emit event
			Self::deposit_event(Event::StateChannelClosed {
				channel_id: channel_id.clone(),
				final_state: final_state.clone(),
			});

			Ok(())
		}
	}
}