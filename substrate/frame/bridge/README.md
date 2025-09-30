# Bridge Pallet

A FRAME pallet that provides cross-chain bridge functionality for blockchain interoperability, developed by REChain Network Solutions LLC.

## Overview

The Bridge pallet enables:
- Cross-chain asset transfers
- Bridge configuration and management
- Validator-based transfer confirmation
- Asset mapping between different chains
- Fee management for cross-chain operations

## Features

- **Bridge Management**: Create and configure cross-chain bridges
- **Asset Mapping**: Map assets between different blockchains
- **Transfer Processing**: Handle cross-chain asset transfers with validation
- **Validator System**: Multi-signature validation for transfer confirmation
- **Fee Structure**: Configurable fees for bridge operations
- **Transfer Status Tracking**: Complete transfer lifecycle management

## Interface

### Dispatchable Functions

- `create_bridge(bridge_id, target_chain, threshold, fee_percentage)` - Create new cross-chain bridge
- `add_bridge_validator(bridge_id, validator)` - Add validator to bridge
- `create_asset_mapping(source_chain, source_asset, target_asset, conversion_rate, bridge_fee, min_transfer, max_transfer)` - Create asset mapping
- `initiate_cross_chain_transfer(bridge_id, source_asset, target_chain, target_asset, recipient, amount)` - Start cross-chain transfer
- `confirm_cross_chain_transfer(transfer_id)` - Confirm transfer (validator only)
- `complete_cross_chain_transfer(transfer_id)` - Complete transfer (operator only)

### Storage

- `BridgeConfigs`: Maps bridge IDs to bridge configurations
- `AssetMappings`: Maps source chains and assets to target asset information
- `PendingTransfers`: Maps transfer IDs to transfer information
- `BridgeValidators`: Maps bridge IDs to validator lists

### Events

- `BridgeCreated`: Emitted when a new bridge is created
- `BridgeUpdated`: Emitted when bridge configuration is updated
- `AssetMappingCreated`: Emitted when asset mapping is created
- `CrossChainTransferInitiated`: Emitted when transfer is initiated
- `CrossChainTransferConfirmed`: Emitted when transfer is confirmed
- `CrossChainTransferCompleted`: Emitted when transfer is completed
- `ValidatorAdded`: Emitted when validator is added to bridge
- `ValidatorRemoved`: Emitted when validator is removed from bridge

### Errors

- `BridgeAlreadyExists`: Bridge identifier already exists
- `BridgeNotFound`: Bridge does not exist
- `AssetMappingAlreadyExists`: Asset mapping already exists
- `AssetMappingNotFound`: Asset mapping does not exist
- `TransferNotFound`: Transfer does not exist
- `NotAuthorized`: Caller is not authorized to perform the action
- `BridgeInactive`: Bridge is not active
- `InsufficientValidators`: Not enough validators for operation
- `InvalidAmount`: Transfer amount is invalid
- `TransferAlreadyProcessed`: Transfer has already been processed
- `ChainIdTooLong`: Chain identifier exceeds maximum length
- `AssetIdTooLong`: Asset identifier exceeds maximum length
- `MaxValidatorsReached`: Maximum validators per bridge reached
- `ValidatorNotFound`: Validator not found
- `ThresholdTooHigh`: Validator threshold is too high

## Usage

### Prerequisites

Add the pallet to your runtime's `Cargo.toml`:

```toml
[dependencies]
pallet-bridge = { version = "1.0.0-dev", default-features = false, path = "../../frame/bridge" }
```

### Configuration

Configure the pallet in your runtime:

```rust
impl pallet_bridge::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxChainIdLength = ConstU32<64>;
    type MaxAssetIdLength = ConstU32<128>;
    type MaxValidatorsPerBridge = ConstU32<100>;
    type MinValidators = ConstU32<3>;
}
```

Add it to your runtime's pallets:

```rust
construct_runtime!(
    pub struct Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        // ... other pallets
        Bridge: pallet_bridge,
        // ... other pallets
    }
);
```

## Example Usage

Create a cross-chain bridge:

```rust
let bridge_id = b"ethereum-bridge".to_vec();
let target_chain = b"ethereum".to_vec();
let threshold = 5; // Require 5 validators
let fee_percentage = 50; // 0.5% fee

Bridge::create_bridge(
    RuntimeOrigin::signed(operator),
    bridge_id,
    target_chain,
    threshold,
    fee_percentage
)?;
```

Create asset mapping:

```rust
let source_chain = b"rechain".to_vec();
let source_asset = b"RECH".to_vec();
let target_asset = b"ETH".to_vec();
let conversion_rate = 1000000000000000000; // 1 RECH = 1 ETH in wei
let bridge_fee = 1000000000000000; // 0.001 ETH fee
let min_transfer = 1000000000000000000; // 1 ETH minimum
let max_transfer = 100000000000000000000; // 100 ETH maximum

Bridge::create_asset_mapping(
    RuntimeOrigin::signed(operator),
    source_chain,
    source_asset,
    target_asset,
    conversion_rate,
    bridge_fee,
    min_transfer,
    max_transfer
)?;
```

Initiate cross-chain transfer:

```rust
let bridge_id = b"ethereum-bridge".to_vec();
let source_asset = b"RECH".to_vec();
let target_chain = b"ethereum".to_vec();
let target_asset = b"ETH".to_vec();
let recipient = b"0x742d35Cc6634C0532925a3b8D0007f0d6b5c2".to_vec();
let amount = 1000000000000000000; // 1 RECH

Bridge::initiate_cross_chain_transfer(
    RuntimeOrigin::signed(user),
    bridge_id,
    source_asset,
    target_chain,
    target_asset,
    recipient,
    amount
)?;
```

## Integration with Web Ecosystem

The Bridge pallet is designed to work with:
- **Web3 Pallet**: For DApp cross-chain functionality
- **Web4 Pallet**: For cross-chain domain resolution
- **Web5 Pallet**: For cross-chain identity verification

Example integration:

```rust
// 1. Create bridge for DApp
Bridge::create_bridge(origin, bridge_id, target_chain, threshold, fee)?;

// 2. Add DApp validators
for validator in dapp_validators {
    Bridge::add_bridge_validator(origin, bridge_id.clone(), validator)?;
}

// 3. Create asset mapping for DApp token
Bridge::create_asset_mapping(origin, source_chain, token_symbol, target_asset, rate, fee, min, max)?;

// 4. Initiate cross-chain DApp interaction
Bridge::initiate_cross_chain_transfer(origin, bridge_id, source_asset, target_chain, target_asset, recipient, amount)?;
```

## Security Features

- **Multi-signature validation**: Requires minimum validator confirmations
- **Transfer amount limits**: Prevents excessively large transfers
- **Fee validation**: Ensures proper fee collection
- **Authorization checks**: Strict permission controls
- **Status tracking**: Complete transfer lifecycle monitoring

## Performance Considerations

- **Gas optimization**: Efficient storage and computation
- **Batch processing**: Support for multiple transfers
- **Validator management**: Scalable validator sets
- **Asset mapping**: Efficient cross-chain asset lookup