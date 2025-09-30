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

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Native currency type for marketplace fees
		type Currency: fungible::Inspect<Self::AccountId> + fungible::Mutate<Self::AccountId>;

		/// Maximum length of collection name
		#[pallet::constant]
		type MaxCollectionNameLength: Get<u32>;

		/// Maximum length of token metadata
		#[pallet::constant]
		type MaxTokenMetadataLength: Get<u32>;

		/// Maximum royalty percentage (basis points)
		#[pallet::constant]
		type MaxRoyaltyPercentage: Get<u32>;

		/// Marketplace fee percentage
		#[pallet::constant]
		type MarketplaceFee: Get<u32>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// NFT Collections storage
	#[pallet::storage]
	#[pallet::getter(fn nft_collections)]
	pub type NFTCollections<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxCollectionNameLength>,
		NFTCollection<T>,
		OptionQuery,
	>;

	/// NFT Tokens storage
	#[pallet::storage]
	#[pallet::getter(fn nft_tokens)]
	pub type NFTTokens<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxCollectionNameLength>,
		Blake2_128Concat,
		u64, // Token ID
		NFTToken<T>,
		OptionQuery,
	>;

	/// Token ownership storage
	#[pallet::storage]
	#[pallet::getter(fn token_ownership)]
	pub type TokenOwnership<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		(T::AccountId, BoundedVec<u8, T::MaxCollectionNameLength>, u64), // (Owner, Collection, TokenId)
		(),
		OptionQuery,
	>;

	/// Marketplace listings
	#[pallet::storage]
	#[pallet::getter(fn marketplace_listings)]
	pub type MarketplaceListings<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		(BoundedVec<u8, T::MaxCollectionNameLength>, u64), // (Collection, TokenId)
		MarketplaceListing<T>,
		OptionQuery,
	>;

	/// Collection information
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct NFTCollection<T: Config> {
		/// Collection name
		pub name: BoundedVec<u8, T::MaxCollectionNameLength>,
		/// Collection description
		pub description: BoundedVec<u8, T::MaxTokenMetadataLength>,
		/// Collection creator
		pub creator: T::AccountId,
		/// Maximum token supply
		pub max_supply: Option<u64>,
		/// Current token count
		pub current_supply: u64,
		/// Default royalty percentage
		pub royalty_percentage: u32,
		/// Creation timestamp
		pub created: T::BlockNumber,
	}

	/// NFT token information
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct NFTToken<T: Config> {
		/// Token ID
		pub id: u64,
		/// Token metadata URI
		pub metadata_uri: BoundedVec<u8, T::MaxTokenMetadataLength>,
		/// Token owner
		pub owner: T::AccountId,
		/// Creator of token
		pub creator: T::AccountId,
		/// Royalty percentage for this token
		pub royalty_percentage: u32,
		/// Creation timestamp
		pub created: T::BlockNumber,
		/// Minted timestamp
		pub minted: T::BlockNumber,
	}

	/// Marketplace listing
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct MarketplaceListing<T: Config> {
		/// Collection name
		pub collection: BoundedVec<u8, T::MaxCollectionNameLength>,
		/// Token ID
		pub token_id: u64,
		/// Listing price
		pub price: u128,
		/// Seller address
		pub seller: T::AccountId,
		/// Listing timestamp
		pub listed_at: T::BlockNumber,
		/// Listing expiration
		pub expires_at: Option<T::BlockNumber>,
		/// Active status
		pub active: bool,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New NFT collection created
		NFTCollectionCreated {
			collection: BoundedVec<u8, T::MaxCollectionNameLength>,
			creator: T::AccountId,
			max_supply: Option<u64>,
		},
		/// NFT token minted
		NFTMinted {
			collection: BoundedVec<u8, T::MaxCollectionNameLength>,
			token_id: u64,
			creator: T::AccountId,
			owner: T::AccountId,
		},
		/// NFT token transferred
		NFTTransferred {
			collection: BoundedVec<u8, T::MaxCollectionNameLength>,
			token_id: u64,
			from: T::AccountId,
			to: T::AccountId,
		},
		/// NFT listed on marketplace
		NFTListed {
			collection: BoundedVec<u8, T::MaxCollectionNameLength>,
			token_id: u64,
			seller: T::AccountId,
			price: u128,
		},
		/// NFT sold on marketplace
		NFTSold {
			collection: BoundedVec<u8, T::MaxCollectionNameLength>,
			token_id: u64,
			seller: T::AccountId,
			buyer: T::AccountId,
			price: u128,
		},
		/// NFT listing cancelled
		NFTListingCancelled {
			collection: BoundedVec<u8, T::MaxCollectionNameLength>,
			token_id: u64,
			seller: T::AccountId,
		},
		/// Royalty paid
		RoyaltyPaid {
			collection: BoundedVec<u8, T::MaxCollectionNameLength>,
			token_id: u64,
			recipient: T::AccountId,
			amount: u128,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Collection already exists
		CollectionAlreadyExists,
		/// Collection does not exist
		CollectionNotFound,
		/// Token already exists
		TokenAlreadyExists,
		/// Token does not exist
		TokenNotFound,
		/// Not authorized to perform this action
		NotAuthorized,
		/// Collection name too long
		CollectionNameTooLong,
		/// Token metadata too long
		TokenMetadataTooLong,
		/// Maximum supply reached
		MaxSupplyReached,
		/// Listing already exists
		ListingAlreadyExists,
		/// Listing does not exist
		ListingNotFound,
		/// Listing expired
		ListingExpired,
		/// Insufficient balance for purchase
		InsufficientBalance,
		/// Invalid royalty percentage
		InvalidRoyalty,
		/// Transfer failed
		TransferFailed,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create new NFT collection
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_nft_collection(
			origin: OriginFor<T>,
			name: Vec<u8>,
			description: Vec<u8>,
			max_supply: Option<u64>,
			royalty_percentage: u32,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let name = BoundedVec::<u8, T::MaxCollectionNameLength>::try_from(name.clone())
				.map_err(|_| Error::<T>::CollectionNameTooLong)?;
			let description = BoundedVec::<u8, T::MaxTokenMetadataLength>::try_from(description)
				.map_err(|_| Error::<T>::TokenMetadataTooLong)?;

			// Validate royalty percentage
			ensure!(royalty_percentage <= T::MaxRoyaltyPercentage::get(), Error::<T>::InvalidRoyalty);

			// Check if collection already exists
			ensure!(!NFTCollections::<T>::contains_key(&name), Error::<T>::CollectionAlreadyExists);

			// Create collection
			let collection = NFTCollection::<T> {
				name: name.clone(),
				description,
				creator: who.clone(),
				max_supply,
				current_supply: 0,
				royalty_percentage,
				created: frame_system::Pallet::<T>::block_number(),
			};

			// Store collection
			NFTCollections::<T>::insert(&name, collection);

			// Emit event
			Self::deposit_event(Event::NFTCollectionCreated {
				collection: name.clone(),
				creator: who,
				max_supply,
			});

			Ok(())
		}

		/// Mint NFT token
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
		pub fn mint_nft(
			origin: OriginFor<T>,
			collection: Vec<u8>,
			token_id: u64,
			metadata_uri: Vec<u8>,
			royalty_percentage: u32,
			to: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let collection = BoundedVec::<u8, T::MaxCollectionNameLength>::try_from(collection.clone())
				.map_err(|_| Error::<T>::CollectionNameTooLong)?;
			let metadata_uri = BoundedVec::<u8, T::MaxTokenMetadataLength>::try_from(metadata_uri)
				.map_err(|_| Error::<T>::TokenMetadataTooLong)?;

			// Get collection
			let mut nft_collection = NFTCollections::<T>::get(&collection)
				.ok_or(Error::<T>::CollectionNotFound)?;

			// Check authorization (only creator can mint)
			ensure!(nft_collection.creator == who, Error::<T>::NotAuthorized);

			// Check if token already exists
			ensure!(!NFTTokens::<T>::contains_key(&collection, token_id), Error::<T>::TokenAlreadyExists);

			// Check max supply
			if let Some(max) = nft_collection.max_supply {
				ensure!(nft_collection.current_supply < max, Error::<T>::MaxSupplyReached);
			}

			// Validate royalty percentage
			ensure!(royalty_percentage <= T::MaxRoyaltyPercentage::get(), Error::<T>::InvalidRoyalty);

			// Create NFT token
			let nft_token = NFTToken::<T> {
				id: token_id,
				metadata_uri: metadata_uri.clone(),
				owner: to.clone(),
				creator: who.clone(),
				royalty_percentage,
				created: frame_system::Pallet::<T>::block_number(),
				minted: frame_system::Pallet::<T>::block_number(),
			};

			// Store token
			NFTTokens::<T>::insert(&collection, token_id, nft_token);

			// Update collection supply
			nft_collection.current_supply += 1;
			NFTCollections::<T>::insert(&collection, nft_collection);

			// Record ownership
			TokenOwnership::<T>::insert(&to, &(to.clone(), collection.clone(), token_id), ());

			// Emit event
			Self::deposit_event(Event::NFTMinted {
				collection: collection.clone(),
				token_id,
				creator: who,
				owner: to,
			});

			Ok(())
		}

		/// List NFT on marketplace
		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(2, 2).ref_time())]
		pub fn list_nft(
			origin: OriginFor<T>,
			collection: Vec<u8>,
			token_id: u64,
			price: u128,
			duration: Option<T::BlockNumber>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vector
			let collection = BoundedVec::<u8, T::MaxCollectionNameLength>::try_from(collection.clone())
				.map_err(|_| Error::<T>::CollectionNameTooLong)?;

			// Verify token ownership
			ensure!(TokenOwnership::<T>::contains_key(&who, &(who.clone(), collection.clone(), token_id)), Error::<T>::NotAuthorized);

			// Check if already listed
			ensure!(!MarketplaceListings::<T>::contains_key(&(collection.clone(), token_id)), Error::<T>::ListingAlreadyExists);

			// Calculate expiration
			let expires_at = if let Some(duration) = duration {
				Some(frame_system::Pallet::<T>::block_number() + duration)
			} else {
				None
			};

			// Create listing
			let listing = MarketplaceListing::<T> {
				collection: collection.clone(),
				token_id,
				price,
				seller: who.clone(),
				listed_at: frame_system::Pallet::<T>::block_number(),
				expires_at,
				active: true,
			};

			// Store listing
			MarketplaceListings::<T>::insert(&(collection.clone(), token_id), listing);

			// Emit event
			Self::deposit_event(Event::NFTListed {
				collection: collection.clone(),
				token_id,
				seller: who,
				price,
			});

			Ok(())
		}

		/// Purchase NFT from marketplace
		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(3, 4).ref_time())]
		pub fn purchase_nft(
			origin: OriginFor<T>,
			collection: Vec<u8>,
			token_id: u64,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vector
			let collection = BoundedVec::<u8, T::MaxCollectionNameLength>::try_from(collection.clone())
				.map_err(|_| Error::<T>::CollectionNameTooLong)?;

			// Get listing
			let mut listing = MarketplaceListings::<T>::get(&(collection.clone(), token_id))
				.ok_or(Error::<T>::ListingNotFound)?;

			// Check if listing is active
			ensure!(listing.active, Error::<T>::ListingNotFound);

			// Check if listing expired
			if let Some(expiry) = listing.expires_at {
				ensure!(frame_system::Pallet::<T>::block_number() < expiry, Error::<T>::ListingExpired);
			}

			// Get token
			let mut token = NFTTokens::<T>::get(&collection, token_id)
				.ok_or(Error::<T>::TokenNotFound)?;

			// Verify current owner
			ensure!(token.owner == listing.seller, Error::<T>::NotAuthorized);

			// Check buyer balance
			ensure!(T::Currency::reducible_balance(&who, false) >= listing.price, Error::<T>::InsufficientBalance);

			// Calculate fees and royalties
			let marketplace_fee = (listing.price * T::MarketplaceFee::get() as u128) / 10000;
			let royalty_amount = (listing.price * token.royalty_percentage as u128) / 10000;
			let seller_amount = listing.price - marketplace_fee - royalty_amount;

			// Transfer funds
			// Marketplace fee (burn or treasury)
			// Royalty to creator
			if royalty_amount > 0 {
				// Transfer royalty to token creator
				T::Currency::transfer(&who, &token.creator, royalty_amount, false)?;
			}

			// Payment to seller
			T::Currency::transfer(&who, &listing.seller, seller_amount, false)?;

			// Transfer NFT ownership
			let old_owner = token.owner.clone();
			token.owner = who.clone();
			NFTTokens::<T>::insert(&collection, token_id, token);

			// Update ownership records
			TokenOwnership::<T>::remove(&old_owner, &(old_owner, collection.clone(), token_id));
			TokenOwnership::<T>::insert(&who, &(who.clone(), collection.clone(), token_id), ());

			// Deactivate listing
			listing.active = false;
			MarketplaceListings::<T>::insert(&(collection.clone(), token_id), listing);

			// Emit events
			Self::deposit_event(Event::NFTSold {
				collection: collection.clone(),
				token_id,
				seller: listing.seller,
				buyer: who.clone(),
				price: listing.price,
			});

			if royalty_amount > 0 {
				Self::deposit_event(Event::RoyaltyPaid {
					collection: collection.clone(),
					token_id,
					recipient: token.creator,
					amount: royalty_amount,
				});
			}

			Ok(())
		}

		/// Transfer NFT token
		#[pallet::call_index(4)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
		pub fn transfer_nft(
			origin: OriginFor<T>,
			collection: Vec<u8>,
			token_id: u64,
			to: T::AccountId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vector
			let collection = BoundedVec::<u8, T::MaxCollectionNameLength>::try_from(collection.clone())
				.map_err(|_| Error::<T>::CollectionNameTooLong)?;

			// Verify token ownership
			ensure!(TokenOwnership::<T>::contains_key(&who, &(who.clone(), collection.clone(), token_id)), Error::<T>::NotAuthorized);

			// Get token
			let mut token = NFTTokens::<T>::get(&collection, token_id)
				.ok_or(Error::<T>::TokenNotFound)?;

			// Update ownership
			let old_owner = token.owner.clone();
			token.owner = to.clone();
			NFTTokens::<T>::insert(&collection, token_id, token);

			// Update ownership records
			TokenOwnership::<T>::remove(&old_owner, &(old_owner, collection.clone(), token_id));
			TokenOwnership::<T>::insert(&to, &(to.clone(), collection.clone(), token_id), ());

			// Emit event
			Self::deposit_event(Event::NFTTransferred {
				collection: collection.clone(),
				token_id,
				from: old_owner,
				to: to.clone(),
			});

			Ok(())
		}

		/// Cancel marketplace listing
		#[pallet::call_index(5)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn cancel_nft_listing(
			origin: OriginFor<T>,
			collection: Vec<u8>,
			token_id: u64,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vector
			let collection = BoundedVec::<u8, T::MaxCollectionNameLength>::try_from(collection.clone())
				.map_err(|_| Error::<T>::CollectionNameTooLong)?;

			// Get listing
			let mut listing = MarketplaceListings::<T>::get(&(collection.clone(), token_id))
				.ok_or(Error::<T>::ListingNotFound)?;

			// Check authorization (only seller can cancel)
			ensure!(listing.seller == who, Error::<T>::NotAuthorized);

			// Cancel listing
			listing.active = false;
			MarketplaceListings::<T>::insert(&(collection.clone(), token_id), listing);

			// Emit event
			Self::deposit_event(Event::NFTListingCancelled {
				collection: collection.clone(),
				token_id,
				seller: who,
			});

			Ok(())
		}
	}
}