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

		/// Maximum length of a dApp name
		#[pallet::constant]
		type MaxDappNameLength: Get<u32>;

		/// Maximum length of contract metadata
		#[pallet::constant]
		type MaxContractMetadataLength: Get<u32>;

		/// Maximum number of contracts per dApp
		#[pallet::constant]
		type MaxContractsPerDapp: Get<u32>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// DApp registry storage
	#[pallet::storage]
	#[pallet::getter(fn dapps)]
	pub type Dapps<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxDappNameLength>,
		DappInfo<T>,
		OptionQuery,
	>;

	/// Contract storage references
	#[pallet::storage]
	#[pallet::getter(fn contract_references)]
	pub type ContractReferences<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxDappNameLength>,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxContractMetadataLength>,
		ContractInfo<T>,
		OptionQuery,
	>;

	/// DApp information structure
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct DappInfo<T: Config> {
		/// DApp owner
		pub owner: T::AccountId,
		/// DApp description
		pub description: BoundedVec<u8, T::MaxContractMetadataLength>,
		/// Creation timestamp
		pub created: T::BlockNumber,
		/// Last update timestamp
		pub updated: T::BlockNumber,
		/// Contract count
		pub contract_count: u32,
	}

	/// Contract information structure
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct ContractInfo<T: Config> {
		/// Contract deployer
		pub deployer: T::AccountId,
		/// Contract bytecode hash
		pub bytecode_hash: BoundedVec<u8, T::MaxContractMetadataLength>,
		/// Contract ABI hash
		pub abi_hash: BoundedVec<u8, T::MaxContractMetadataLength>,
		/// Deployment timestamp
		pub deployed_at: T::BlockNumber,
		/// Gas optimization level
		pub gas_optimization: u32,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new DApp was registered
		DappRegistered {
			dapp_name: BoundedVec<u8, T::MaxDappNameLength>,
			owner: T::AccountId,
		},
		/// DApp metadata was updated
		DappUpdated {
			dapp_name: BoundedVec<u8, T::MaxDappNameLength>,
			owner: T::AccountId,
		},
		/// Contract was deployed
		ContractDeployed {
			dapp_name: BoundedVec<u8, T::MaxDappNameLength>,
			contract_metadata: BoundedVec<u8, T::MaxContractMetadataLength>,
			deployer: T::AccountId,
		},
		/// Contract was optimized
		ContractOptimized {
			dapp_name: BoundedVec<u8, T::MaxDappNameLength>,
			contract_metadata: BoundedVec<u8, T::MaxContractMetadataLength>,
			optimization_level: u32,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// DApp already exists
		DappAlreadyExists,
		/// DApp does not exist
		DappNotFound,
		/// Contract reference already exists
		ContractAlreadyExists,
		/// Contract reference does not exist
		ContractNotFound,
		/// Not authorized to perform this action
		NotAuthorized,
		/// DApp name too long
		DappNameTooLong,
		/// Contract metadata too long
		ContractMetadataTooLong,
		/// Maximum contracts per DApp reached
		MaxContractsReached,
		/// Invalid gas optimization level
		InvalidGasOptimization,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Register a new Web3 DApp
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn register_dapp(
			origin: OriginFor<T>,
			dapp_name: Vec<u8>,
			description: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let dapp_name = BoundedVec::<u8, T::MaxDappNameLength>::try_from(dapp_name)
				.map_err(|_| Error::<T>::DappNameTooLong)?;
			let description = BoundedVec::<u8, T::MaxContractMetadataLength>::try_from(description)
				.map_err(|_| Error::<T>::ContractMetadataTooLong)?;

			// Check if DApp already exists
			ensure!(!Dapps::<T>::contains_key(&dapp_name), Error::<T>::DappAlreadyExists);

			// Create DApp info
			let dapp_info = DappInfo::<T> {
				owner: who.clone(),
				description,
				created: frame_system::Pallet::<T>::block_number(),
				updated: frame_system::Pallet::<T>::block_number(),
				contract_count: 0,
			};

			// Store DApp
			Dapps::<T>::insert(&dapp_name, dapp_info);

			// Emit event
			Self::deposit_event(Event::DappRegistered {
				dapp_name: dapp_name.clone(),
				owner: who,
			});

			Ok(())
		}

		/// Deploy a smart contract for a DApp
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
		pub fn deploy_contract(
			origin: OriginFor<T>,
			dapp_name: Vec<u8>,
			contract_metadata: Vec<u8>,
			bytecode_hash: Vec<u8>,
			abi_hash: Vec<u8>,
			gas_optimization: u32,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let dapp_name = BoundedVec::<u8, T::MaxDappNameLength>::try_from(dapp_name.clone())
				.map_err(|_| Error::<T>::DappNameTooLong)?;
			let contract_metadata = BoundedVec::<u8, T::MaxContractMetadataLength>::try_from(contract_metadata)
				.map_err(|_| Error::<T>::ContractMetadataTooLong)?;
			let bytecode_hash = BoundedVec::<u8, T::MaxContractMetadataLength>::try_from(bytecode_hash)
				.map_err(|_| Error::<T>::ContractMetadataTooLong)?;
			let abi_hash = BoundedVec::<u8, T::MaxContractMetadataLength>::try_from(abi_hash)
				.map_err(|_| Error::<T>::ContractMetadataTooLong)?;

			// Validate gas optimization level
			ensure!(gas_optimization <= 1000, Error::<T>::InvalidGasOptimization);

			// Get DApp
			let mut dapp_info = Dapps::<T>::get(&dapp_name)
				.ok_or(Error::<T>::DappNotFound)?;

			// Check authorization (only owner or check if open DApp)
			ensure!(dapp_info.owner == who, Error::<T>::NotAuthorized);

			// Check contract limit
			ensure!(dapp_info.contract_count < T::MaxContractsPerDapp::get(), Error::<T>::MaxContractsReached);

			// Check if contract already exists
			ensure!(!ContractReferences::<T>::contains_key(&dapp_name, &contract_metadata), Error::<T>::ContractAlreadyExists);

			// Create contract info
			let contract_info = ContractInfo::<T> {
				deployer: who.clone(),
				bytecode_hash,
				abi_hash,
				deployed_at: frame_system::Pallet::<T>::block_number(),
				gas_optimization,
			};

			// Store contract reference
			ContractReferences::<T>::insert(&dapp_name, &contract_metadata, contract_info);

			// Update DApp contract count
			dapp_info.contract_count += 1;
			dapp_info.updated = frame_system::Pallet::<T>::block_number();
			Dapps::<T>::insert(&dapp_name, dapp_info);

			// Emit event
			Self::deposit_event(Event::ContractDeployed {
				dapp_name: dapp_name.clone(),
				contract_metadata: contract_metadata.clone(),
				deployer: who,
			});

			Ok(())
		}

		/// Optimize contract gas usage
		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn optimize_contract(
			origin: OriginFor<T>,
			dapp_name: Vec<u8>,
			contract_metadata: Vec<u8>,
			new_gas_optimization: u32,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let dapp_name = BoundedVec::<u8, T::MaxDappNameLength>::try_from(dapp_name)
				.map_err(|_| Error::<T>::DappNameTooLong)?;
			let contract_metadata = BoundedVec::<u8, T::MaxContractMetadataLength>::try_from(contract_metadata)
				.map_err(|_| Error::<T>::ContractMetadataTooLong)?;

			// Validate optimization level
			ensure!(new_gas_optimization <= 1000, Error::<T>::InvalidGasOptimization);

			// Get contract
			let mut contract_info = ContractReferences::<T>::get(&dapp_name, &contract_metadata)
				.ok_or(Error::<T>::ContractNotFound)?;

			// Check authorization (only deployer)
			ensure!(contract_info.deployer == who, Error::<T>::NotAuthorized);

			// Update gas optimization
			contract_info.gas_optimization = new_gas_optimization;
			ContractReferences::<T>::insert(&dapp_name, &contract_metadata, contract_info);

			// Emit event
			Self::deposit_event(Event::ContractOptimized {
				dapp_name: dapp_name.clone(),
				contract_metadata: contract_metadata.clone(),
				optimization_level: new_gas_optimization,
			});

			Ok(())
		}

		/// Update DApp metadata
		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn update_dapp(
			origin: OriginFor<T>,
			dapp_name: Vec<u8>,
			description: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let dapp_name = BoundedVec::<u8, T::MaxDappNameLength>::try_from(dapp_name)
				.map_err(|_| Error::<T>::DappNameTooLong)?;
			let description = BoundedVec::<u8, T::MaxContractMetadataLength>::try_from(description)
				.map_err(|_| Error::<T>::ContractMetadataTooLong)?;

			// Get existing DApp
			let mut dapp_info = Dapps::<T>::get(&dapp_name)
				.ok_or(Error::<T>::DappNotFound)?;

			// Check ownership
			ensure!(dapp_info.owner == who, Error::<T>::NotAuthorized);

			// Update description and timestamp
			dapp_info.description = description;
			dapp_info.updated = frame_system::Pallet::<T>::block_number();

			// Store updated DApp
			Dapps::<T>::insert(&dapp_name, dapp_info);

			// Emit event
			Self::deposit_event(Event::DappUpdated {
				dapp_name: dapp_name.clone(),
				owner: who,
			});

			Ok(())
		}
	}
}