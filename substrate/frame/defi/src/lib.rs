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
	use sp_arithmetic::fixed_point::{FixedU128, FixedPointNumber};

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Native currency type
		type Currency: fungible::Inspect<Self::AccountId> + fungible::Mutate<Self::AccountId> + fungible::Transfer<Self::AccountId>;

		/// Maximum length of token symbol
		#[pallet::constant]
		type MaxTokenSymbolLength: Get<u32>;

		/// Maximum length of pool name
		#[pallet::constant]
		type MaxPoolNameLength: Get<u32>;

		/// Fee numerator (parts per million)
		#[pallet::constant]
		type FeeNumerator: Get<u32>;

		/// Minimum liquidity for pool creation
		#[pallet::constant]
		type MinLiquidity: Get<u128>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Token registry storage
	#[pallet::storage]
	#[pallet::getter(fn tokens)]
	pub type Tokens<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxTokenSymbolLength>,
		TokenInfo<T>,
		OptionQuery,
	>;

	/// Liquidity pools storage
	#[pallet::storage]
	#[pallet::getter(fn liquidity_pools)]
	pub type LiquidityPools<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxPoolNameLength>,
		LiquidityPool<T>,
		OptionQuery,
	>;

	/// User liquidity positions
	#[pallet::storage]
	#[pallet::getter(fn user_positions)]
	pub type UserLiquidityPositions<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxPoolNameLength>,
		LiquidityPosition<T>,
		OptionQuery,
	>;

	/// Lending pools storage
	#[pallet::storage]
	#[pallet::getter(fn lending_pools)]
	pub type LendingPools<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxTokenSymbolLength>,
		LendingPool<T>,
		OptionQuery,
	>;

	/// User lending positions
	#[pallet::storage]
	#[pallet::getter(fn lending_positions)]
	pub type UserLendingPositions<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxTokenSymbolLength>,
		LendingPosition<T>,
		OptionQuery,
	>;

	/// Token information
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct TokenInfo<T: Config> {
		/// Token name
		pub name: BoundedVec<u8, T::MaxTokenSymbolLength>,
		/// Token symbol
		pub symbol: BoundedVec<u8, T::MaxTokenSymbolLength>,
		/// Token decimals
		pub decimals: u8,
		/// Total supply
		pub total_supply: u128,
		/// Token creator
		pub creator: T::AccountId,
		/// Creation timestamp
		pub created: T::BlockNumber,
	}

	/// Liquidity pool information
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct LiquidityPool<T: Config> {
		/// Pool name
		pub name: BoundedVec<u8, T::MaxPoolNameLength>,
		/// Token A
		pub token_a: BoundedVec<u8, T::MaxTokenSymbolLength>,
		/// Token B
		pub token_b: BoundedVec<u8, T::MaxTokenSymbolLength>,
		/// Reserve A
		pub reserve_a: u128,
		/// Reserve B
		pub reserve_b: u128,
		/// Total liquidity
		pub total_liquidity: u128,
		/// Fee numerator
		pub fee_numerator: u32,
		/// Creator
		pub creator: T::AccountId,
		/// Creation timestamp
		pub created: T::BlockNumber,
	}

	/// User liquidity position
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct LiquidityPosition<T: Config> {
		/// Pool name
		pub pool_name: BoundedVec<u8, T::MaxPoolNameLength>,
		/// Liquidity amount
		pub liquidity_amount: u128,
		/// Token A amount
		pub token_a_amount: u128,
		/// Token B amount
		pub token_b_amount: u128,
		/// Position timestamp
		pub timestamp: T::BlockNumber,
	}

	/// Lending pool information
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct LendingPool<T: Config> {
		/// Token symbol
		pub token_symbol: BoundedVec<u8, T::MaxTokenSymbolLength>,
		/// Total deposits
		pub total_deposits: u128,
		/// Total borrows
		pub total_borrows: u128,
		/// Interest rate (fixed point)
		pub interest_rate: u128,
		/// Utilization rate
		pub utilization_rate: u128,
		/// Reserve factor
		pub reserve_factor: u32,
		/// Liquidation threshold
		pub liquidation_threshold: u128,
		/// Creation timestamp
		pub created: T::BlockNumber,
	}

	/// User lending position
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct LendingPosition<T: Config> {
		/// Token symbol
		pub token_symbol: BoundedVec<u8, T::MaxTokenSymbolLength>,
		/// Deposit amount
		pub deposit_amount: u128,
		/// Borrow amount
		pub borrow_amount: u128,
		/// Interest accrued
		pub interest_accrued: u128,
		/// Last update timestamp
		pub last_update: T::BlockNumber,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New token was created
		TokenCreated {
			symbol: BoundedVec<u8, T::MaxTokenSymbolLength>,
			creator: T::AccountId,
			total_supply: u128,
		},
		/// Liquidity pool was created
		LiquidityPoolCreated {
			pool_name: BoundedVec<u8, T::MaxPoolNameLength>,
			token_a: BoundedVec<u8, T::MaxTokenSymbolLength>,
			token_b: BoundedVec<u8, T::MaxTokenSymbolLength>,
			creator: T::AccountId,
		},
		/// Liquidity was added
		LiquidityAdded {
			pool_name: BoundedVec<u8, T::MaxPoolNameLength>,
			provider: T::AccountId,
			amount_a: u128,
			amount_b: u128,
			liquidity_minted: u128,
		},
		/// Liquidity was removed
		LiquidityRemoved {
			pool_name: BoundedVec<u8, T::MaxPoolNameLength>,
			provider: T::AccountId,
			amount_a: u128,
			amount_b: u128,
			liquidity_burned: u128,
		},
		/// Token swap occurred
		TokenSwapped {
			pool_name: BoundedVec<u8, T::MaxPoolNameLength>,
			user: T::AccountId,
			token_in: BoundedVec<u8, T::MaxTokenSymbolLength>,
			token_out: BoundedVec<u8, T::MaxTokenSymbolLength>,
			amount_in: u128,
			amount_out: u128,
		},
		/// Lending pool was created
		LendingPoolCreated {
			token_symbol: BoundedVec<u8, T::MaxTokenSymbolLength>,
			creator: T::AccountId,
			interest_rate: u128,
		},
		/// Deposit made to lending pool
		DepositMade {
			token_symbol: BoundedVec<u8, T::MaxTokenSymbolLength>,
			user: T::AccountId,
			amount: u128,
		},
		/// Loan taken from lending pool
		LoanTaken {
			token_symbol: BoundedVec<u8, T::MaxTokenSymbolLength>,
			user: T::AccountId,
			amount: u128,
		},
		/// Loan repaid
		LoanRepaid {
			token_symbol: BoundedVec<u8, T::MaxTokenSymbolLength>,
			user: T::AccountId,
			amount: u128,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Token already exists
		TokenAlreadyExists,
		/// Token does not exist
		TokenNotFound,
		/// Pool already exists
		PoolAlreadyExists,
		/// Pool does not exist
		PoolNotFound,
		/// Insufficient liquidity
		InsufficientLiquidity,
		/// Insufficient balance
		InsufficientBalance,
		/// Invalid amount
		InvalidAmount,
		/// Slippage tolerance exceeded
		SlippageExceeded,
		/// Pool ratio mismatch
		PoolRatioMismatch,
		/// Lending pool does not exist
		LendingPoolNotFound,
		/// Insufficient collateral
		InsufficientCollateral,
		/// Position does not exist
		PositionNotFound,
		/// Token symbol too long
		TokenSymbolTooLong,
		/// Pool name too long
		PoolNameTooLong,
		/// Arithmetic overflow
		Overflow,
		/// Division by zero
		DivisionByZero,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new token
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_token(
			origin: OriginFor<T>,
			name: Vec<u8>,
			symbol: Vec<u8>,
			decimals: u8,
			total_supply: u128,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let name = BoundedVec::<u8, T::MaxTokenSymbolLength>::try_from(name)
				.map_err(|_| Error::<T>::TokenSymbolTooLong)?;
			let symbol = BoundedVec::<u8, T::MaxTokenSymbolLength>::try_from(symbol.clone())
				.map_err(|_| Error::<T>::TokenSymbolTooLong)?;

			// Check if token already exists
			ensure!(!Tokens::<T>::contains_key(&symbol), Error::<T>::TokenAlreadyExists);

			// Create token info
			let token_info = TokenInfo::<T> {
				name: name.clone(),
				symbol: symbol.clone(),
				decimals,
				total_supply,
				creator: who.clone(),
				created: frame_system::Pallet::<T>::block_number(),
			};

			// Store token
			Tokens::<T>::insert(&symbol, token_info);

			// Mint initial supply to creator
			T::Currency::mint_into(&who, total_supply)?;

			// Emit event
			Self::deposit_event(Event::TokenCreated {
				symbol: symbol.clone(),
				creator: who,
				total_supply,
			});

			Ok(())
		}

		/// Create liquidity pool
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_liquidity_pool(
			origin: OriginFor<T>,
			pool_name: Vec<u8>,
			token_a: Vec<u8>,
			token_b: Vec<u8>,
			amount_a: u128,
			amount_b: u128,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let pool_name = BoundedVec::<u8, T::MaxPoolNameLength>::try_from(pool_name.clone())
				.map_err(|_| Error::<T>::PoolNameTooLong)?;
			let token_a = BoundedVec::<u8, T::MaxTokenSymbolLength>::try_from(token_a.clone())
				.map_err(|_| Error::<T>::TokenSymbolTooLong)?;
			let token_b = BoundedVec::<u8, T::MaxTokenSymbolLength>::try_from(token_b.clone())
				.map_err(|_| Error::<T>::TokenSymbolTooLong)?;

			// Check if pool already exists
			ensure!(!LiquidityPools::<T>::contains_key(&pool_name), Error::<T>::PoolAlreadyExists);

			// Check minimum liquidity
			let initial_liquidity = (amount_a * amount_b).integer_sqrt();
			ensure!(initial_liquidity >= T::MinLiquidity::get(), Error::<T>::InsufficientLiquidity);

			// Check token balances
			ensure!(T::Currency::reducible_balance(&who, false) >= amount_a, Error::<T>::InsufficientBalance);

			// Create liquidity pool
			let liquidity_pool = LiquidityPool::<T> {
				name: pool_name.clone(),
				token_a: token_a.clone(),
				token_b: token_b.clone(),
				reserve_a: amount_a,
				reserve_b: amount_b,
				total_liquidity: initial_liquidity,
				fee_numerator: T::FeeNumerator::get(),
				creator: who.clone(),
				created: frame_system::Pallet::<T>::block_number(),
			};

			// Store liquidity pool
			LiquidityPools::<T>::insert(&pool_name, liquidity_pool);

			// Create user position
			let user_position = LiquidityPosition::<T> {
				pool_name: pool_name.clone(),
				liquidity_amount: initial_liquidity,
				token_a_amount: amount_a,
				token_b_amount: amount_b,
				timestamp: frame_system::Pallet::<T>::block_number(),
			};

			// Store user position
			UserLiquidityPositions::<T>::insert(&who, &pool_name, user_position);

			// Emit event
			Self::deposit_event(Event::LiquidityPoolCreated {
				pool_name: pool_name.clone(),
				token_a: token_a.clone(),
				token_b: token_b.clone(),
				creator: who,
			});

			Ok(())
		}

		/// Add liquidity to pool
		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
		pub fn add_liquidity(
			origin: OriginFor<T>,
			pool_name: Vec<u8>,
			amount_a: u128,
			amount_b: u128,
			min_liquidity: u128,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vector
			let pool_name = BoundedVec::<u8, T::MaxPoolNameLength>::try_from(pool_name.clone())
				.map_err(|_| Error::<T>::PoolNameTooLong)?;

			// Get pool
			let mut pool = LiquidityPools::<T>::get(&pool_name)
				.ok_or(Error::<T>::PoolNotFound)?;

			// Calculate optimal amounts based on current ratio
			let optimal_b = (amount_a * pool.reserve_b) / pool.reserve_a;
			ensure!(amount_b >= optimal_b, Error::<T>::PoolRatioMismatch);

			// Calculate liquidity to mint
			let liquidity_minted = (amount_a * pool.total_liquidity) / pool.reserve_a;

			// Check minimum liquidity requirement
			ensure!(liquidity_minted >= min_liquidity, Error::<T>::InsufficientLiquidity);

			// Update pool reserves
			pool.reserve_a = pool.reserve_a.checked_add(amount_a)
				.ok_or(Error::<T>::Overflow)?;
			pool.reserve_b = pool.reserve_b.checked_add(amount_b)
				.ok_or(Error::<T>::Overflow)?;
			pool.total_liquidity = pool.total_liquidity.checked_add(liquidity_minted)
				.ok_or(Error::<T>::Overflow)?;

			// Update or create user position
			let mut user_position = UserLiquidityPositions::<T>::get(&who, &pool_name)
				.unwrap_or(LiquidityPosition::<T> {
					pool_name: pool_name.clone(),
					liquidity_amount: 0,
					token_a_amount: 0,
					token_b_amount: 0,
					timestamp: frame_system::Pallet::<T>::block_number(),
				});

			user_position.liquidity_amount = user_position.liquidity_amount.checked_add(liquidity_minted)
				.ok_or(Error::<T>::Overflow)?;
			user_position.token_a_amount = user_position.token_a_amount.checked_add(amount_a)
				.ok_or(Error::<T>::Overflow)?;
			user_position.token_b_amount = user_position.token_b_amount.checked_add(amount_b)
				.ok_or(Error::<T>::Overflow)?;
			user_position.timestamp = frame_system::Pallet::<T>::block_number();

			// Store updated pool and position
			LiquidityPools::<T>::insert(&pool_name, pool);
			UserLiquidityPositions::<T>::insert(&who, &pool_name, user_position);

			// Emit event
			Self::deposit_event(Event::LiquidityAdded {
				pool_name: pool_name.clone(),
				provider: who,
				amount_a,
				amount_b,
				liquidity_minted,
			});

			Ok(())
		}

		/// Swap tokens
		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn swap_tokens(
			origin: OriginFor<T>,
			pool_name: Vec<u8>,
			token_in: Vec<u8>,
			amount_in: u128,
			min_amount_out: u128,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let pool_name = BoundedVec::<u8, T::MaxPoolNameLength>::try_from(pool_name.clone())
				.map_err(|_| Error::<T>::PoolNameTooLong)?;
			let token_in = BoundedVec::<u8, T::MaxTokenSymbolLength>::try_from(token_in)
				.map_err(|_| Error::<T>::TokenSymbolTooLong)?;

			// Get pool
			let mut pool = LiquidityPools::<T>::get(&pool_name)
				.ok_or(Error::<T>::PoolNotFound)?;

			// Determine token in/out and calculate swap
			let (reserve_in, reserve_out) = if token_in == pool.token_a {
				(pool.reserve_a, pool.reserve_b)
			} else if token_in == pool.token_b {
				(pool.reserve_b, pool.reserve_a)
			} else {
				return Err(Error::<T>::TokenNotFound.into());
			};

			// Calculate amount out using AMM formula
			let amount_in_with_fee = amount_in * (1000 - pool.fee_numerator);
			let numerator = amount_in_with_fee * reserve_out;
			let denominator = (reserve_in * 1000) + amount_in_with_fee;
			let amount_out = numerator / denominator;

			// Check slippage tolerance
			ensure!(amount_out >= min_amount_out, Error::<T>::SlippageExceeded);

			// Update reserves
			if token_in == pool.token_a {
				pool.reserve_a = pool.reserve_a.checked_add(amount_in)
					.ok_or(Error::<T>::Overflow)?;
				pool.reserve_b = pool.reserve_b.checked_sub(amount_out)
					.ok_or(Error::<T>::Overflow)?;
			} else {
				pool.reserve_b = pool.reserve_b.checked_add(amount_in)
					.ok_or(Error::<T>::Overflow)?;
				pool.reserve_a = pool.reserve_a.checked_sub(amount_out)
					.ok_or(Error::<T>::Overflow)?;
			}

			// Store updated pool
			LiquidityPools::<T>::insert(&pool_name, pool);

			// Emit event
			Self::deposit_event(Event::TokenSwapped {
				pool_name: pool_name.clone(),
				user: who,
				token_in: token_in.clone(),
				token_out: if token_in == pool.token_a { pool.token_b.clone() } else { pool.token_a.clone() },
				amount_in,
				amount_out,
			});

			Ok(())
		}

		/// Create lending pool
		#[pallet::call_index(4)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_lending_pool(
			origin: OriginFor<T>,
			token_symbol: Vec<u8>,
			interest_rate: u128,
			reserve_factor: u32,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vector
			let token_symbol = BoundedVec::<u8, T::MaxTokenSymbolLength>::try_from(token_symbol.clone())
				.map_err(|_| Error::<T>::TokenSymbolTooLong)?;

			// Check if lending pool already exists
			ensure!(!LendingPools::<T>::contains_key(&token_symbol), Error::<T>::PoolAlreadyExists);

			// Create lending pool
			let lending_pool = LendingPool::<T> {
				token_symbol: token_symbol.clone(),
				total_deposits: 0,
				total_borrows: 0,
				interest_rate,
				utilization_rate: 0,
				reserve_factor,
				liquidation_threshold: 150, // 150% collateralization ratio
				created: frame_system::Pallet::<T>::block_number(),
			};

			// Store lending pool
			LendingPools::<T>::insert(&token_symbol, lending_pool);

			// Emit event
			Self::deposit_event(Event::LendingPoolCreated {
				token_symbol: token_symbol.clone(),
				creator: who,
				interest_rate,
			});

			Ok(())
		}

		/// Deposit tokens to lending pool
		#[pallet::call_index(5)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
		pub fn deposit_to_lending_pool(
			origin: OriginFor<T>,
			token_symbol: Vec<u8>,
			amount: u128,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vector
			let token_symbol = BoundedVec::<u8, T::MaxTokenSymbolLength>::try_from(token_symbol.clone())
				.map_err(|_| Error::<T>::TokenSymbolTooLong)?;

			// Get or create lending pool
			let mut lending_pool = LendingPools::<T>::get(&token_symbol)
				.ok_or(Error::<T>::LendingPoolNotFound)?;

			// Get or create user position
			let mut user_position = UserLendingPositions::<T>::get(&who, &token_symbol)
				.unwrap_or(LendingPosition::<T> {
					token_symbol: token_symbol.clone(),
					deposit_amount: 0,
					borrow_amount: 0,
					interest_accrued: 0,
					last_update: frame_system::Pallet::<T>::block_number(),
				});

			// Update pool and position
			lending_pool.total_deposits = lending_pool.total_deposits.checked_add(amount)
				.ok_or(Error::<T>::Overflow)?;
			user_position.deposit_amount = user_position.deposit_amount.checked_add(amount)
				.ok_or(Error::<T>::Overflow)?;
			user_position.last_update = frame_system::Pallet::<T>::block_number();

			// Recalculate utilization rate
			if lending_pool.total_deposits > 0 {
				lending_pool.utilization_rate = (lending_pool.total_borrows * 1000) / lending_pool.total_deposits;
			}

			// Store updated pool and position
			LendingPools::<T>::insert(&token_symbol, lending_pool);
			UserLendingPositions::<T>::insert(&who, &token_symbol, user_position);

			// Emit event
			Self::deposit_event(Event::DepositMade {
				token_symbol: token_symbol.clone(),
				user: who,
				amount,
			});

			Ok(())
		}

		/// Borrow from lending pool
		#[pallet::call_index(6)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
		pub fn borrow_from_lending_pool(
			origin: OriginFor<T>,
			token_symbol: Vec<u8>,
			borrow_amount: u128,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vector
			let token_symbol = BoundedVec::<u8, T::MaxTokenSymbolLength>::try_from(token_symbol.clone())
				.map_err(|_| Error::<T>::TokenSymbolTooLong)?;

			// Get lending pool
			let mut lending_pool = LendingPools::<T>::get(&token_symbol)
				.ok_or(Error::<T>::LendingPoolNotFound)?;

			// Get user position
			let mut user_position = UserLendingPositions::<T>::get(&who, &token_symbol)
				.ok_or(Error::<T>::PositionNotFound)?;

			// Check borrowing capacity based on deposits and collateral
			let max_borrow = user_position.deposit_amount * lending_pool.liquidation_threshold / 100;
			let total_borrow_after = user_position.borrow_amount.checked_add(borrow_amount)
				.ok_or(Error::<T>::Overflow)?;

			ensure!(total_borrow_after <= max_borrow, Error::<T>::InsufficientCollateral);

			// Update pool and position
			lending_pool.total_borrows = lending_pool.total_borrows.checked_add(borrow_amount)
				.ok_or(Error::<T>::Overflow)?;
			user_position.borrow_amount = total_borrow_after;
			user_position.last_update = frame_system::Pallet::<T>::block_number();

			// Recalculate utilization rate
			if lending_pool.total_deposits > 0 {
				lending_pool.utilization_rate = (lending_pool.total_borrows * 1000) / lending_pool.total_deposits;
			}

			// Store updated pool and position
			LendingPools::<T>::insert(&token_symbol, lending_pool);
			UserLendingPositions::<T>::insert(&who, &token_symbol, user_position);

			// Emit event
			Self::deposit_event(Event::LoanTaken {
				token_symbol: token_symbol.clone(),
				user: who,
				amount: borrow_amount,
			});

			Ok(())
		}

		/// Repay loan to lending pool
		#[pallet::call_index(7)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
		pub fn repay_lending_pool(
			origin: OriginFor<T>,
			token_symbol: Vec<u8>,
			repay_amount: u128,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vector
			let token_symbol = BoundedVec::<u8, T::MaxTokenSymbolLength>::try_from(token_symbol.clone())
				.map_err(|_| Error::<T>::TokenSymbolTooLong)?;

			// Get lending pool
			let mut lending_pool = LendingPools::<T>::get(&token_symbol)
				.ok_or(Error::<T>::LendingPoolNotFound)?;

			// Get user position
			let mut user_position = UserLendingPositions::<T>::get(&who, &token_symbol)
				.ok_or(Error::<T>::PositionNotFound)?;

			// Calculate repayment amount (principal + interest)
			let total_repayment = repay_amount.min(user_position.borrow_amount + user_position.interest_accrued);

			// Update pool and position
			lending_pool.total_borrows = lending_pool.total_borrows.checked_sub(total_repayment)
				.ok_or(Error::<T>::Overflow)?;
			user_position.borrow_amount = user_position.borrow_amount.checked_sub(total_repayment)
				.ok_or(Error::<T>::Overflow)?;
			user_position.last_update = frame_system::Pallet::<T>::block_number();

			// Recalculate utilization rate
			if lending_pool.total_deposits > 0 {
				lending_pool.utilization_rate = (lending_pool.total_borrows * 1000) / lending_pool.total_deposits;
			}

			// Store updated pool and position
			LendingPools::<T>::insert(&token_symbol, lending_pool);
			UserLendingPositions::<T>::insert(&who, &token_symbol, user_position);

			// Emit event
			Self::deposit_event(Event::LoanRepaid {
				token_symbol: token_symbol.clone(),
				user: who,
				amount: total_repayment,
			});

			Ok(())
		}
	}
}