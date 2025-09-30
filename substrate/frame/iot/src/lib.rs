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

		/// Maximum length of device identifier
		#[pallet::constant]
		type MaxDeviceIdLength: Get<u32>;

		/// Maximum length of sensor data
		#[pallet::constant]
		type MaxSensorDataLength: Get<u32>;

		/// Maximum length of device metadata
		#[pallet::constant]
		type MaxDeviceMetadataLength: Get<u32>;

		/// Maximum number of devices per owner
		#[pallet::constant]
		type MaxDevicesPerOwner: Get<u32>;

		/// Data transmission fee
		#[pallet::constant]
		type DataTransmissionFee: Get<u128>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// IoT devices storage
	#[pallet::storage]
	#[pallet::getter(fn iot_devices)]
	pub type IoTDevices<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxDeviceIdLength>,
		IoTDevice<T>,
		OptionQuery,
	>;

	/// Device sensor data storage
	#[pallet::storage]
	#[pallet::getter(fn device_sensor_data)]
	pub type DeviceSensorData<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxDeviceIdLength>,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxDeviceIdLength>, // Sensor ID
		SensorData<T>,
		OptionQuery,
	>;

	/// Device ownership storage
	#[pallet::storage]
	#[pallet::getter(fn device_ownership)]
	pub type DeviceOwnership<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxDeviceIdLength>,
		(),
		OptionQuery,
	>;

	/// Device groups storage
	#[pallet::storage]
	#[pallet::getter(fn device_groups)]
	pub type DeviceGroups<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxDeviceIdLength>,
		DeviceGroup<T>,
		OptionQuery,
	>;

	/// IoT device information
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct IoTDevice<T: Config> {
		/// Device identifier
		pub device_id: BoundedVec<u8, T::MaxDeviceIdLength>,
		/// Device name
		pub name: BoundedVec<u8, T::MaxDeviceIdLength>,
		/// Device owner
		pub owner: T::AccountId,
		/// Device type
		pub device_type: DeviceType,
		/// Device location
		pub location: BoundedVec<u8, T::MaxDeviceMetadataLength>,
		/// Device metadata
		pub metadata: BoundedVec<u8, T::MaxDeviceMetadataLength>,
		/// Device status
		pub status: DeviceStatus,
		/// Registration timestamp
		pub registered: T::BlockNumber,
		/// Last update timestamp
		pub last_update: T::BlockNumber,
		/// Data transmission count
		pub transmission_count: u64,
	}

	/// Device types
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum DeviceType {
		/// Sensor device
		Sensor,
		/// Actuator device
		Actuator,
		/// Gateway device
		Gateway,
		/// Edge computing device
		EdgeDevice,
		/// Wearable device
		Wearable,
		/// Industrial IoT device
		Industrial,
		/// Custom device type
		Custom,
	}

	/// Device status
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum DeviceStatus {
		/// Device is active
		Active,
		/// Device is inactive
		Inactive,
		/// Device is in maintenance
		Maintenance,
		/// Device is offline
		Offline,
		/// Device is faulty
		Faulty,
	}

	/// Sensor data information
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct SensorData<T: Config> {
		/// Device identifier
		pub device_id: BoundedVec<u8, T::MaxDeviceIdLength>,
		/// Sensor identifier
		pub sensor_id: BoundedVec<u8, T::MaxDeviceIdLength>,
		/// Sensor type
		pub sensor_type: SensorType,
		/// Data value
		pub value: BoundedVec<u8, T::MaxSensorDataLength>,
		/// Data timestamp
		pub timestamp: T::BlockNumber,
		/// Data quality score
		pub quality_score: u32,
		/// Calibration status
		pub calibrated: bool,
	}

	/// Sensor types
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum SensorType {
		/// Temperature sensor
		Temperature,
		/// Humidity sensor
		Humidity,
		/// Pressure sensor
		Pressure,
		/// Motion sensor
		Motion,
		/// Light sensor
		Light,
		/// GPS sensor
		GPS,
		/// Air quality sensor
		AirQuality,
		/// Custom sensor type
		Custom,
	}

	/// Device group information
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct DeviceGroup<T: Config> {
		/// Group identifier
		pub group_id: BoundedVec<u8, T::MaxDeviceIdLength>,
		/// Group name
		pub name: BoundedVec<u8, T::MaxDeviceIdLength>,
		/// Group description
		pub description: BoundedVec<u8, T::MaxDeviceMetadataLength>,
		/// Group owner
		pub owner: T::AccountId,
		/// Device count
		pub device_count: u32,
		/// Group status
		pub status: GroupStatus,
		/// Creation timestamp
		pub created: T::BlockNumber,
	}

	/// Group status
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum GroupStatus {
		/// Group is active
		Active,
		/// Group is inactive
		Inactive,
		/// Group is archived
		Archived,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New IoT device registered
		IoTDeviceRegistered {
			device_id: BoundedVec<u8, T::MaxDeviceIdLength>,
			owner: T::AccountId,
			device_type: DeviceType,
		},
		/// Device status updated
		DeviceStatusUpdated {
			device_id: BoundedVec<u8, T::MaxDeviceIdLength>,
			old_status: DeviceStatus,
			new_status: DeviceStatus,
		},
		/// Sensor data received
		SensorDataReceived {
			device_id: BoundedVec<u8, T::MaxDeviceIdLength>,
			sensor_id: BoundedVec<u8, T::MaxDeviceIdLength>,
			sensor_type: SensorType,
			quality_score: u32,
		},
		/// Device group created
		DeviceGroupCreated {
			group_id: BoundedVec<u8, T::MaxDeviceIdLength>,
			owner: T::AccountId,
		},
		/// Device added to group
		DeviceAddedToGroup {
			group_id: BoundedVec<u8, T::MaxDeviceIdLength>,
			device_id: BoundedVec<u8, T::MaxDeviceIdLength>,
		},
		/// Device metadata updated
		DeviceMetadataUpdated {
			device_id: BoundedVec<u8, T::MaxDeviceIdLength>,
			owner: T::AccountId,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Device already exists
		DeviceAlreadyExists,
		/// Device not found
		DeviceNotFound,
		/// Sensor data not found
		SensorDataNotFound,
		/// Not authorized to perform this action
		NotAuthorized,
		/// Device identifier too long
		DeviceIdTooLong,
		/// Device metadata too long
		DeviceMetadataTooLong,
		/// Sensor data too long
		SensorDataTooLong,
		/// Maximum devices per owner reached
		MaxDevicesPerOwnerReached,
		/// Device not active
		DeviceNotActive,
		/// Insufficient funds for transmission
		InsufficientFunds,
		/// Group already exists
		GroupAlreadyExists,
		/// Group not found
		GroupNotFound,
		/// Device already in group
		DeviceAlreadyInGroup,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Register new IoT device
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn register_iot_device(
			origin: OriginFor<T>,
			device_id: Vec<u8>,
			name: Vec<u8>,
			device_type: DeviceType,
			location: Vec<u8>,
			metadata: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let device_id = BoundedVec::<u8, T::MaxDeviceIdLength>::try_from(device_id.clone())
				.map_err(|_| Error::<T>::DeviceIdTooLong)?;
			let name = BoundedVec::<u8, T::MaxDeviceIdLength>::try_from(name)
				.map_err(|_| Error::<T>::DeviceIdTooLong)?;
			let location = BoundedVec::<u8, T::MaxDeviceMetadataLength>::try_from(location)
				.map_err(|_| Error::<T>::DeviceMetadataTooLong)?;
			let metadata = BoundedVec::<u8, T::MaxDeviceMetadataLength>::try_from(metadata)
				.map_err(|_| Error::<T>::DeviceMetadataTooLong)?;

			// Check if device already exists
			ensure!(!IoTDevices::<T>::contains_key(&device_id), Error::<T>::DeviceAlreadyExists);

			// Check device limit per owner
			let current_count = DeviceOwnership::<T>::iter_prefix(&who).count() as u32;
			ensure!(current_count < T::MaxDevicesPerOwner::get(), Error::<T>::MaxDevicesPerOwnerReached);

			// Create IoT device
			let iot_device = IoTDevice::<T> {
				device_id: device_id.clone(),
				name,
				owner: who.clone(),
				device_type: device_type.clone(),
				location,
				metadata,
				status: DeviceStatus::Active,
				registered: frame_system::Pallet::<T>::block_number(),
				last_update: frame_system::Pallet::<T>::block_number(),
				transmission_count: 0,
			};

			// Store IoT device
			IoTDevices::<T>::insert(&device_id, iot_device);

			// Record ownership
			DeviceOwnership::<T>::insert(&who, &device_id, ());

			// Emit event
			Self::deposit_event(Event::IoTDeviceRegistered {
				device_id: device_id.clone(),
				owner: who,
				device_type,
			});

			Ok(())
		}

		/// Submit sensor data from IoT device
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 2).ref_time())]
		pub fn submit_sensor_data(
			origin: OriginFor<T>,
			device_id: Vec<u8>,
			sensor_id: Vec<u8>,
			sensor_type: SensorType,
			data: Vec<u8>,
			quality_score: u32,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let device_id = BoundedVec::<u8, T::MaxDeviceIdLength>::try_from(device_id.clone())
				.map_err(|_| Error::<T>::DeviceIdTooLong)?;
			let sensor_id = BoundedVec::<u8, T::MaxDeviceIdLength>::try_from(sensor_id.clone())
				.map_err(|_| Error::<T>::DeviceIdTooLong)?;
			let data = BoundedVec::<u8, T::MaxSensorDataLength>::try_from(data)
				.map_err(|_| Error::<T>::SensorDataTooLong)?;

			// Verify device ownership
			ensure!(DeviceOwnership::<T>::contains_key(&who, &device_id), Error::<T>::NotAuthorized);

			// Get device
			let mut device = IoTDevices::<T>::get(&device_id)
				.ok_or(Error::<T>::DeviceNotFound)?;

			// Check if device is active
			ensure!(device.status == DeviceStatus::Active, Error::<T>::DeviceNotActive);

			// Create sensor data
			let sensor_data = SensorData::<T> {
				device_id: device_id.clone(),
				sensor_id: sensor_id.clone(),
				sensor_type: sensor_type.clone(),
				value: data,
				timestamp: frame_system::Pallet::<T>::block_number(),
				quality_score,
				calibrated: true, // Assume calibrated for now
			};

			// Store sensor data
			DeviceSensorData::<T>::insert(&device_id, &sensor_id, sensor_data);

			// Update device transmission count
			device.transmission_count += 1;
			device.last_update = frame_system::Pallet::<T>::block_number();
			IoTDevices::<T>::insert(&device_id, device);

			// Emit event
			Self::deposit_event(Event::SensorDataReceived {
				device_id: device_id.clone(),
				sensor_id: sensor_id.clone(),
				sensor_type,
				quality_score,
			});

			Ok(())
		}

		/// Update device status
		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn update_device_status(
			origin: OriginFor<T>,
			device_id: Vec<u8>,
			new_status: DeviceStatus,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vector
			let device_id = BoundedVec::<u8, T::MaxDeviceIdLength>::try_from(device_id)
				.map_err(|_| Error::<T>::DeviceIdTooLong)?;

			// Get device
			let mut device = IoTDevices::<T>::get(&device_id)
				.ok_or(Error::<T>::DeviceNotFound)?;

			// Check authorization
			ensure!(device.owner == who, Error::<T>::NotAuthorized);

			// Update device status
			let old_status = device.status.clone();
			device.status = new_status.clone();
			device.last_update = frame_system::Pallet::<T>::block_number();

			// Store updated device
			IoTDevices::<T>::insert(&device_id, device);

			// Emit event
			Self::deposit_event(Event::DeviceStatusUpdated {
				device_id: device_id.clone(),
				old_status,
				new_status,
			});

			Ok(())
		}

		/// Create device group
		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_device_group(
			origin: OriginFor<T>,
			group_id: Vec<u8>,
			name: Vec<u8>,
			description: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let group_id = BoundedVec::<u8, T::MaxDeviceIdLength>::try_from(group_id.clone())
				.map_err(|_| Error::<T>::DeviceIdTooLong)?;
			let name = BoundedVec::<u8, T::MaxDeviceIdLength>::try_from(name)
				.map_err(|_| Error::<T>::DeviceIdTooLong)?;
			let description = BoundedVec::<u8, T::MaxDeviceMetadataLength>::try_from(description)
				.map_err(|_| Error::<T>::DeviceMetadataTooLong)?;

			// Check if group already exists
			ensure!(!DeviceGroups::<T>::contains_key(&group_id), Error::<T>::GroupAlreadyExists);

			// Create device group
			let device_group = DeviceGroup::<T> {
				group_id: group_id.clone(),
				name,
				description,
				owner: who.clone(),
				device_count: 0,
				status: GroupStatus::Active,
				created: frame_system::Pallet::<T>::block_number(),
			};

			// Store device group
			DeviceGroups::<T>::insert(&group_id, device_group);

			// Emit event
			Self::deposit_event(Event::DeviceGroupCreated {
				group_id: group_id.clone(),
				owner: who,
			});

			Ok(())
		}

		/// Add device to group
		#[pallet::call_index(4)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn add_device_to_group(
			origin: OriginFor<T>,
			group_id: Vec<u8>,
			device_id: Vec<u8>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vectors
			let group_id = BoundedVec::<u8, T::MaxDeviceIdLength>::try_from(group_id)
				.map_err(|_| Error::<T>::DeviceIdTooLong)?;
			let device_id = BoundedVec::<u8, T::MaxDeviceIdLength>::try_from(device_id)
				.map_err(|_| Error::<T>::DeviceIdTooLong)?;

			// Get group
			let mut group = DeviceGroups::<T>::get(&group_id)
				.ok_or(Error::<T>::GroupNotFound)?;

			// Check authorization (group owner)
			ensure!(group.owner == who, Error::<T>::NotAuthorized);

			// Get device
			let device = IoTDevices::<T>::get(&device_id)
				.ok_or(Error::<T>::DeviceNotFound)?;

			// Check device ownership
			ensure!(device.owner == who, Error::<T>::NotAuthorized);

			// Check if device already in group (simplified check)
			// In real implementation, would track group membership

			// Update group device count
			group.device_count += 1;

			// Store updated group
			DeviceGroups::<T>::insert(&group_id, group);

			// Emit event
			Self::deposit_event(Event::DeviceAddedToGroup {
				group_id: group_id.clone(),
				device_id: device_id.clone(),
			});

			Ok(())
		}

		/// Update device metadata
		#[pallet::call_index(5)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1, 1).ref_time())]
		pub fn update_device_metadata(
			origin: OriginFor<T>,
			device_id: Vec<u8>,
			new_location: Option<Vec<u8>>,
			new_metadata: Option<Vec<u8>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// Convert to bounded vector
			let device_id = BoundedVec::<u8, T::MaxDeviceIdLength>::try_from(device_id)
				.map_err(|_| Error::<T>::DeviceIdTooLong)?;

			// Get device
			let mut device = IoTDevices::<T>::get(&device_id)
				.ok_or(Error::<T>::DeviceNotFound)?;

			// Check authorization
			ensure!(device.owner == who, Error::<T>::NotAuthorized);

			// Update location if provided
			if let Some(location) = new_location {
				let location = BoundedVec::<u8, T::MaxDeviceMetadataLength>::try_from(location)
					.map_err(|_| Error::<T>::DeviceMetadataTooLong)?;
				device.location = location;
			}

			// Update metadata if provided
			if let Some(metadata) = new_metadata {
				let metadata = BoundedVec::<u8, T::MaxDeviceMetadataLength>::try_from(metadata)
					.map_err(|_| Error::<T>::DeviceMetadataTooLong)?;
				device.metadata = metadata;
			}

			// Update timestamp
			device.last_update = frame_system::Pallet::<T>::block_number();

			// Store updated device
			IoTDevices::<T>::insert(&device_id, device);

			// Emit event
			Self::deposit_event(Event::DeviceMetadataUpdated {
				device_id: device_id.clone(),
				owner: who,
			});

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Get device sensor data with quality filtering
		pub fn get_device_sensor_data_filtered(
			device_id: &BoundedVec<u8, T::MaxDeviceIdLength>,
			sensor_id: &BoundedVec<u8, T::MaxDeviceIdLength>,
			min_quality: u32,
		) -> Option<SensorData<T>> {
			if let Some(sensor_data) = DeviceSensorData::<T>::get(device_id, sensor_id) {
				if sensor_data.quality_score >= min_quality {
					Some(sensor_data)
				} else {
					None
				}
			} else {
				None
			}
		}

		/// Get devices by type
		pub fn get_devices_by_type(device_type: &DeviceType) -> Vec<BoundedVec<u8, T::MaxDeviceIdLength>> {
			let mut devices = Vec::new();
			for (device_id, device) in IoTDevices::<T>::iter() {
				if device.device_type == *device_type && device.status == DeviceStatus::Active {
					devices.push(device_id);
				}
			}
			devices
		}

		/// Get device transmission statistics
		pub fn get_device_statistics(device_id: &BoundedVec<u8, T::MaxDeviceIdLength>) -> Option<(u64, T::BlockNumber, T::BlockNumber)> {
			if let Some(device) = IoTDevices::<T>::get(device_id) {
				Some((device.transmission_count, device.registered, device.last_update))
			} else {
				None
			}
		}
	}
}