# Web3 Pallet

A FRAME pallet that provides Web3 functionality for decentralized applications, smart contracts, and blockchain integration, developed by REChain Network Solutions LLC.

## Overview

The Web3 pallet enables:
- Registration of decentralized applications (DApps)
- Smart contract deployment tracking
- Gas optimization management
- Cross-chain DApp compatibility
- Contract metadata management

## Features

- **DApp Registry**: Register and manage decentralized applications
- **Contract Deployment**: Track smart contract deployments with metadata
- **Gas Optimization**: Monitor and optimize contract gas usage
- **Access Control**: Owner-based permission system for DApp management
- **Metadata Storage**: Store contract ABIs, bytecode hashes, and descriptions

## Interface

### Dispatchable Functions

- `register_dapp(dapp_name, description)` - Register a new DApp
- `deploy_contract(dapp_name, contract_metadata, bytecode_hash, abi_hash, gas_optimization)` - Deploy a smart contract
- `optimize_contract(dapp_name, contract_metadata, new_gas_optimization)` - Optimize contract gas usage
- `update_dapp(dapp_name, description)` - Update DApp metadata

### Storage

- `Dapps`: Maps DApp names to DApp information
- `ContractReferences`: Maps DApp names and contract metadata to contract information

### Events

- `DappRegistered`: Emitted when a new DApp is registered
- `DappUpdated`: Emitted when DApp metadata is updated
- `ContractDeployed`: Emitted when a smart contract is deployed
- `ContractOptimized`: Emitted when contract gas optimization is updated

### Errors

- `DappAlreadyExists`: DApp name is already registered
- `DappNotFound`: DApp does not exist
- `ContractAlreadyExists`: Contract metadata already exists
- `ContractNotFound`: Contract reference does not exist
- `NotAuthorized`: Caller is not authorized to perform the action
- `DappNameTooLong`: DApp name exceeds maximum length
- `ContractMetadataTooLong`: Contract metadata exceeds maximum length
- `MaxContractsReached`: Maximum contracts per DApp limit reached
- `InvalidGasOptimization`: Gas optimization level is invalid

## Usage

### Prerequisites

Add the pallet to your runtime's `Cargo.toml`:

```toml
[dependencies]
pallet-web3 = { version = "3.0.0-dev", default-features = false, path = "../../frame/web3" }
```

### Configuration

Configure the pallet in your runtime:

```rust
impl pallet_web3::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxDappNameLength = ConstU32<100>;
    type MaxContractMetadataLength = ConstU32<1024>;
    type MaxContractsPerDapp = ConstU32<50>;
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
        Web3: pallet_web3,
        // ... other pallets
    }
);
```

## Example Usage

Register a new DApp:

```rust
let dapp_name = b"my-defi-app".to_vec();
let description = b"A decentralized finance application".to_vec();

Web3::register_dapp(RuntimeOrigin::signed(account), dapp_name, description)?;
```

Deploy a smart contract:

```rust
let dapp_name = b"my-defi-app".to_vec();
let contract_metadata = b"erc20-token".to_vec();
let bytecode_hash = b"QmYwAPJzv5CZsnAztmj5DrE8yQkCjXaieQaxZ".to_vec();
let abi_hash = b"QmYwAPJzv5CZsnAztmj5DrE8yQkCjXaieQaxY".to_vec();
let gas_optimization = 800; // 80% optimization

Web3::deploy_contract(
    RuntimeOrigin::signed(account),
    dapp_name,
    contract_metadata,
    bytecode_hash,
    abi_hash,
    gas_optimization
)?;
```

## Integration with Web4 and Web5

The Web3 pallet is designed to work seamlessly with:
- **Web4 Pallet**: For domain registration and content hosting
- **Web5 Pallet**: For decentralized identity and credential management

Example cross-pallet integration:

```rust
// Register DApp with Web3
Web3::register_dapp(origin, dapp_name, description)?;

// Register domain with Web4
Web4::register_domain(origin, domain_name, content_hash)?;

// Create DID with Web5
Web5::create_did(origin, did, public_keys, services)?;