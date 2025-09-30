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

		/// Maximum length of model identifier
		#[pallet::constant]
		type MaxModelIdLength: Get<u32>;

		/// Maximum length of model metadata
		#[pallet::constant]
		type MaxModelMetadataLength: Get<u32>;

		/// Maximum length of training data hash
		#[pallet::constant]
		type MaxTrainingDataLength: Get<u32>;

		/// Maximum number of models per developer
		#[pallet::constant]
		type MaxModelsPerDeveloper: Get<u32>;

		/// Computation fee for AI operations
		#[pallet::constant]
		type ComputationFee: Get<u128>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// AI model registry storage
	#[pallet::storage]
	#[pallet::getter(fn ai_models)]
	pub type AIModels<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxModelIdLength>,
		AIModel<T>,
		OptionQuery,
	>;

	/// Model training data storage
	#[pallet::storage]
	#[pallet::getter(fn training_data)]
	pub type TrainingData<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxModelIdLength>,
		TrainingDataInfo<T>,
		OptionQuery,
	>;

	/// Model predictions storage
	#[pallet::storage]
	#[pallet::getter(fn model_predictions)]
	pub type ModelPredictions<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxModelIdLength>,
		PredictionResult<T>,
		OptionQuery,
	>;

	/// Developer model count storage
	#[pallet::storage]
	#[pallet::getter(fn developer_models)]
	pub type DeveloperModels<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		u32,
		ValueQuery,
	>;

	/// Model information
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct AIModel<T: Config> {
		/// Model identifier
		pub model_id: BoundedVec<u8, T::MaxModelIdLength>,
		/// Model name
		pub name: BoundedVec<u8, T::MaxModelIdLength>,
		/// Model description
		pub description: BoundedVec<u8, T::MaxModelMetadataLength>,
		/// Model developer
		pub developer: T::AccountId,
		/// Model type
		pub model_type: ModelType,
		/// Model version
		pub version: BoundedVec<u8, T::MaxModelIdLength>,
		/// Model parameters hash
		pub parameters_hash: BoundedVec<u8, T::MaxModelMetadataLength>,
		/// Model accuracy metrics
		pub accuracy: u32, // Basis points (0-10000)
		/// Model status
		pub status: ModelStatus,
		/// Creation timestamp
		pub created: T::BlockNumber,
		/// Last updated
		pub last_updated: T::BlockNumber,
		/// Usage count
		pub usage_count: u64,
	}

	/// Model types
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum ModelType {
		/// Neural Network
		NeuralNetwork,
		/// Decision Tree
		DecisionTree,
		/// Support Vector Machine
		SVM,
		/// Random Forest
		RandomForest,
		/// Gradient Boosting
		GradientBoosting,
		/// Deep Learning
		DeepLearning,
		/// Custom model
		Custom,
	}

	/// Model status
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum ModelStatus {
		/// Model is being trained
		Training,
		/// Model is ready for use
		Active,
		/// Model is deprecated
		Deprecated,
		/// Model is suspended
		Suspended,
	}

	/// Training data information
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct TrainingDataInfo<T: Config> {
		/// Model identifier
		pub model_id: BoundedVec<u8, T::MaxModelIdLength>,
		/// Training data hash
		pub data_hash: BoundedVec<u8, T::MaxTrainingDataLength>,
		/// Data size
		pub data_size: u64,
		/// Training epochs
		pub epochs: u32,
		/// Training accuracy
		pub training_accuracy: u32,
		/// Validation accuracy
		pub validation_accuracy: u32,
		/// Training timestamp
		pub trained_at: T::BlockNumber,
		/// Training duration
		pub training_duration: T::BlockNumber,
	}

	/// Prediction result
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct PredictionResult<T: Config> {
		/// Model identifier
		pub model_id: BoundedVec<u8, T::MaxModelIdLength>,
		/// Input data hash
		pub input_hash: BoundedVec<u8, T::MaxModelMetadataLength>,
		/// Prediction output
		pub prediction: BoundedVec<u8, T::MaxModelMetadataLength>,
		/// Confidence score
		pub confidence: u32,
		/// Prediction timestamp
		pub predicted_at: T::BlockNumber,
		/// Gas used for prediction
		pub gas_used: u64,
		/// Prediction cost
		pub cost: u128,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New AI model registered
		AIModelRegistered {
			model_id: BoundedVec<u8, T::MaxModelIdLength>,
			developer: T::AccountId,
			model_type: ModelType,
		},
		/// Model training completed
		ModelTrainingCompleted {
			model_id: BoundedVec<u8, T::MaxModelIdLength>,
			developer: T::AccountId,
			training_accuracy: u32,
		},
		/// Model prediction made
		ModelPredictionMade {
			model_id: BoundedVec<u8, T::MaxModelIdLength>,
			user: T::AccountId,
			confidence: u32,
		},
		/// Model status updated
		ModelStatusUpdated {
			model_id: BoundedVec<u8, T::MaxModelIdLength>,
			old_status: ModelStatus,
			new_status: ModelStatus,
		},
		/// Training data submitted
		TrainingDataSubmitted {
			model_id: BoundedVec<u8, T::MaxModelIdLength>,
			developer: T::AccountId,
			data_size: u64,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Model already exists
		ModelAlreadyExists,
		/// Model not found
		ModelNotFound,
		/// Training data not found
		TrainingDataNotFound,
		/// Not authorized to perform this action
		NotAuthorized,
		/// Model identifier too long
		ModelIdTooLong,
		/// Model metadata too long
		ModelMetadataTooLong,
		/// Training data too long
		TrainingDataTooLong,
		/// Maximum models per developer reached
		MaxModelsPerDeveloperReached,
		/// Model not active
		ModelNotActive,
		/// Insufficient funds for computation
		InsufficientFunds,
		/// Invalid model status
		InvalidModelStatus,
		/// Training data already exists
		TrainingDataAlreadyExists,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Register new AI model
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn register_ai_model(
			origin: OriginFor<T>,
			model_id: Vec<u8>,
			name: Vec<u8>,
			description: Vec<u8>,
			model_type: ModelType,
			version: Vec<u8>,
			parameters_hash: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let model_id = BoundedVec::<u8, T::MaxModelIdLength>::try_from(model_id.clone())
				.map_err(|_| Error::<T>::ModelIdTooLong)?;
			let name = BoundedVec::<u8, T::MaxModelIdLength>::try_from(name)
				.map_err(|_| Error::<T>::ModelIdTooLong)?;
			let description = BoundedVec::<u8, T::MaxModelMetadataLength>::try_from(description)
				.map_err(|_| Error::<T>::ModelMetadataTooLong)?;
			let version = BoundedVec::<u8, T::MaxModelIdLength>::try_from(version)
				.map_err(|_| Error::<T>::ModelIdTooLong)?;
			let parameters_hash = BoundedVec::<u8, T::MaxModelMetadataLength>::try_from(parameters_hash)
				.map_err(|_| Error::<T>::ModelMetadataTooLong)?;

			// Check if model already exists
			ensure!(!AIModels::<T>::contains_key(&model_id), Error::<T>::ModelAlreadyExists);

			// Check developer model limit
			let current_count = DeveloperModels::<T>::get(&who);
			ensure!(current_count < T::MaxModelsPerDeveloper::get(), Error::<T>::MaxModelsPerDeveloperReached);

			// Create AI model
			let ai_model = AIModel::<T> {
				model_id: model_id.clone(),
				name,
				description,
				developer: who.clone(),
				model_type: model_type.clone(),
				version,
				parameters_hash,
				accuracy: 0,
				status: ModelStatus::Training,
				created: frame_system::Pallet::<T>::block_number(),
				last_updated: frame_system::Pallet::<T>::block_number(),
				usage_count: 0,
			};

			// Store AI model
			AIModels::<T>::insert(&model_id, ai_model);

			// Update developer model count
			DeveloperModels::<T>::insert(&who, current_count + 1);

			// Emit event
			Self::deposit_event(Event::AIModelRegistered {
				model_id: model_id.clone(),
				developer: who,
				model_type,
			});

			Ok(())
		}

		/// Submit training data for model
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn submit_training_data(
			origin: OriginFor<T>,
			model_id: Vec<u8>,
			data_hash: Vec<u8>,
			data_size: u64,
			epochs: u32,
			training_accuracy: u32,
			validation_accuracy: u32,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let model_id = BoundedVec::<u8, T::MaxModelIdLength>::try_from(model_id.clone())
				.map_err(|_| Error::<T>::ModelIdTooLong)?;
			let data_hash = BoundedVec::<u8, T::MaxTrainingDataLength>::try_from(data_hash)
				.map_err(|_| Error::<T>::TrainingDataTooLong)?;

			// Get model
			let mut model = AIModels::<T>::get(&model_id)
				.ok_or(Error::<T>::ModelNotFound)?;

			// Check authorization (only model developer)
			ensure!(model.developer == who, Error::<T>::NotAuthorized);

			// Check if training data already exists
			ensure!(!TrainingData::<T>::contains_key(&model_id), Error::<T>::TrainingDataAlreadyExists);

			// Create training data info
			let training_data_info = TrainingDataInfo::<T> {
				model_id: model_id.clone(),
				data_hash,
				data_size,
				epochs,
				training_accuracy,
				validation_accuracy,
				trained_at: frame_system::Pallet::<T>::block_number(),
				training_duration: T::BlockNumber::zero(), // Would be calculated
			};

			// Store training data
			TrainingData::<T>::insert(&model_id, training_data_info);

			// Update model accuracy and status
			model.accuracy = validation_accuracy;
			model.status = ModelStatus::Active;
			model.last_updated = frame_system::Pallet::<T>::block_number();

			// Store updated model
			AIModels::<T>::insert(&model_id, model);

			// Emit event
			Self::deposit_event(Event::TrainingDataSubmitted {
				model_id: model_id.clone(),
				developer: who,
				data_size,
			});

			Self::deposit_event(Event::ModelTrainingCompleted {
				model_id: model_id.clone(),
				developer: who,
				training_accuracy: validation_accuracy,
			});

			Ok(())
		}

		/// Make prediction using AI model
		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn make_prediction(
			origin: OriginFor<T>,
			model_id: Vec<u8>,
			input_data: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let model_id = BoundedVec::<u8, T::MaxModelIdLength>::try_from(model_id.clone())
				.map_err(|_| Error::<T>::ModelIdTooLong)?;
			let input_hash = BoundedVec::<u8, T::MaxModelMetadataLength>::try_from(
				sp_runtime::traits::Hash::hash(&input_data).as_ref().to_vec()
			).map_err(|_| Error::<T>::ModelMetadataTooLong)?;

			// Get model
			let mut model = AIModels::<T>::get(&model_id)
				.ok_or(Error::<T>::ModelNotFound)?;

			// Check if model is active
			ensure!(model.status == ModelStatus::Active, Error::<T>::ModelNotActive);

			// Simulate AI prediction (in real implementation, this would call an AI runtime)
			let prediction = Self::generate_prediction(&input_data, &model)?;
			let confidence = Self::calculate_confidence(&model)?;

			// Create prediction result
			let prediction_result = PredictionResult::<T> {
				model_id: model_id.clone(),
				input_hash,
				prediction: BoundedVec::<u8, T::MaxModelMetadataLength>::try_from(prediction)
					.map_err(|_| Error::<T>::ModelMetadataTooLong)?,
				confidence,
				predicted_at: frame_system::Pallet::<T>::block_number(),
				gas_used: T::ComputationFee::get() as u64,
				cost: T::ComputationFee::get(),
			};

			// Store prediction result
			ModelPredictions::<T>::insert(&model_id, prediction_result);

			// Update model usage
			model.usage_count += 1;
			model.last_updated = frame_system::Pallet::<T>::block_number();
			AIModels::<T>::insert(&model_id, model);

			// Emit event
			Self::deposit_event(Event::ModelPredictionMade {
				model_id: model_id.clone(),
				user: who,
				confidence,
			});

			Ok(())
		}

		/// Update model status
		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn update_model_status(
			origin: OriginFor<T>,
			model_id: Vec<u8>,
			new_status: ModelStatus,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vector
			let model_id = BoundedVec::<u8, T::MaxModelIdLength>::try_from(model_id)
				.map_err(|_| Error::<T>::ModelIdTooLong)?;

			// Get model
			let mut model = AIModels::<T>::get(&model_id)
				.ok_or(Error::<T>::ModelNotFound)?;

			// Check authorization (only model developer)
			ensure!(model.developer == who, Error::<T>::NotAuthorized);

			// Update model status
			let old_status = model.status.clone();
			model.status = new_status.clone();
			model.last_updated = frame_system::Pallet::<T>::block_number();

			// Store updated model
			AIModels::<T>::insert(&model_id, model);

			// Emit event
			Self::deposit_event(Event::ModelStatusUpdated {
				model_id: model_id.clone(),
				old_status,
				new_status,
			});

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Generate prediction using AI model (simplified)
		fn generate_prediction(input_data: &[u8], model: &AIModel<T>) -> Result<Vec<u8>, Error<T>> {
			// In a real implementation, this would:
			// 1. Load the AI model parameters
			// 2. Run the model inference
			// 3. Return the prediction result

			// For this example, we'll simulate a prediction based on model type
			let prediction = match model.model_type {
				ModelType::NeuralNetwork => format!("Neural network prediction for: {:?}", input_data),
				ModelType::DecisionTree => format!("Decision tree classification: {:?}", input_data),
				ModelType::SVM => format!("SVM prediction result: {:?}", input_data),
				ModelType::RandomForest => format!("Random forest prediction: {:?}", input_data),
				ModelType::GradientBoosting => format!("Gradient boosting result: {:?}", input_data),
				ModelType::DeepLearning => format!("Deep learning prediction: {:?}", input_data),
				ModelType::Custom => format!("Custom model prediction: {:?}", input_data),
			};

			Ok(prediction.as_bytes().to_vec())
		}

		/// Calculate confidence score for prediction
		fn calculate_confidence(model: &AIModel<T>) -> Result<u32, Error<T>> {
			// In a real implementation, this would calculate actual confidence
			// based on model performance metrics

			// For this example, base confidence on model accuracy
			Ok(model.accuracy)
		}

		/// Get model accuracy
		pub fn get_model_accuracy(model_id: &BoundedVec<u8, T::MaxModelIdLength>) -> Option<u32> {
			AIModels::<T>::get(model_id).map(|model| model.accuracy)
		}

		/// Get prediction confidence
		pub fn get_prediction_confidence(model_id: &BoundedVec<u8, T::MaxModelIdLength>) -> Option<u32> {
			ModelPredictions::<T>::get(model_id).map(|prediction| prediction.confidence)
		}
	}
}