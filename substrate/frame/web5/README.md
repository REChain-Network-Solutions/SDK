# Web5 Pallet

A FRAME pallet that provides Web5 functionality for decentralized identity, self-sovereign identity, and verifiable credentials, developed by REChain Network Solutions LLC.

## Overview

The Web5 pallet enables:
- Decentralized Identity (DID) management
- Verifiable Credentials issuance and verification
- Self-sovereign identity features
- Service endpoints for identity interactions
- Credential revocation and management

## Features

- **DID Registry**: Create and manage decentralized identities
- **Verifiable Credentials**: Issue, verify, and revoke credentials
- **Service Management**: Add and manage service endpoints for DIDs
- **Credential Lifecycle**: Complete credential management from issuance to revocation
- **Access Control**: Controller-based permission system for identity management
- **Expiration Support**: Time-based credential expiration

## Interface

### Dispatchable Functions

- `create_did(did, public_keys, services)` - Create a new decentralized identity
- `issue_credential(subject_did, credential_id, credential_type, data_hash, expires_at)` - Issue a verifiable credential
- `revoke_credential(subject_did, credential_id)` - Revoke a verifiable credential
- `add_service(did, service_id, service_type, service_endpoint)` - Add a service to a DID

### Storage

- `DecentralizedIds`: Maps DID identifiers to identity information
- `VerifiableCredentials`: Maps DID and credential ID to credential information

### Events

- `DidCreated`: Emitted when a new DID is created
- `DidUpdated`: Emitted when DID information is updated
- `CredentialIssued`: Emitted when a verifiable credential is issued
- `CredentialRevoked`: Emitted when a credential is revoked
- `ServiceAdded`: Emitted when a service is added to a DID

### Errors

- `DidAlreadyExists`: DID identifier is already registered
- `DidNotFound`: DID does not exist
- `CredentialAlreadyExists`: Credential ID already exists
- `CredentialNotFound`: Credential does not exist
- `NotAuthorized`: Caller is not authorized to perform the action
- `DidTooLong`: DID identifier exceeds maximum length
- `IdentityMetadataTooLong`: Identity metadata exceeds maximum length
- `MaxCredentialsReached`: Maximum credentials per identity limit reached
- `CredentialExpired`: Credential has expired
- `CredentialAlreadyRevoked`: Credential is already revoked
- `InvalidService`: Invalid service information provided

## Usage

### Prerequisites

Add the pallet to your runtime's `Cargo.toml`:

```toml
[dependencies]
pallet-web5 = { version = "5.0.0-dev", default-features = false, path = "../../frame/web5" }
```

### Configuration

Configure the pallet in your runtime:

```rust
impl pallet_web5::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxDidLength = ConstU32<100>;
    type MaxIdentityMetadataLength = ConstU32<1024>;
    type MaxCredentialsPerIdentity = ConstU32<100>;
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
        Web5: pallet_web5,
        // ... other pallets
    }
);
```

## Example Usage

Create a new DID:

```rust
let did = b"did:rechain:1234567890abcdef".to_vec();
let public_keys = vec![
    b"key-1-public-hash".to_vec(),
    b"key-2-public-hash".to_vec(),
];
let services = vec![
    (
        b"credential-service".to_vec(),
        b"CredentialRepository".to_vec(),
        b"https://credentials.rechain.network".to_vec(),
    ),
];

Web5::create_did(RuntimeOrigin::signed(account), did, public_keys, services)?;
```

Issue a verifiable credential:

```rust
let subject_did = b"did:rechain:1234567890abcdef".to_vec();
let credential_id = b"university-degree-2024".to_vec();
let credential_type = b"EducationalCredential".to_vec();
let data_hash = b"QmYwAPJzv5CZsnAztmj5DrE8yQkCjXaieQaxZ".to_vec();
let expires_at = Some(block_number + (365 * 24 * 60 * 10)); // 1 year from now

Web5::issue_credential(
    RuntimeOrigin::signed(issuer_account),
    subject_did,
    credential_id,
    credential_type,
    data_hash,
    expires_at
)?;
```

## Integration with Web3 and Web4

The Web5 pallet is designed to work seamlessly with:
- **Web3 Pallet**: For DApp credential verification and authorization
- **Web4 Pallet**: For hosting identity-related content and metadata

Example cross-pallet integration:

```rust
// 1. Create DID with Web5
Web5::create_did(origin, did, public_keys, services)?;

// 2. Register DApp with Web3
Web3::register_dapp(origin, dapp_name, description)?;

// 3. Issue credential for DApp access
Web5::issue_credential(origin, user_did, credential_id, credential_type, data_hash, None)?;

// 4. Register domain for identity hosting with Web4
Web4::register_domain(origin, domain_name, identity_content_hash)?;
```

## Web5 Standards Compliance

The Web5 pallet implements key Web5 standards:
- **DID Core**: W3C Decentralized Identifiers specification
- **VC Data Model**: W3C Verifiable Credentials Data Model
- **DID Methods**: Extensible DID method support
- **Credential Exchange**: Standards-compliant credential formats

## Security Considerations

- Credentials can only be revoked by their issuer
- DID controllers have full authority over their identity
- Expired credentials cannot be used
- Service endpoints are validated before addition
- Public keys are stored as hashes for privacy