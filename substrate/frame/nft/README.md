# NFT Pallet

A FRAME pallet that provides comprehensive non-fungible token (NFT) marketplace functionality, developed by REChain Network Solutions LLC.

## Overview

The NFT pallet enables:
- NFT collection creation and management
- Token minting with metadata and royalties
- Marketplace for buying and selling NFTs
- Royalty enforcement for creators
- Collection-based organization
- Advanced marketplace features

## Features

- **Collection Management**: Create and configure NFT collections
- **Token Minting**: Mint NFTs with metadata and royalty settings
- **Marketplace**: Full marketplace with listings and sales
- **Royalty System**: Automatic royalty payments to creators
- **Transfer Management**: Secure NFT ownership transfers
- **Listing Management**: Time-based and flexible listings

## Interface

### Dispatchable Functions

- `create_nft_collection(name, description, max_supply, royalty_percentage)` - Create NFT collection
- `mint_nft(collection, token_id, metadata_uri, royalty_percentage, to)` - Mint NFT token
- `list_nft(collection, token_id, price, duration)` - List NFT on marketplace
- `purchase_nft(collection, token_id)` - Purchase NFT from marketplace
- `transfer_nft(collection, token_id, to)` - Transfer NFT ownership
- `cancel_nft_listing(collection, token_id)` - Cancel marketplace listing

### Storage

- `NFTCollections`: Maps collection names to collection information
- `NFTTokens`: Maps collection and token ID to token information
- `TokenOwnership`: Maps owners to their token ownership records
- `MarketplaceListings`: Maps collection and token ID to marketplace listings

### Events

- `NFTCollectionCreated`: Emitted when new collection is created
- `NFTMinted`: Emitted when new NFT is minted
- `NFTTransferred`: Emitted when NFT ownership changes
- `NFTListed`: Emitted when NFT is listed on marketplace
- `NFTSold`: Emitted when NFT is sold
- `NFTListingCancelled`: Emitted when listing is cancelled
- `RoyaltyPaid`: Emitted when royalty is paid to creator

### Errors

- `CollectionAlreadyExists`: Collection name already exists
- `CollectionNotFound`: Collection does not exist
- `TokenAlreadyExists`: Token ID already exists in collection
- `TokenNotFound`: Token does not exist
- `NotAuthorized`: Caller is not authorized to perform action
- `CollectionNameTooLong`: Collection name exceeds maximum length
- `TokenMetadataTooLong`: Token metadata exceeds maximum length
- `MaxSupplyReached`: Collection maximum supply reached
- `ListingAlreadyExists`: NFT already listed on marketplace
- `ListingNotFound`: Listing does not exist
- `ListingExpired`: Marketplace listing has expired
- `InsufficientBalance`: Insufficient balance for purchase
- `InvalidRoyalty`: Invalid royalty percentage
- `TransferFailed`: NFT transfer failed

## Usage

### Prerequisites

Add the pallet to your runtime's `Cargo.toml`:

```toml
[dependencies]
pallet-nft = { version = "1.0.0-dev", default-features = false, path = "../../frame/nft" }
```

### Configuration

Configure the pallet in your runtime:

```rust
impl pallet_nft::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances; // Native currency for marketplace
    type MaxCollectionNameLength = ConstU32<64>;
    type MaxTokenMetadataLength = ConstU32<512>;
    type MaxRoyaltyPercentage = ConstU32<1000>; // 10% max royalty
    type MarketplaceFee = ConstU32<250>; // 2.5% marketplace fee
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
        NFT: pallet_nft,
        // ... other pallets
    }
);
```

## Example Usage

Create an NFT collection:

```rust
let name = b"My Art Collection".to_vec();
let description = b"A collection of unique digital art pieces".to_vec();
let max_supply = Some(10000); // Limited to 10,000 tokens
let royalty_percentage = 500; // 5% royalty

NFT::create_nft_collection(
    RuntimeOrigin::signed(creator),
    name,
    description,
    max_supply,
    royalty_percentage
)?;
```

Mint an NFT:

```rust
let collection = b"My Art Collection".to_vec();
let token_id = 1;
let metadata_uri = b"ipfs://QmYwAPJzv5CZsnAztmj5DrE8yQkCjXaieQaxZ".to_vec();
let royalty_percentage = 500; // 5% royalty for this token
let recipient = some_account;

NFT::mint_nft(
    RuntimeOrigin::signed(creator),
    collection,
    token_id,
    metadata_uri,
    royalty_percentage,
    recipient
)?;
```

List NFT on marketplace:

```rust
let collection = b"My Art Collection".to_vec();
let token_id = 1;
let price = 1000000000000000000; // 1 RECH
let duration = Some(7 * 24 * 60 * 10); // 7 days

NFT::list_nft(
    RuntimeOrigin::signed(owner),
    collection,
    token_id,
    price,
    duration
)?;
```

Purchase NFT:

```rust
let collection = b"My Art Collection".to_vec();
let token_id = 1;

NFT::purchase_nft(
    RuntimeOrigin::signed(buyer),
    collection,
    token_id
)?;
```

## Royalty System

The NFT pallet implements automatic royalty payments:

- **Creator Royalties**: Percentage of each sale paid to original creator
- **Maximum Limits**: Configurable maximum royalty percentages
- **Automatic Enforcement**: Royalties paid automatically on marketplace sales
- **Multiple Sales**: Royalties apply to all secondary market transactions

Royalty calculation:
```
royalty_amount = (sale_price * royalty_percentage) / 10000
seller_amount = sale_price - marketplace_fee - royalty_amount
```

## Integration with Web Ecosystem

The NFT pallet integrates with:
- **Web3 Pallet**: NFT DApp development
- **Web4 Pallet**: NFT domain registration and metadata hosting
- **Web5 Pallet**: NFT creator identity verification
- **DeFi Pallet**: NFT-collateralized lending
- **Bridge Pallet**: Cross-chain NFT transfers

Example integration:

```rust
// 1. Create NFT collection with Web3 DApp
NFT::create_nft_collection(origin, name, description, max_supply, royalty)?;

// 2. Register domain for collection with Web4
Web4::register_domain(origin, collection_domain, metadata_hash)?;

// 3. Create creator identity with Web5
Web5::create_did(origin, creator_did, public_keys, services)?;

// 4. Issue creator credentials with Web5
Web5::issue_credential(origin, creator_did, credential_id, credential_type, data, None)?;

// 5. Set up DeFi lending with NFT collateral
DeFi::create_lending_pool(origin, collateral_token, interest_rate, reserve_factor)?;

// 6. Enable cross-chain NFT transfers with Bridge
Bridge::create_bridge(origin, nft_bridge_id, target_chain, threshold, fee)?;
```

## Advanced Marketplace Features

### Dynamic Pricing
- **Time-based listings**: Expiring marketplace listings
- **Price discovery**: Market-driven pricing mechanisms
- **Bulk operations**: Multiple NFT operations in single transaction

### Collection Management
- **Supply control**: Limited edition collections
- **Metadata updates**: Dynamic NFT metadata
- **Batch minting**: Efficient multiple token creation

### Creator Economy
- **Royalty enforcement**: Automatic secondary sale royalties
- **Creator verification**: Web5-based creator identity
- **Revenue sharing**: Multiple creator revenue streams

## Security Features

- **Ownership verification**: Cryptographic ownership proofs
- **Royalty protection**: Enforced creator compensation
- **Marketplace security**: Secure listing and purchase mechanisms
- **Access control**: Strict authorization for all operations
- **Input validation**: Comprehensive parameter validation

## Performance Considerations

- **Efficient storage**: Optimized data structures for NFT metadata
- **Batch operations**: Support for multiple NFT operations
- **Gas optimization**: Efficient marketplace transactions
- **Scalable design**: Handles large collections and high transaction volumes