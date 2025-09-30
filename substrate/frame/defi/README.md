# DeFi Pallet

A FRAME pallet that provides comprehensive decentralized finance functionality including DEX, lending, and yield farming, developed by REChain Network Solutions LLC.

## Overview

The DeFi pallet enables:
- Token creation and management
- Automated market making (AMM) with liquidity pools
- Token swapping with AMM algorithm
- Lending and borrowing with interest rates
- Yield farming and liquidity mining
- Advanced DeFi strategies and protocols

## Features

- **Token Factory**: Create custom tokens with configurable parameters
- **AMM DEX**: Automated market maker with concentrated liquidity
- **Lending Protocol**: Full lending/borrowing system with interest rates
- **Yield Farming**: Liquidity incentives and reward distribution
- **Risk Management**: Collateralization ratios and liquidation protection
- **Fee Structure**: Configurable fees for all operations

## Interface

### Dispatchable Functions

- `create_token(name, symbol, decimals, total_supply)` - Create new token
- `create_liquidity_pool(pool_name, token_a, token_b, amount_a, amount_b)` - Create liquidity pool
- `add_liquidity(pool_name, amount_a, amount_b, min_liquidity)` - Add liquidity to pool
- `swap_tokens(pool_name, token_in, amount_in, min_amount_out)` - Swap tokens
- `create_lending_pool(token_symbol, interest_rate, reserve_factor)` - Create lending pool
- `deposit_to_lending_pool(token_symbol, amount)` - Deposit to lending pool
- `borrow_from_lending_pool(token_symbol, borrow_amount)` - Borrow from lending pool
- `repay_lending_pool(token_symbol, repay_amount)` - Repay lending pool

### Storage

- `Tokens`: Maps token symbols to token information
- `LiquidityPools`: Maps pool names to liquidity pool information
- `UserLiquidityPositions`: Maps users to their liquidity positions
- `LendingPools`: Maps token symbols to lending pool information
- `UserLendingPositions`: Maps users to their lending positions

### Events

- `TokenCreated`: Emitted when new token is created
- `LiquidityPoolCreated`: Emitted when liquidity pool is created
- `LiquidityAdded`: Emitted when liquidity is added
- `LiquidityRemoved`: Emitted when liquidity is removed
- `TokenSwapped`: Emitted when tokens are swapped
- `LendingPoolCreated`: Emitted when lending pool is created
- `DepositMade`: Emitted when deposit is made
- `LoanTaken`: Emitted when loan is taken
- `LoanRepaid`: Emitted when loan is repaid

### Errors

- `TokenAlreadyExists`: Token symbol already exists
- `TokenNotFound`: Token does not exist
- `PoolAlreadyExists`: Pool already exists
- `PoolNotFound`: Pool does not exist
- `InsufficientLiquidity`: Not enough liquidity for operation
- `InsufficientBalance`: Insufficient token balance
- `InvalidAmount`: Invalid amount specified
- `SlippageExceeded`: Price slippage tolerance exceeded
- `PoolRatioMismatch`: Token ratio mismatch
- `LendingPoolNotFound`: Lending pool does not exist
- `InsufficientCollateral`: Not enough collateral for loan
- `PositionNotFound`: Position does not exist
- `TokenSymbolTooLong`: Token symbol exceeds maximum length
- `PoolNameTooLong`: Pool name exceeds maximum length
- `Overflow`: Arithmetic overflow occurred
- `DivisionByZero`: Division by zero attempted

## Usage

### Prerequisites

Add the pallet to your runtime's `Cargo.toml`:

```toml
[dependencies]
pallet-defi = { version = "1.0.0-dev", default-features = false, path = "../../frame/defi" }
```

### Configuration

Configure the pallet in your runtime:

```rust
impl pallet_defi::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances; // Native currency
    type MaxTokenSymbolLength = ConstU32<32>;
    type MaxPoolNameLength = ConstU32<64>;
    type FeeNumerator = ConstU32<30>; // 0.3% fee
    type MinLiquidity = ConstU128<1000>;
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
        DeFi: pallet_defi,
        // ... other pallets
    }
);
```

## Example Usage

Create a new token:

```rust
let name = b"My Token".to_vec();
let symbol = b"MTK".to_vec();
let decimals = 18;
let total_supply = 1000000000000000000000000; // 1 million tokens

DeFi::create_token(
    RuntimeOrigin::signed(creator),
    name,
    symbol,
    decimals,
    total_supply
)?;
```

Create a liquidity pool:

```rust
let pool_name = b"MTK-RECH-Pool".to_vec();
let token_a = b"MTK".to_vec();
let token_b = b"RECH".to_vec();
let amount_a = 1000000000000000000000; // 1000 MTK
let amount_b = 1000000000000000000000; // 1000 RECH

DeFi::create_liquidity_pool(
    RuntimeOrigin::signed(creator),
    pool_name,
    token_a,
    token_b,
    amount_a,
    amount_b
)?;
```

Swap tokens:

```rust
let pool_name = b"MTK-RECH-Pool".to_vec();
let token_in = b"MTK".to_vec();
let amount_in = 100000000000000000000; // 100 MTK
let min_amount_out = 80000000000000000000; // Minimum 80 RECH expected

DeFi::swap_tokens(
    RuntimeOrigin::signed(user),
    pool_name,
    token_in,
    amount_in,
    min_amount_out
)?;
```

Create lending pool:

```rust
let token_symbol = b"RECH".to_vec();
let interest_rate = 50000000000000000; // 5% APY
let reserve_factor = 1000; // 10% reserve factor

DeFi::create_lending_pool(
    RuntimeOrigin::signed(admin),
    token_symbol,
    interest_rate,
    reserve_factor
)?;
```

## Advanced Features

### Yield Farming

The DeFi pallet supports yield farming through:
- **Liquidity Mining**: Rewards for providing liquidity
- **Staking Rewards**: Benefits for long-term positions
- **Performance Fees**: Additional rewards for high performers

### Risk Management

- **Over-collateralization**: Requires collateral exceeding loan value
- **Liquidation Protection**: Automatic liquidation for under-collateralized positions
- **Interest Rate Models**: Dynamic rates based on utilization
- **Reserve Requirements**: Minimum reserves for protocol stability

### Integration with Other Pallets

The DeFi pallet integrates seamlessly with:
- **Web3 Pallet**: DApp DeFi functionality
- **Web4 Pallet**: DeFi domain registration
- **Web5 Pallet**: DeFi credential verification
- **Bridge Pallet**: Cross-chain DeFi operations

Example integration:

```rust
// 1. Create DeFi token with Web3 DApp
DeFi::create_token(origin, name, symbol, decimals, supply)?;

// 2. Register domain for DeFi project with Web4
Web4::register_domain(origin, domain_name, content_hash)?;

// 3. Create DID for DeFi protocol with Web5
Web5::create_did(origin, protocol_did, public_keys, services)?;

// 4. Set up cross-chain DeFi with Bridge
Bridge::create_bridge(origin, bridge_id, target_chain, threshold, fee)?;
```

## Mathematical Models

### AMM Pricing Formula

The pallet uses the constant product formula:
```
x * y = k
```
Where:
- `x` and `y` are token reserves
- `k` is the constant product
- Price is determined by the ratio of reserves

### Interest Rate Calculation

Lending pools use dynamic interest rates:
```
utilization_rate = total_borrows / total_deposits
interest_rate = base_rate + (utilization_rate * multiplier)
```

## Security Considerations

- **Reentrancy Protection**: All state changes are atomic
- **Overflow Protection**: Safe arithmetic operations
- **Access Control**: Strict authorization checks
- **Input Validation**: Comprehensive parameter validation
- **Emergency Controls**: Circuit breakers for extreme situations

## Performance Optimization

- **Gas Efficiency**: Optimized storage and computation
- **Batch Operations**: Support for multiple operations
- **Efficient Lookup**: Indexed storage for fast queries
- **Scalable Design**: Handles high-volume DeFi operations