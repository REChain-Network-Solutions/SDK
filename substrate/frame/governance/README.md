# Governance Pallet

A FRAME pallet that provides advanced governance systems including democracy, council management, and treasury functionality, developed by REChain Network Solutions LLC.

## Overview

The Governance pallet enables:
- Democratic proposal and voting system
- Council member management and administration
- Treasury proposal and funding system
- Decentralized decision-making processes
- Weighted voting based on stake or reputation
- Proposal lifecycle management

## Features

- **Democratic Proposals**: Community-driven proposal system with voting
- **Council Management**: Elected/appointed council with administrative powers
- **Treasury System**: Community treasury with proposal funding
- **Voting Mechanisms**: Multiple voting strategies (stake-weighted, equal voting)
- **Proposal Lifecycle**: Complete proposal management from creation to execution
- **Access Control**: Role-based permissions for different operations

## Interface

### Dispatchable Functions

- `create_proposal(title, description, metadata_hash)` - Create governance proposal
- `vote_on_proposal(proposal_id, direction)` - Vote on active proposal
- `add_council_member(member, name, role)` - Add council member
- `create_treasury_proposal(beneficiary, amount, description)` - Create treasury spending proposal
- `approve_treasury_proposal(treasury_proposal_id)` - Approve treasury proposal (council only)
- `execute_proposal(proposal_id)` - Execute approved proposal (council only)

### Storage

- `Proposals`: Maps proposal IDs to proposal information
- `ProposalVotes`: Maps proposal and voter to vote information
- `CouncilMembers`: Maps members to council member information
- `TreasuryProposals`: Maps treasury proposal IDs to treasury proposals
- `TreasuryBalance`: Current treasury balance
- `NextProposalId`: Counter for proposal IDs

### Events

- `ProposalCreated`: Emitted when new proposal is created
- `VoteCast`: Emitted when vote is cast on proposal
- `ProposalStatusChanged`: Emitted when proposal status changes
- `CouncilMemberAdded`: Emitted when council member is added
- `CouncilMemberRemoved`: Emitted when council member is removed
- `TreasuryProposalCreated`: Emitted when treasury proposal is created
- `TreasuryProposalApproved`: Emitted when treasury proposal is approved
- `TreasurySpent`: Emitted when treasury funds are spent

### Errors

- `ProposalAlreadyExists`: Proposal ID already exists
- `ProposalNotFound`: Proposal does not exist
- `InsufficientDeposit`: Insufficient deposit for proposal creation
- `VotingPeriodEnded`: Voting period has ended
- `AlreadyVoted`: Account has already voted on proposal
- `NotAuthorized`: Account not authorized for operation
- `ProposalTitleTooLong`: Proposal title exceeds maximum length
- `ProposalDescriptionTooLong`: Proposal description exceeds maximum length
- `CouncilFull`: Council has reached maximum membership
- `CouncilMemberNotFound`: Council member not found
- `TreasuryProposalNotFound`: Treasury proposal not found
- `InsufficientTreasuryBalance`: Treasury has insufficient balance
- `InvalidProposalStatus`: Proposal is in invalid status
- `CouncilVoteRequired`: Council vote required for operation

## Usage

### Prerequisites

Add the pallet to your runtime's `Cargo.toml`:

```toml
[dependencies]
pallet-governance = { version = "1.0.0-dev", default-features = false, path = "../../frame/governance" }
```

### Configuration

Configure the pallet in your runtime:

```rust
impl pallet_governance::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances; // Native currency for deposits
    type MaxProposalTitleLength = ConstU32<128>;
    type MaxProposalDescriptionLength = ConstU32<1024>;
    type MinimumProposalDeposit = ConstU128<1000000000000000000>; // 1 token
    type VotingPeriod = ConstU32<7200>; // 1 hour (assuming 6s blocks)
    type EnactmentDelay = ConstU32<7200>; // 1 hour delay
    type MaxCouncilMembers = ConstU32<23>;
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
        Governance: pallet_governance,
        // ... other pallets
    }
);
```

## Example Usage

Create a governance proposal:

```rust
let title = b"Improve Network Security".to_vec();
let description = b"This proposal aims to enhance network security through additional validation measures.".to_vec();
let metadata_hash = Some(b"QmYwAPJzv5CZsnAztmj5DrE8yQkCjXaieQaxZ".to_vec());

Governance::create_proposal(
    RuntimeOrigin::signed(proposer),
    title,
    description,
    metadata_hash
)?;
```

Vote on a proposal:

```rust
let proposal_id = 1;
let vote_direction = VoteDirection::Aye;

Governance::vote_on_proposal(
    RuntimeOrigin::signed(voter),
    proposal_id,
    vote_direction
)?;
```

Create treasury proposal:

```rust
let beneficiary = some_account;
let amount = 50000000000000000000000; // 50,000 tokens
let description = b"Development grant for new DeFi protocol".to_vec();

Governance::create_treasury_proposal(
    RuntimeOrigin::signed(proposer),
    beneficiary,
    amount,
    description
)?;
```

## Integration with Other Pallets

The Governance pallet integrates with:
- **Web3 Pallet**: DApp governance proposals
- **Web4 Pallet**: Domain governance decisions
- **Web5 Pallet**: Identity governance and credential standards
- **DeFi Pallet**: Treasury funding for DeFi initiatives
- **Bridge Pallet**: Cross-chain governance decisions
- **Oracle Pallet**: Oracle governance and data validation

Example integration:

```rust
// 1. Create governance proposal for new feature
Governance::create_proposal(origin, title, description, metadata)?;

// 2. Vote on proposal with Web5 credentials
Governance::vote_on_proposal(origin, proposal_id, direction)?;

// 3. Fund approved proposal through treasury
Governance::create_treasury_proposal(origin, beneficiary, amount, description)?;

// 4. Execute approved proposal
Governance::execute_proposal(origin, proposal_id)?;
```

## Advanced Governance Features

### Proposal Types
- **Technical Proposals**: Runtime upgrades, parameter changes
- **Treasury Proposals**: Spending requests, budget allocations
- **Community Proposals**: General community initiatives
- **Emergency Proposals**: Fast-tracked critical updates

### Voting Mechanisms
- **Stake-Weighted Voting**: Vote weight based on token holdings
- **Equal Voting**: One account, one vote
- **Quadratic Voting**: Vote weight based on square root of stake
- **Conviction Voting**: Time-locked votes with increased weight

### Treasury Management
- **Proposal System**: Community-driven spending proposals
- **Council Approval**: Multi-signature treasury approvals
- **Budget Tracking**: Real-time treasury balance monitoring
- **Spending Limits**: Configurable spending constraints

## Security Considerations

- **Proposal Validation**: Comprehensive proposal parameter validation
- **Vote Verification**: Cryptographic vote verification
- **Access Control**: Strict role-based permissions
- **Emergency Controls**: Circuit breakers for governance emergencies
- **Audit Trail**: Complete governance action logging

## Performance Optimization

- **Efficient Storage**: Optimized data structures for governance data
- **Batch Operations**: Support for multiple governance operations
- **Gas Optimization**: Efficient governance transaction processing
- **Scalable Design**: Handles large numbers of proposals and votes