//! Comprehensive integration example for Web3 + Web4 + Web5
//!
//! This file demonstrates how the three web pallets work together to create
//! a complete decentralized web ecosystem.

use frame_support::{decl_module, decl_storage, decl_event, dispatch::DispatchResult};
use frame_system::ensure_signed;

/// Combined Web Ecosystem Configuration
pub struct WebEcosystemConfig;

impl pallet_web3::Config for WebEcosystemConfig {
    type RuntimeEvent = RuntimeEvent;
    type MaxDappNameLength = ConstU32<100>;
    type MaxContractMetadataLength = ConstU32<1024>;
    type MaxContractsPerDapp = ConstU32<50>;
}

impl pallet_web4::Config for WebEcosystemConfig {
    type RuntimeEvent = RuntimeEvent;
    type MaxDomainLength = ConstU32<255>;
    type MaxContentLength = ConstU32<2048>;
}

impl pallet_web5::Config for WebEcosystemConfig {
    type RuntimeEvent = RuntimeEvent;
    type MaxDidLength = ConstU32<128>;
    type MaxIdentityMetadataLength = ConstU32<1024>;
    type MaxCredentialsPerIdentity = ConstU32<100>;
}

/// Example of a complete decentralized web application setup
pub fn setup_complete_web_ecosystem(
    origin: RuntimeOrigin,
    developer: AccountId,
    dapp_name: &str,
    domain_name: &str,
    did: &str,
) -> DispatchResult {
    let who = ensure_signed(origin)?;

    // Step 1: Create decentralized identity (Web5)
    let did_bytes = did.as_bytes().to_vec();
    let public_keys = vec![
        b"ed25519-key-1".to_vec(),
        b"x25519-key-2".to_vec(),
    ];
    let services = vec![
        (
            b"web3-dapp-service".to_vec(),
            b"Web3DApp".to_vec(),
            format!("https://{}.rechain.network", dapp_name).as_bytes().to_vec(),
        ),
    ];

    Web5::create_did(
        RuntimeOrigin::signed(who.clone()),
        did_bytes.clone(),
        public_keys,
        services,
    )?;

    // Step 2: Register DApp (Web3)
    let dapp_name_bytes = dapp_name.as_bytes().to_vec();
    let dapp_description = format!("Web3 DApp by {}", developer).as_bytes().to_vec();

    Web3::register_dapp(
        RuntimeOrigin::signed(who.clone()),
        dapp_name_bytes.clone(),
        dapp_description,
    )?;

    // Step 3: Deploy smart contract for DApp (Web3)
    let contract_metadata = b"main-contract".to_vec();
    let bytecode_hash = b"QmYwAPJzv5CZsnAztmj5DrE8yQkCjXaieQaxZ".to_vec();
    let abi_hash = b"QmYwAPJzv5CZsnAztmj5DrE8yQkCjXaieQaxY".to_vec();

    Web3::deploy_contract(
        RuntimeOrigin::signed(who.clone()),
        dapp_name_bytes.clone(),
        contract_metadata,
        bytecode_hash,
        abi_hash,
        850, // 85% gas optimization
    )?;

    // Step 4: Register domain for DApp (Web4)
    let domain_name_bytes = domain_name.as_bytes().to_vec();
    let content_hash = b"QmYwAPJzv5CZsnAztmj5DrE8yQkCjXaieQaxW".to_vec();

    Web4::register_domain(
        RuntimeOrigin::signed(who.clone()),
        domain_name_bytes.clone(),
        content_hash,
    )?;

    // Step 5: Issue developer credential (Web5)
    let credential_id = format!("developer-credential-{}", dapp_name).as_bytes().to_vec();
    let credential_type = b"DeveloperCredential".to_vec();
    let credential_data = format!("Certified developer for {}", dapp_name).as_bytes().to_vec();

    Web5::issue_credential(
        RuntimeOrigin::signed(who.clone()),
        did_bytes,
        credential_id,
        credential_type,
        credential_data,
        None, // No expiration
    )?;

    Ok(())
}

/// Example: User interaction with the web ecosystem
pub fn user_access_dapp(
    origin: RuntimeOrigin,
    user: AccountId,
    dapp_name: &str,
    user_did: &str,
) -> DispatchResult {
    let who = ensure_signed(origin)?;

    // Verify user's identity exists
    let user_did_bytes = user_did.as_bytes().to_vec();
    ensure!(Web5::decentralized_ids(user_did_bytes.clone()).is_some(), "User DID not found");

    // Check if user has access credentials
    let access_credential = format!("access-{}", dapp_name).as_bytes().to_vec();
    let credential_info = Web5::verifiable_credentials(user_did_bytes.clone(), access_credential.clone());

    match credential_info {
        Some(cred) => {
            ensure!(!cred.revoked, "Credential revoked");
            // User has valid credentials, grant access
            Ok(())
        },
        None => {
            // Request access or redirect to registration
            Err("Access credentials required".into())
        }
    }
}

/// Migration helper: Upgrade from Web3 to Web4/Web5
pub fn migrate_web3_to_web5(
    origin: RuntimeOrigin,
    dapp_owner: AccountId,
    dapp_name: &str,
) -> DispatchResult {
    let who = ensure_signed(origin)?;

    // 1. Get existing DApp info from Web3
    let dapp_name_bytes = dapp_name.as_bytes().to_vec();
    let dapp_info = Web3::dapps(dapp_name_bytes.clone())
        .ok_or("DApp not found in Web3")?;

    // 2. Create DID for the DApp (Web5)
    let did = format!("did:rechain:dapp:{}", dapp_name).as_bytes().to_vec();
    let public_keys = vec![
        format!("dapp-key-{}", dapp_name).as_bytes().to_vec(),
    ];
    let services = vec![
        (
            format!("{}-service", dapp_name).as_bytes().to_vec(),
            b"DAppService".to_vec(),
            format!("https://{}.rechain.network", dapp_name).as_bytes().to_vec(),
        ),
    ];

    Web5::create_did(
        RuntimeOrigin::signed(dapp_owner.clone()),
        did.clone(),
        public_keys,
        services,
    )?;

    // 3. Register domain for DApp (Web4)
    let domain_name = format!("{}.web4", dapp_name);
    let content_hash = b"QmYwAPJzv5CZsnAztmj5DrE8yQkCjXaieQaxP".to_vec();

    Web4::register_domain(
        RuntimeOrigin::signed(dapp_owner.clone()),
        domain_name.as_bytes().to_vec(),
        content_hash,
    )?;

    // 4. Issue ownership credential (Web5)
    let credential_id = format!("dapp-ownership-{}", dapp_name).as_bytes().to_vec();
    let credential_type = b"DAppOwnership".to_vec();
    let ownership_data = format!("Owner of {} DApp", dapp_name).as_bytes().to_vec();

    Web5::issue_credential(
        RuntimeOrigin::signed(dapp_owner),
        did,
        credential_id,
        credential_type,
        ownership_data,
        None,
    )?;

    Ok(())
}