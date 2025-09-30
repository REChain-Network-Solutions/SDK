# Oracle Pallet

A FRAME pallet that provides oracle functionality for external data feeds and price oracles, developed by REChain Network Solutions LLC.

## Overview

The Oracle pallet enables:
- External data feed management
- Price oracle functionality
- Multi-source data aggregation
- Oracle operator incentivization
- Data freshness validation
- Decentralized data provision

## Features

- **Oracle Feeds**: Configurable data feeds with multiple operators
- **Data Aggregation**: Median-based aggregation of operator submissions
- **Operator Management**: Oracle operator registration and management
- **Incentivization**: Rewards for accurate and timely data provision
- **Freshness Checks**: Data staleness validation and controls
- **Access Control**: Permission-based oracle management

## Interface

### Dispatchable Functions

- `create_oracle_feed(feed_name, description, required_operators, max_staleness, min_submissions, reward_amount)` - Create oracle feed
- `add_oracle_operator(feed_name, operator)` - Add operator to feed
- `submit_oracle_data(feed_name, key, value, raw_value)` - Submit oracle data
- `get_oracle_value(feed_name, key)` - Get oracle value (read-only)

### Storage

- `OracleFeeds`: Maps feed names to oracle feed configurations
- `OracleData`: Maps feed and key to oracle value information
- `OracleOperators`: Maps feeds to their operator lists
- `OperatorStakes`: Maps operators to their stakes per feed

### Events

- `OracleFeedCreated`: Emitted when new oracle feed is created
- `OracleOperatorAdded`: Emitted when operator is added to feed
- `OracleDataSubmitted`: Emitted when oracle data is submitted
- `OracleValueUpdated`: Emitted when oracle value is updated
- `OracleOperatorRewarded`: Emitted when operator is rewarded

### Errors

- `OracleFeedAlreadyExists`: Oracle feed already exists
- `OracleFeedNotFound`: Oracle feed does not exist
- `OracleDataNotFound`: Oracle data not found
- `NotAuthorized`: Account not authorized for operation
- `FeedNameTooLong`: Feed name exceeds maximum length
- `DataKeyTooLong`: Data key exceeds maximum length
- `DataValueTooLong`: Data value exceeds maximum length
- `OperatorAlreadyExists`: Operator already exists in feed
- `OperatorNotFound`: Operator not found in feed
- `InsufficientOperators`: Not enough operators for feed
- `MaxOperatorsReached`: Maximum operators reached
- `DataTooStale`: Oracle data is too stale
- `InsufficientStake`: Insufficient operator stake
- `InvalidOperatorCount`: Invalid operator count specified

## Usage

### Prerequisites

Add the pallet to your runtime's `Cargo.toml`:

```toml
[dependencies]
pallet-oracle = { version = "1.0.0-dev", default-features = false, path = "../../frame/oracle" }
```

### Configuration

Configure the pallet in your runtime:

```rust
impl pallet_oracle::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances; // Native currency for rewards
    type MaxFeedNameLength = ConstU32<64>;
    type MaxDataKeyLength = ConstU32<128>;
    type MaxOracleOperators = ConstU32<50>;
    type MinOracleOperators = ConstU32<3>;
    type OracleReward = ConstU128<1000000000000000000>; // 1 token reward
    type MaxDataValueLength = ConstU32<1024>;
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
        Oracle: pallet_oracle,
        // ... other pallets
    }
);
```

## Example Usage

Create an oracle feed:

```rust
let feed_name = b"ETH-USD-Price".to_vec();
let description = b"Ethereum USD price feed".to_vec();
let required_operators = 5;
let max_staleness = 3600; // 1 hour in blocks
let min_submissions = 3;
let reward_amount = 1000000000000000000; // 1 token

Oracle::create_oracle_feed(
    RuntimeOrigin::signed(creator),
    feed_name,
    description,
    required_operators,
    max_staleness,
    min_submissions,
    reward_amount
)?;
```

Add oracle operator:

```rust
let feed_name = b"ETH-USD-Price".to_vec();
let operator = oracle_operator_account;

Oracle::add_oracle_operator(
    RuntimeOrigin::signed(feed_owner),
    feed_name,
    operator
)?;
```

Submit oracle data:

```rust
let feed_name = b"ETH-USD-Price".to_vec();
let key = b"latest_price".to_vec();
let value = b"2000.50".to_vec(); // Human readable
let raw_value = 2000500000000000000000; // Wei equivalent

Oracle::submit_oracle_data(
    RuntimeOrigin::signed(operator),
    feed_name,
    key,
    value,
    raw_value
)?;
```

## Data Aggregation

The Oracle pallet uses median-based aggregation:

```rust
// Example aggregation logic
let mut values: Vec<u128> = submissions.iter().map(|(_, val)| *val).collect();
values.sort();
let median = if values.len() % 2 == 0 {
    (values[mid-1] + values[mid]) / 2
} else {
    values[mid]
};
```

## Integration with DeFi

The Oracle pallet is essential for DeFi applications:

```rust
// 1. Create price feed oracle
Oracle::create_oracle_feed(origin, feed_name, description, operators, staleness, submissions, reward)?;

// 2. Use oracle data in DeFi pallet
let price = Oracle::get_median_value(feed_name, key);
match price {
    Some(valid_price) => {
        // Use price for lending ratios, liquidations, etc.
        DeFi::update_lending_rates(valid_price)?;
    },
    None => {
        // Handle missing price data
        return Err("Price data unavailable".into());
    }
}
```

## Security Features

- **Multi-Operator Validation**: Requires multiple independent data sources
- **Stake-Based Incentives**: Operators stake tokens for honest behavior
- **Data Freshness**: Automatic staleness detection and handling
- **Reward Distribution**: Automatic reward payment for valid submissions
- **Access Control**: Strict authorization for feed management

## Performance Optimization

- **Efficient Aggregation**: Fast median calculation for large operator sets
- **Gas Optimization**: Minimal gas usage for data submissions
- **Storage Optimization**: Compact data structures for oracle values
- **Scalable Design**: Supports multiple feeds with many operators