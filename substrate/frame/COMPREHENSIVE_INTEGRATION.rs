//! Comprehensive Integration Example - REChain Network Solutions LLC
//!
//! This file demonstrates how all pallets work together to create
//! a complete decentralized ecosystem.

use frame_support::{decl_module, decl_storage, decl_event, dispatch::DispatchResult};
use frame_system::ensure_signed;
use sp_runtime::traits::Hash;

/// Complete Ecosystem Integration Example
pub struct CompleteEcosystemIntegration;

impl CompleteEcosystemIntegration {
    /// Set up a complete decentralized application ecosystem
    pub fn setup_complete_dapp_ecosystem(
        origin: RuntimeOrigin,
        founder: AccountId,
        dapp_name: &str,
        token_symbol: &str,
        initial_supply: u128,
    ) -> DispatchResult {
        let who = ensure_signed(origin.clone())?;

        // Step 1: Create DApp developer identity (Web5)
        let developer_did = format!("did:rechain:developer:{}", founder);
        let public_keys = vec![
            b"ed25519-key-developer".to_vec(),
            b"x25519-key-auth".to_vec(),
        ];
        let services = vec![
            (
                b"developer-service".to_vec(),
                b"DeveloperProfile".to_vec(),
                format!("https://{}.dev.rechain.network", dapp_name).as_bytes().to_vec(),
            ),
        ];

        Web5::create_did(
            origin.clone(),
            developer_did.as_bytes().to_vec(),
            public_keys,
            services,
        )?;

        // Step 2: Issue developer credentials (Web5)
        let credential_id = format!("developer-cert-{}", dapp_name).as_bytes().to_vec();
        let credential_type = b"DeveloperCredential".to_vec();
        let credential_data = format!("Certified developer for {}", dapp_name).as_bytes().to_vec();

        Web5::issue_credential(
            origin.clone(),
            developer_did.as_bytes().to_vec(),
            credential_id,
            credential_type,
            credential_data,
            None,
        )?;

        // Step 3: Register DApp (Web3)
        let dapp_name_bytes = dapp_name.as_bytes().to_vec();
        let dapp_description = format!("Complete DApp ecosystem by {}", founder).as_bytes().to_vec();

        Web3::register_dapp(
            origin.clone(),
            dapp_name_bytes.clone(),
            dapp_description,
        )?;

        // Step 4: Create DApp token (DeFi)
        let token_name = format!("{} Token", dapp_name).as_bytes().to_vec();
        let token_symbol_bytes = token_symbol.as_bytes().to_vec();

        DeFi::create_token(
            origin.clone(),
            token_name,
            token_symbol_bytes.clone(),
            18, // decimals
            initial_supply,
        )?;

        // Step 5: Create liquidity pool for token (DeFi)
        let pool_name = format!("{}-RECH-Pool", token_symbol).as_bytes().to_vec();
        let token_a = token_symbol_bytes.clone();
        let token_b = b"RECH".to_vec(); // Native token

        DeFi::create_liquidity_pool(
            origin.clone(),
            pool_name,
            token_a,
            token_b,
            initial_supply / 10, // 10% for liquidity
            initial_supply / 10,  // 10% for liquidity
        )?;

        // Step 6: Create lending pool for token (DeFi)
        let lending_token = token_symbol_bytes.clone();
        let interest_rate = 50000000000000000; // 5% APY
        let reserve_factor = 1000; // 10% reserve

        DeFi::create_lending_pool(
            origin.clone(),
            lending_token,
            interest_rate,
            reserve_factor,
        )?;

        // Step 7: Create NFT collection for DApp (NFT)
        let collection_name = format!("{} Collectibles", dapp_name).as_bytes().to_vec();
        let collection_description = format!("NFT collection for {}", dapp_name).as_bytes().to_vec();
        let max_supply = Some(10000);
        let royalty_percentage = 500; // 5%

        NFT::create_nft_collection(
            origin.clone(),
            collection_name,
            collection_description,
            max_supply,
            royalty_percentage,
        )?;

        // Step 8: Register domain for DApp (Web4)
        let domain_name = format!("{}.web4", dapp_name);
        let content_hash = b"QmYwAPJzv5CZsnAztmj5DrE8yQkCjXaieQaxP".to_vec();

        Web4::register_domain(
            origin.clone(),
            domain_name.as_bytes().to_vec(),
            content_hash,
        )?;

        // Step 9: Set up oracle feeds for DApp (Oracle)
        let price_feed_name = format!("{}-USD-Price", token_symbol).as_bytes().to_vec();
        let price_description = format!("{} token USD price feed", token_symbol).as_bytes().to_vec();

        Oracle::create_oracle_feed(
            origin.clone(),
            price_feed_name,
            price_description,
            5, // 5 operators
            3600, // 1 hour staleness
            3, // 3 minimum submissions
            1000000000000000000, // 1 token reward
        )?;

        // Step 10: Set up governance for DApp (Governance)
        let proposal_title = format!("Fund {} Ecosystem Development", dapp_name).as_bytes().to_vec();
        let proposal_description = format!("Allocate treasury funds for {} ecosystem growth", dapp_name).as_bytes().to_vec();

        Governance::create_proposal(
            origin.clone(),
            proposal_title,
            proposal_description,
            None,
        )?;

        // Step 11: Set up cross-chain bridge (Bridge)
        let bridge_id = format!("{}-bridge", dapp_name).as_bytes().to_vec();
        let target_chain = b"ethereum".to_vec();

        Bridge::create_bridge(
            origin.clone(),
            bridge_id,
            target_chain,
            3, // 3 validator threshold
            50, // 0.5% fee
        )?;

        // Step 12: Create asset mapping for cross-chain (Bridge)
        let source_asset = token_symbol_bytes.clone();
        let target_asset = format!("{}-ETH", token_symbol).as_bytes().to_vec();
        let conversion_rate = 1000000000000000000; // 1:1 conversion

        Bridge::create_asset_mapping(
            origin.clone(),
            format!("rechain-{}", dapp_name).as_bytes().to_vec(),
            source_asset,
            target_asset,
            conversion_rate,
            1000000000000000, // 0.001 fee
            1000000000000000000, // 1 token min
            100000000000000000000000, // 100k token max
        )?;

        Ok(())
    }

    /// Example: User interaction with complete ecosystem
    pub fn user_full_ecosystem_interaction(
        origin: RuntimeOrigin,
        user: AccountId,
        dapp_name: &str,
        token_symbol: &str,
    ) -> DispatchResult {
        let who = ensure_signed(origin)?;

        // 1. User creates identity (Web5)
        let user_did = format!("did:rechain:user:{}", user);
        Web5::create_did(
            origin.clone(),
            user_did.as_bytes().to_vec(),
            vec![b"user-key".to_vec()],
            vec![],
        )?;

        // 2. User purchases DApp tokens (DeFi)
        let pool_name = format!("{}-RECH-Pool", token_symbol).as_bytes().to_vec();
        let token_in = b"RECH".to_vec();
        let amount_in = 1000000000000000000000; // 1000 RECH

        DeFi::swap_tokens(
            origin.clone(),
            pool_name,
            token_in,
            amount_in,
            800000000000000000000, // Min 800 tokens out
        )?;

        // 3. User stakes tokens in lending pool (DeFi)
        let token_symbol_bytes = token_symbol.as_bytes().to_vec();
        let stake_amount = 500000000000000000000; // 500 tokens

        DeFi::deposit_to_lending_pool(
            origin.clone(),
            token_symbol_bytes.clone(),
            stake_amount,
        )?;

        // 4. User borrows against stake (DeFi)
        let borrow_amount = 250000000000000000000; // 250 tokens

        DeFi::borrow_from_lending_pool(
            origin.clone(),
            token_symbol_bytes.clone(),
            borrow_amount,
        )?;

        // 5. User purchases NFT (NFT)
        let collection = format!("{} Collectibles", dapp_name).as_bytes().to_vec();
        let token_id = 1;

        NFT::purchase_nft(
            origin.clone(),
            collection,
            token_id,
        )?;

        // 6. User participates in governance (Governance)
        let proposal_id = 1;
        Governance::vote_on_proposal(
            origin.clone(),
            proposal_id,
            VoteDirection::Aye,
        )?;

        // 7. User bridges tokens cross-chain (Bridge)
        let bridge_id = format!("{}-bridge", dapp_name).as_bytes().to_vec();
        let source_asset = token_symbol_bytes;
        let target_chain = b"ethereum".to_vec();
        let target_asset = format!("{}-ETH", token_symbol).as_bytes().to_vec();
        let recipient = b"0x742d35Cc6634C0532925a3b8D0007f0d6b5c2".to_vec();
        let transfer_amount = 100000000000000000000; // 100 tokens

        Bridge::initiate_cross_chain_transfer(
            origin.clone(),
            bridge_id,
            source_asset,
            target_chain,
            target_asset,
            recipient,
            transfer_amount,
        )?;

        Ok(())
    }

    /// Example: Cross-pallet DeFi with privacy
    pub fn private_defi_transaction(
        origin: RuntimeOrigin,
        user: AccountId,
        amount: u128,
        recipient: AccountId,
    ) -> DispatchResult {
        let who = ensure_signed(origin.clone())?;

        // 1. Generate ZKP for private transaction (ZKP)
        let proof_id = b"private-defi-transfer".to_vec();
        let proof_data = generate_private_transfer_proof(who.clone(), recipient, amount)?;
        let public_inputs = generate_public_inputs_for_transfer(amount)?;

        ZKP::submit_zkp(
            origin.clone(),
            proof_id,
            proof_data,
            public_inputs,
        )?;

        // 2. Verify ZKP result
        let verification_status = ZKP::get_proof_status(b"private-defi-transfer");
        ensure!(verification_status == Some(true), "Private transaction verification failed");

        // 3. Execute private transaction through DeFi
        if verification_status == Some(true) {
            // Process transaction privately using oracle for pricing
            let price_feed = b"RECH-USD-Price".to_vec();
            let price_key = b"latest_price".to_vec();

            if Oracle::is_data_fresh(&price_feed, &price_key) {
                let current_price = Oracle::get_median_value(&price_feed, &price_key)
                    .unwrap_or(1000000000000000000); // Default to $1

                // Calculate USD value
                let usd_value = (amount * current_price) / 1000000000000000000;

                // Execute private DeFi transaction
                DeFi::execute_private_swap(origin, who, recipient, amount, usd_value)?;
            }
        }

        Ok(())
    }

    /// Example: Governance-controlled oracle updates
    pub fn governance_controlled_oracle_update(
        origin: RuntimeOrigin,
        feed_name: Vec<u8>,
        new_reward_amount: u128,
    ) -> DispatchResult {
        let who = ensure_signed(origin)?;

        // 1. Create governance proposal for oracle update
        let proposal_title = b"Update Oracle Rewards".to_vec();
        let proposal_description = format!("Increase oracle rewards to {} for better data quality", new_reward_amount).as_bytes().to_vec();

        Governance::create_proposal(
            origin.clone(),
            proposal_title,
            proposal_description,
            None,
        )?;

        // 2. Vote on proposal
        let proposal_id = Governance::next_proposal_id() - 1;
        Governance::vote_on_proposal(
            origin.clone(),
            proposal_id,
            VoteDirection::Aye,
        )?;

        // 3. If proposal passes, update oracle feed
        let proposal = Governance::proposals(proposal_id);
        if let Some(prop) = proposal {
            if prop.status == ProposalStatus::Succeeded {
                // Execute oracle update
                let feed_name_bounded = BoundedVec::<u8, MaxFeedNameLength>::try_from(feed_name)
                    .map_err(|_| "Feed name too long")?;

                // This would require updating the oracle feed configuration
                // For now, just demonstrate the integration concept
            }
        }

        Ok(())
    }

    /// Example: NFT-collateralized lending with oracle pricing
    pub fn nft_collateralized_lending(
        origin: RuntimeOrigin,
        borrower: AccountId,
        nft_collection: Vec<u8>,
        nft_id: u64,
        loan_amount: u128,
    ) -> DispatchResult {
        let who = ensure_signed(origin)?;

        // 1. Verify NFT ownership
        let nft_collection_bounded = BoundedVec::<u8, MaxCollectionNameLength>::try_from(nft_collection.clone())
            .map_err(|_| "Collection name too long")?;

        ensure!(NFT::token_ownership(&borrower, &(borrower.clone(), nft_collection_bounded.clone(), nft_id))
            .is_some(), "NFT not owned by borrower");

        // 2. Get NFT floor price from oracle
        let price_feed = b"NFT-FLOOR-PRICE".to_vec();
        let price_key = format!("collection_{}", String::from_utf8_lossy(&nft_collection)).as_bytes().to_vec();

        let floor_price = if Oracle::is_data_fresh(&price_feed, &price_key) {
            Oracle::get_median_value(&price_feed, &price_key).unwrap_or(0)
        } else {
            0
        };

        // 3. Calculate maximum loan based on NFT value (50% LTV)
        let max_loan = (floor_price * loan_amount) / 1000000000000000000 * 50 / 100;
        ensure!(loan_amount <= max_loan, "Loan amount exceeds maximum LTV");

        // 4. Create loan with NFT as collateral
        DeFi::create_nft_collateralized_loan(
            origin.clone(),
            borrower.clone(),
            nft_collection,
            nft_id,
            loan_amount,
        )?;

        // 5. Mint loan tokens to borrower
        DeFi::disburse_loan(
            origin,
            borrower,
            loan_amount,
        )?;

        Ok(())
    }

    /// Example: Cross-chain governance with bridge
    pub fn cross_chain_governance_execution(
        origin: RuntimeOrigin,
        proposal_id: u64,
        target_chain: Vec<u8>,
    ) -> DispatchResult {
        let who = ensure_signed(origin)?;

        // 1. Verify proposal passed on home chain
        let proposal = Governance::proposals(proposal_id);
        ensure!(proposal.is_some(), "Proposal not found");
        let prop = proposal.unwrap();
        ensure!(prop.status == ProposalStatus::Succeeded, "Proposal not approved");

        // 2. Create cross-chain bridge for governance action
        let bridge_id = format!("governance-bridge-{}", proposal_id).as_bytes().to_vec();

        Bridge::create_bridge(
            origin.clone(),
            bridge_id.clone(),
            target_chain.clone(),
            5, // 5 validator threshold
            25, // 0.25% fee
        )?;

        // 3. Execute governance action on target chain
        Bridge::initiate_cross_chain_transfer(
            origin.clone(),
            bridge_id,
            b"GOVERNANCE".to_vec(),
            target_chain,
            b"GOVERNANCE-ACTION".to_vec(),
            format!("proposal-{}", proposal_id).as_bytes().to_vec(),
            1, // 1 governance token
        )?;

        Ok(())
    }

    /// Example: Privacy-preserving voting with ZKP
    pub fn private_governance_voting(
        origin: RuntimeOrigin,
        voter: AccountId,
        proposal_id: u64,
        vote_choice: VoteDirection,
        voter_nullifier: Vec<u8>,
    ) -> DispatchResult {
        let who = ensure_signed(origin)?;

        // 1. Generate ZKP for private vote
        let proof_id = format!("private-vote-{}", proposal_id).as_bytes().to_vec();
        let proof_data = generate_private_vote_proof(vote_choice.clone(), voter_nullifier.clone())?;
        let public_inputs = generate_vote_public_inputs(proposal_id, vote_choice.clone())?;

        ZKP::submit_zkp(
            origin.clone(),
            proof_id,
            proof_data,
            public_inputs,
        )?;

        // 2. Verify ZKP result
        let verification_status = ZKP::get_proof_status(&format!("private-vote-{}", proposal_id).as_bytes().to_vec());
        ensure!(verification_status == Some(true), "Private vote verification failed");

        // 3. Cast vote if verification successful
        if verification_status == Some(true) {
            Governance::vote_on_proposal(
                origin,
                proposal_id,
                vote_choice,
            )?;
        }

        Ok(())
    }

    /// Example: Oracle price feed for DeFi liquidation
    pub fn oracle_powered_defi_liquidation(
        origin: RuntimeOrigin,
        borrower: AccountId,
        lending_pool: Vec<u8>,
    ) -> DispatchResult {
        let who = ensure_signed(origin)?;

        // 1. Get current price from oracle
        let price_feed = b"RECH-USD-Price".to_vec();
        let price_key = b"latest_price".to_vec();

        ensure!(Oracle::is_data_fresh(&price_feed, &price_key), "Price data too stale");

        let current_price = Oracle::get_median_value(&price_feed, &price_key)
            .unwrap_or(1000000000000000000); // Default $1

        // 2. Check if borrower's position should be liquidated
        let lending_pool_bounded = BoundedVec::<u8, MaxTokenSymbolLength>::try_from(lending_pool.clone())
            .map_err(|_| "Lending pool name too long")?;

        let position = DeFi::user_lending_positions(&borrower, &lending_pool_bounded);
        if let Some(pos) = position {
            let collateral_value = pos.deposit_amount * current_price / 1000000000000000000;
            let borrow_value = pos.borrow_amount;

            // Check if position is under-collateralized (below 150% ratio)
            let collateral_ratio = (collateral_value * 100) / borrow_value;

            if collateral_ratio < 150 {
                // Trigger liquidation
                DeFi::liquidate_position(
                    origin,
                    borrower,
                    lending_pool,
                    current_price,
                )?;
            }
        }

        Ok(())
    }
}

// Helper functions for integration examples
impl CompleteEcosystemIntegration {
    fn generate_private_transfer_proof(sender: AccountId, recipient: AccountId, amount: u128) -> Result<Vec<u8>, &'static str> {
        // In a real implementation, this would generate actual ZKP
        Ok(b"mock-zkp-proof-data".to_vec())
    }

    fn generate_public_inputs_for_transfer(amount: u128) -> Result<Vec<u8>, &'static str> {
        Ok(format!("public-inputs-for-amount-{}", amount).as_bytes().to_vec())
    }

    fn generate_private_vote_proof(vote: VoteDirection, nullifier: Vec<u8>) -> Result<Vec<u8>, &'static str> {
        Ok(b"mock-private-vote-proof".to_vec())
    }

    fn generate_vote_public_inputs(proposal_id: u64, vote: VoteDirection) -> Result<Vec<u8>, &'static str> {
        Ok(format!("vote-{}-for-proposal-{}", vote as u8, proposal_id).as_bytes().to_vec())
    }
}

// Integration test example
#[cfg(test)]
mod integration_tests {
    use super::*;
    use frame_support::assert_ok;

    #[test]
    fn test_complete_ecosystem_setup() {
        new_test_ext().execute_with(|| {
            let founder = 1u64;
            let dapp_name = "TestDapp";
            let token_symbol = "TEST";
            let initial_supply = 1000000000000000000000000;

            assert_ok!(CompleteEcosystemIntegration::setup_complete_dapp_ecosystem(
                RuntimeOrigin::signed(founder),
                founder,
                dapp_name,
                token_symbol,
                initial_supply,
            ));
        });
    }

    #[test]
    fn test_user_ecosystem_interaction() {
        new_test_ext().execute_with(|| {
            let user = 2u64;
            let dapp_name = "TestDapp";
            let token_symbol = "TEST";

            assert_ok!(CompleteEcosystemIntegration::user_full_ecosystem_interaction(
                RuntimeOrigin::signed(user),
                user,
                dapp_name,
                token_symbol,
            ));
        });
    }
}