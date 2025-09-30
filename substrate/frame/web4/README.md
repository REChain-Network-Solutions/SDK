# Web4 Pallet

A FRAME pallet that provides Web4 functionality for decentralized web applications, developed by REChain Network Solutions LLC.

## Overview

The Web4 pallet enables:
- Registration of Web4 domains
- Storage of domain content hashes
- Domain ownership management
- Transfer of domain ownership

## Features

- **Domain Registration**: Register new Web4 domains with content hashes
- **Content Updates**: Update domain content hashes
- **Ownership Transfer**: Transfer domain ownership to other accounts
- **Access Control**: Only domain owners can update or transfer their domains

## Interface

### Dispatchable Functions

- `register_domain(domain, content_hash)` - Register a new Web4 domain
- `update_domain(domain, content_hash)` - Update domain content hash
- `transfer_domain(domain, to)` - Transfer domain ownership

### Storage

- `Domains`: Maps domain names to domain information

### Events

- `DomainRegistered`: Emitted when a new domain is registered
- `DomainUpdated`: Emitted when domain content is updated
- `DomainTransferred`: Emitted when domain ownership is transferred

### Errors

- `DomainAlreadyExists`: Domain name is already taken
- `DomainNotFound`: Domain does not exist
- `NotAuthorized`: Caller is not authorized to perform the action
- `DomainTooLong`: Domain name exceeds maximum length
- `ContentTooLong`: Content hash exceeds maximum length

## Usage

### Prerequisites

Add the pallet to your runtime's `Cargo.toml`:

```toml
[dependencies]
pallet-web4 = { version = "4.0.0-dev", default-features = false, path = "../../frame/web4" }
```

### Configuration

Configure the pallet in your runtime:

```rust
impl pallet_web4::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxDomainLength = ConstU32<255>;
    type MaxContentLength = ConstU32<1024>;
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
        Web4: pallet_web4,
        // ... other pallets
    }
);
```

## Example Usage

Register a new domain:

```rust
let domain = b"example.web4".to_vec();
let content_hash = b"QmYwAPJzv5CZsnAztmj5DrE8yQkCjXaieQaxZ".to_vec();

Web4::register_domain(RuntimeOrigin::signed(account), domain, content_hash)?;
```

Update domain content:

```rust
let domain = b"example.web4".to_vec();
let new_content_hash = b"QmYwAPJzv5CZsnAztmj5DrE8yQkCjXaieQaxZ_new".to_vec();

Web4::update_domain(RuntimeOrigin::signed(account), domain, new_content_hash)?;
```

Transfer domain ownership:

```rust
let domain = b"example.web4".to_vec();
let new_owner = AccountId::from([2u8; 32]);

Web4::transfer_domain(RuntimeOrigin::signed(account), domain, new_owner)?;