# Zero-Knowledge Proof (ZKP) Pallet

A FRAME pallet that provides zero-knowledge proof verification and privacy-preserving computations, developed by REChain Network Solutions LLC.

## Overview

The ZKP pallet enables:
- Zero-knowledge proof verification
- Privacy-preserving smart contracts
- Anonymous transactions and computations
- Multiple elliptic curve support
- Verifiable computation offloading
- Private data validation

## Features

- **ZKP Verification**: Verify zero-knowledge proofs on-chain
- **Multiple Curves**: Support for BLS12-381, BN254, BW6-761, and custom curves
- **Privacy Preservation**: Enable private transactions and computations
- **Gas Management**: Configurable gas limits for proof verification
- **Key Management**: Verification key registration and updates
- **Proof Lifecycle**: Complete proof submission and verification workflow

## Interface

### Dispatchable Functions

- `register_verification_key(proof_id, verification_key, description, curve_type)` - Register ZKP verification key
- `submit_zkp(proof_id, proof_data, public_inputs)` - Submit proof for verification
- `update_verification_key(proof_id, new_verification_key)` - Update verification key

### Storage

- `VerificationKeys`: Maps proof IDs to verification key information
- `VerifiedProofs`: Maps proof IDs to verification results

### Events

- `VerificationKeyRegistered`: Emitted when verification key is registered
- `ZKPSubmitted`: Emitted when ZKP is submitted for verification
- `ZKPVerified`: Emitted when ZKP verification completes
- `VerificationKeyUpdated`: Emitted when verification key is updated

### Errors

- `VerificationKeyAlreadyExists`: Verification key already exists
- `VerificationKeyNotFound`: Verification key not found
- `ProofAlreadyExists`: Proof already submitted
- `ProofNotFound`: Proof not found
- `NotAuthorized`: Account not authorized for operation
- `ProofIdTooLong`: Proof identifier exceeds maximum length
- `VerificationKeyTooLong`: Verification key exceeds maximum length
- `ProofDataTooLong`: Proof data exceeds maximum length
- `PublicInputsTooLong`: Public inputs exceed maximum length
- `VerificationFailed`: ZKP verification failed
- `GasLimitExceeded`: Proof verification exceeded gas limit
- `UnsupportedCurve`: Unsupported elliptic curve type
- `InvalidProofFormat`: Invalid proof format

## Usage

### Prerequisites

Add the pallet to your runtime's `Cargo.toml`:

```toml
[dependencies]
pallet-zkp = { version = "1.0.0-dev", default-features = false, path = "../../frame/zkp" }
```

### Configuration

Configure the pallet in your runtime:

```rust
impl pallet_zkp::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxProofIdLength = ConstU32<128>;
    type MaxVerificationKeyLength = ConstU32<2048>;
    type MaxProofDataLength = ConstU32<4096>;
    type MaxPublicInputsLength = ConstU32<1024>;
    type ProofVerificationGasLimit = ConstU64<100000>;
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
        ZKP: pallet_zkp,
        // ... other pallets
    }
);
```

## Example Usage

Register verification key:

```rust
let proof_id = b"private-voting-circuit".to_vec();
let verification_key = b"bls12381_vk_data...".to_vec();
let description = b"Circuit for private voting verification".to_vec();
let curve_type = CurveType::BLS12381;

ZKP::register_verification_key(
    RuntimeOrigin::signed(developer),
    proof_id,
    verification_key,
    description,
    curve_type
)?;
```

Submit ZKP for verification:

```rust
let proof_id = b"private-voting-circuit".to_vec();
let proof_data = b"zkp_proof_bytes...".to_vec();
let public_inputs = b"encrypted_vote_data...".to_vec();

ZKP::submit_zkp(
    RuntimeOrigin::signed(voter),
    proof_id,
    proof_data,
    public_inputs
)?;
```

## Supported Curve Types

The ZKP pallet supports multiple elliptic curves:

- **BLS12-381**: Popular for pairing-based cryptography
- **BN254**: Optimized for SNARKs and efficient verification
- **BW6-761**: Advanced curve for complex proof systems
- **Custom**: Extensible for specialized curve implementations

## Privacy Applications

### Private Transactions
```rust
// Submit private transaction proof
let proof_id = b"private-transfer".to_vec();
let proof_data = generate_private_transfer_proof(sender, recipient, amount)?;
let public_inputs = vec![recipient_hash, amount_commitment];

ZKP::submit_zkp(origin, proof_id, proof_data, public_inputs)?;
```

### Anonymous Voting
```rust
// Submit anonymous vote proof
let proof_id = b"anonymous-voting".to_vec();
let proof_data = generate_vote_proof(vote, voter_nullifier)?;
let public_inputs = vec![proposal_id, vote_commitment];

ZKP::submit_zkp(origin, proof_id, proof_data, public_inputs)?;
```

### Private Identity
```rust
// Verify private identity proof
let proof_id = b"private-id".to_vec();
let proof_data = generate_identity_proof(credentials, disclosure_selectors)?;
let public_inputs = vec![public_claims, issuer_signature];

ZKP::submit_zkp(origin, proof_id, proof_data, public_inputs)?;
```

## Integration with Privacy Systems

The ZKP pallet integrates with privacy-focused applications:

```rust
// 1. Register ZKP circuit for private DeFi
ZKP::register_verification_key(origin, defi_proof_id, vk, description, curve)?;

// 2. Submit private transaction
ZKP::submit_zkp(origin, defi_proof_id, proof_data, public_inputs)?;

// 3. Use verified result in DeFi pallet
if ZKP::get_proof_status(defi_proof_id) == Some(true) {
    DeFi::process_private_transaction(origin, transaction_data)?;
}
```

## Advanced ZKP Features

### Proof Composition
- **Recursive Proofs**: Verify proofs within proofs
- **Proof Aggregation**: Combine multiple proofs efficiently
- **Batch Verification**: Verify multiple proofs in single operation

### Gas Optimization
- **Configurable Limits**: Adjustable gas limits per proof type
- **Efficient Verification**: Optimized verification algorithms
- **Preprocessing**: Off-chain preprocessing for faster verification

### Security Features
- **Input Validation**: Comprehensive proof format validation
- **Gas Metering**: Protection against DoS via expensive proofs
- **Key Management**: Secure verification key lifecycle
- **Access Control**: Owner-controlled key management

## Performance Considerations

- **Curve Selection**: Choose optimal curve for specific use cases
- **Proof Size**: Balance between proof size and verification cost
- **Batch Processing**: Process multiple proofs efficiently
- **Caching**: Cache verification keys for repeated use

## Future Enhancements

The ZKP pallet is designed for extensibility:

- **New Curve Support**: Easy addition of new elliptic curves
- **Advanced Proof Systems**: Support for PLONK, Halo2, and other systems
- **Hardware Acceleration**: Integration with cryptographic hardware
- **Cross-Chain Verification**: Verification across multiple chains