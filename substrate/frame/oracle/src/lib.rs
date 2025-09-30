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
	use sp_runtime::traits::{CheckedAdd, CheckedSub, CheckedMul, CheckedDiv, Zero};
	use sp_arithmetic::fixed_point::FixedU128;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Native currency for oracle rewards
		type Currency: fungible::Inspect<Self::AccountId> + fungible::Mutate<Self::AccountId>;

		/// Maximum length of oracle feed name
		#[pallet::constant]
		type MaxFeedNameLength: Get<u32>;

		/// Maximum length of data key
		#[pallet::constant]
		type MaxDataKeyLength: Get<u32>;

		/// Maximum oracle operators
		#[pallet::constant]
		type MaxOracleOperators: Get<u32>;

		/// Minimum oracle operators required
		#[pallet::constant]
		type MinOracleOperators: Get<u32>;

		/// Oracle reward amount
		#[pallet::constant]
		type OracleReward: Get<u128>;

		/// Maximum data value length
		#[pallet::constant]
		type MaxDataValueLength: Get<u32>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Oracle feeds storage
	#[pallet::storage]
	#[pallet::getter(fn oracle_feeds)]
	pub type OracleFeeds<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxFeedNameLength>,
		OracleFeed<T>,
		OptionQuery,
	>;

	/// Oracle data storage
	#[pallet::storage]
	#[pallet::getter(fn oracle_data)]
	pub type OracleData<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxFeedNameLength>,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxDataKeyLength>,
		OracleValue<T>,
		OptionQuery,
	>;

	/// Oracle operators storage
	#[pallet::storage]
	#[pallet::getter(fn oracle_operators)]
	pub type OracleOperators<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxFeedNameLength>,
		BoundedVec<T::AccountId, T::MaxOracleOperators>,
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
		BoundedVec<u8, T::MaxFeedNameLength>,
		u128,
		OptionQuery,
	>;

	/// Oracle feed information
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct OracleFeed<T: Config> {
		/// Feed name
		pub name: BoundedVec<u8, T::MaxFeedNameLength>,
		/// Feed description
		pub description: BoundedVec<u8, T::MaxDataKeyLength>,
		/// Feed owner
		pub owner: T::AccountId,
		/// Required operator count
		pub required_operators: u32,
		/// Maximum staleness (blocks)
		pub max_staleness: T::BlockNumber,
		/// Minimum submissions required
		pub min_submissions: u32,
		/// Reward per submission
		pub reward_amount: u128,
		/// Active status
		pub active: bool,
		/// Creation timestamp
		pub created: T::BlockNumber,
	}

	/// Oracle value information
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct OracleValue<T: Config> {
		/// Feed name
		pub feed_name: BoundedVec<u8, T::MaxFeedNameLength>,
		/// Data key
		pub key: BoundedVec<u8, T::MaxDataKeyLength>,
		/// Current value
		pub value: BoundedVec<u8, T::MaxDataValueLength>,
		/// Value timestamp
		pub timestamp: T::BlockNumber,
		/// Submission round
		pub round: u64,
		/// Aggregated value (for multiple submissions)
		pub aggregated_value: u128,
		/// Submission count
		pub submission_count: u32,
		/// Operator submissions
		pub operator_submissions: BoundedVec<(T::AccountId, u128), T::MaxOracleOperators>,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New oracle feed created
		OracleFeedCreated {
			feed_name: BoundedVec<u8, T::MaxFeedNameLength>,
			owner: T::AccountId,
			required_operators: u32,
		},
		/// Oracle operator added
		OracleOperatorAdded {
			feed_name: BoundedVec<u8, T::MaxFeedNameLength>,
			operator: T::AccountId,
		},
		/// Oracle data submitted
		OracleDataSubmitted {
			feed_name: BoundedVec<u8, T::MaxFeedNameLength>,
			key: BoundedVec<u8, T::MaxDataKeyLength>,
			operator: T::AccountId,
			value: u128,
		},
		/// Oracle value updated
		OracleValueUpdated {
			feed_name: BoundedVec<u8, T::MaxFeedNameLength>,
			key: BoundedVec<u8, T::MaxDataKeyLength>,
			value: u128,
			timestamp: T::BlockNumber,
		},
		/// Oracle operator rewarded
		OracleOperatorRewarded {
			operator: T::AccountId,
			feed_name: BoundedVec<u8, T::MaxFeedNameLength>,
			amount: u128,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Oracle feed already exists
		OracleFeedAlreadyExists,
		/// Oracle feed not found
		OracleFeedNotFound,
		/// Oracle data not found
		OracleDataNotFound,
		/// Not authorized to perform this action
		NotAuthorized,
		/// Feed name too long
		FeedNameTooLong,
		/// Data key too long
		DataKeyTooLong,
		/// Data value too long
		DataValueTooLong,
		/// Operator already exists
		OperatorAlreadyExists,
		/// Operator not found
		OperatorNotFound,
		/// Insufficient operators
		InsufficientOperators,
		/// Maximum operators reached
		MaxOperatorsReached,
		/// Data too stale
		DataTooStale,
		/// Insufficient stake
		InsufficientStake,
		/// Invalid operator count
		InvalidOperatorCount,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create new oracle feed
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_oracle_feed(
			origin: OriginFor<T>,
			feed_name: Vec<u8>,
			description: Vec<u8>,
			required_operators: u32,
			max_staleness: T::BlockNumber,
			min_submissions: u32,
			reward_amount: u128,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let feed_name = BoundedVec::<u8, T::MaxFeedNameLength>::try_from(feed_name.clone())
				.map_err(|_| Error::<T>::FeedNameTooLong)?;
			let description = BoundedVec::<u8, T::MaxDataKeyLength>::try_from(description)
				.map_err(|_| Error::<T>::DataKeyTooLong)?;

			// Validate parameters
			ensure!(required_operators >= T::MinOracleOperators::get(), Error::<T>::InsufficientOperators);
			ensure!(required_operators <= T::MaxOracleOperators::get(), Error::<T>::MaxOperatorsReached);
			ensure!(min_submissions <= required_operators, Error::<T>::InvalidOperatorCount);

			// Check if feed already exists
			ensure!(!OracleFeeds::<T>::contains_key(&feed_name), Error::<T>::OracleFeedAlreadyExists);

			// Create oracle feed
			let oracle_feed = OracleFeed::<T> {
				name: feed_name.clone(),
				description,
				owner: who.clone(),
				required_operators,
				max_staleness,
				min_submissions,
				reward_amount,
				active: true,
				created: frame_system::Pallet::<T>::block_number(),
			};

			// Store oracle feed
			OracleFeeds::<T>::insert(&feed_name, oracle_feed);

			// Initialize empty operator list
			let empty_operators = BoundedVec::<T::AccountId, T::MaxOracleOperators>::new();
			OracleOperators::<T>::insert(&feed_name, empty_operators);

			// Emit event
			Self::deposit_event(Event::OracleFeedCreated {
				feed_name: feed_name.clone(),
				owner: who,
				required_operators,
			});

			Ok(())
		}

		/// Add oracle operator to feed
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn add_oracle_operator(
			origin: OriginFor<T>,
			feed_name: Vec<u8>,
			operator: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vector
			let feed_name = BoundedVec::<u8, T::MaxFeedNameLength>::try_from(feed_name.clone())
				.map_err(|_| Error::<T>::FeedNameTooLong)?;

			// Get oracle feed
			let oracle_feed = OracleFeeds::<T>::get(&feed_name)
				.ok_or(Error::<T>::OracleFeedNotFound)?;

			// Check authorization (only feed owner)
			ensure!(oracle_feed.owner == who, Error::<T>::NotAuthorized);

			// Get current operators
			let mut operators = OracleOperators::<T>::get(&feed_name)
				.unwrap_or_default();

			// Check if operator already exists
			ensure!(!operators.contains(&operator), Error::<T>::OperatorAlreadyExists);

			// Check maximum operators
			ensure!(operators.len() < T::MaxOracleOperators::get() as usize, Error::<T>::MaxOperatorsReached);

			// Add operator
			operators.try_push(operator.clone())
				.map_err(|_| Error::<T>::MaxOperatorsReached)?;

			// Store updated operators
			OracleOperators::<T>::insert(&feed_name, operators);

			// Emit event
			Self::deposit_event(Event::OracleOperatorAdded {
				feed_name: feed_name.clone(),
				operator: operator.clone(),
			});

			Ok(())
		}

		/// Submit oracle data
		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2, 2).ref_time())]
		pub fn submit_oracle_data(
			origin: OriginFor<T>,
			feed_name: Vec<u8>,
			key: Vec<u8>,
			value: Vec<u8>,
			raw_value: u128,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let feed_name = BoundedVec::<u8, T::MaxFeedNameLength>::try_from(feed_name.clone())
				.map_err(|_| Error::<T>::FeedNameTooLong)?;
			let key = BoundedVec::<u8, T::MaxDataKeyLength>::try_from(key.clone())
				.map_err(|_| Error::<T>::DataKeyTooLong)?;
			let value = BoundedVec::<u8, T::MaxDataValueLength>::try_from(value)
				.map_err(|_| Error::<T>::DataValueTooLong)?;

			// Get oracle feed
			let oracle_feed = OracleFeeds::<T>::get(&feed_name)
				.ok_or(Error::<T>::OracleFeedNotFound)?;

			// Check if feed is active
			ensure!(oracle_feed.active, Error::<T>::OracleFeedNotFound);

			// Get operators
			let operators = OracleOperators::<T>::get(&feed_name)
				.unwrap_or_default();

			// Check if caller is authorized operator
			ensure!(operators.contains(&who), Error::<T>::NotAuthorized);

			// Get current oracle data
			let current_block = frame_system::Pallet::<T>::block_number();
			let mut oracle_value = OracleData::<T>::get(&feed_name, &key)
				.unwrap_or(OracleValue::<T> {
					feed_name: feed_name.clone(),
					key: key.clone(),
					value: BoundedVec::<u8, T::MaxDataValueLength>::new(),
					timestamp: T::BlockNumber::zero(),
					round: 0,
					aggregated_value: 0,
					submission_count: 0,
					operator_submissions: BoundedVec::<(T::AccountId, u128), T::MaxOracleOperators>::new(),
				});

			// Check if data is too stale (if not first submission)
			if oracle_value.timestamp > T::BlockNumber::zero() {
				ensure!(current_block - oracle_value.timestamp <= oracle_feed.max_staleness, Error::<T>::DataTooStale);
			}

			// Add or update operator submission
			let mut found = false;
			for i in 0..oracle_value.operator_submissions.len() {
				if oracle_value.operator_submissions[i].0 == who {
					oracle_value.operator_submissions[i] = (who.clone(), raw_value);
					found = true;
					break;
				}
			}

			if !found {
				oracle_value.operator_submissions.try_push((who.clone(), raw_value))
					.map_err(|_| Error::<T>::MaxOperatorsReached)?;
			}

			// Update oracle value
			oracle_value.value = value;
			oracle_value.timestamp = current_block;
			oracle_value.submission_count = oracle_value.operator_submissions.len() as u32;

			// Calculate aggregated value (median of submissions)
			if oracle_value.operator_submissions.len() >= oracle_feed.min_submissions as usize {
				let mut values: Vec<u128> = oracle_value.operator_submissions.iter()
					.map(|(_, val)| *val)
					.collect();
				values.sort();

				let mid = values.len() / 2;
				oracle_value.aggregated_value = if values.len() % 2 == 0 {
					(values[mid-1] + values[mid]) / 2
				} else {
					values[mid]
				};
				oracle_value.round += 1;
			}

			// Store updated oracle data
			OracleData::<T>::insert(&feed_name, &key, oracle_value);

			// Reward operator
			T::Currency::mint_into(&who, T::OracleReward::get())?;

			// Emit events
			Self::deposit_event(Event::OracleDataSubmitted {
				feed_name: feed_name.clone(),
				key: key.clone(),
				operator: who.clone(),
				value: raw_value,
			});

			Self::deposit_event(Event::OracleValueUpdated {
				feed_name: feed_name.clone(),
				key: key.clone(),
				value: oracle_value.aggregated_value,
				timestamp: current_block,
			});

			Self::deposit_event(Event::OracleOperatorRewarded {
				operator: who,
				feed_name: feed_name.clone(),
				amount: T::OracleReward::get(),
			});

			Ok(())
		}

		/// Get oracle value (helper function for other pallets)
		#[pallet::call_index(3)]
		#[pallet::weight(1_000 + T::DbWeight::get().reads(1).ref_time())]
		pub fn get_oracle_value(
			_origin: OriginFor<T>,
			feed_name: Vec<u8>,
			key: Vec<u8>,
		) -> DispatchResult {
			// Convert to bounded vectors
			let feed_name = BoundedVec::<u8, T::MaxFeedNameLength>::try_from(feed_name)
				.map_err(|_| Error::<T>::FeedNameTooLong)?;
			let key = BoundedVec::<u8, T::MaxDataKeyLength>::try_from(key)
				.map_err(|_| Error::<T>::DataKeyTooLong)?;

			// Get oracle value
			let oracle_value = OracleData::<T>::get(&feed_name, &key)
				.ok_or(Error::<T>::OracleDataNotFound)?;

			// This is a read-only operation, mainly for compatibility
			// The actual value can be accessed via storage getters

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Get the median value from operator submissions
		pub fn get_median_value(feed_name: &BoundedVec<u8, T::MaxFeedNameLength>, key: &BoundedVec<u8, T::MaxDataKeyLength>) -> Option<u128> {
			if let Some(oracle_value) = OracleData::<T>::get(feed_name, key) {
				if oracle_value.operator_submissions.len() > 0 {
					let mut values: Vec<u128> = oracle_value.operator_submissions.iter()
						.map(|(_, val)| *val)
						.collect();
					values.sort();

					let mid = values.len() / 2;
					Some(if values.len() % 2 == 0 {
						(values[mid-1] + values[mid]) / 2
					} else {
						values[mid]
					})
				} else {
					None
				}
			} else {
				None
			}
		}

		/// Check if oracle data is fresh
		pub fn is_data_fresh(feed_name: &BoundedVec<u8, T::MaxFeedNameLength>, key: &BoundedVec<u8, T::MaxDataKeyLength>) -> bool {
			if let Some(oracle_value) = OracleData::<T>::get(feed_name, key) {
				if let Some(feed) = OracleFeeds::<T>::get(feed_name) {
					let current_block = frame_system::Pallet::<T>::block_number();
					current_block - oracle_value.timestamp <= feed.max_staleness
				} else {
					false
				}
			} else {
				false
			}
		}
	}
}