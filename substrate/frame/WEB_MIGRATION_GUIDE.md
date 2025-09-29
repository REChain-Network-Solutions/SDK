# Web3 → Web4 → Web5 Migration Guide

A comprehensive guide for migrating between different web versions in the REChain ecosystem, developed by REChain Network Solutions LLC.

## Overview

This guide provides step-by-step instructions for migrating applications and data between:
- **Web3**: Traditional DApp and smart contract functionality
- **Web4**: Domain registration and content hosting
- **Web5**: Decentralized identity and verifiable credentials

## Migration Paths

### Web3 to Web4 Migration

**When to migrate:**
- Adding domain registration to your DApp
- Implementing content hosting capabilities
- Enhancing user experience with readable URLs

**Migration Steps:**

1. **Update Dependencies**
```toml
[dependencies]
pallet-web4 = { version = "4.0.0-dev", default-features = false, path = "../../frame/web4" }
```

2. **Configure Web4 in Runtime**
```rust
impl pallet_web4::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxDomainLength = ConstU32<255>;
    type MaxContentLength = ConstU32<1024>;
}
```

3. **Register DApp Domain**
```rust
// After registering DApp in Web3
let domain_name = format!("{}.web4", dapp_name);
let content_hash = generate_content_hash(dapp_content);

Web4::register_domain(origin, domain_name, content_hash)?;
```

4. **Update Frontend Integration**
```javascript
// Old: Direct IPFS hash
const dappUrl = "ipfs://QmYwAPJzv5CZsnAztmj5DrE8yQkCjXaieQaxZ";

// New: Web4 domain
const dappUrl = "my-dapp.web4";
```

### Web3 to Web5 Migration

**When to migrate:**
- Adding user identity management
- Implementing verifiable credentials
- Supporting self-sovereign identity features

**Migration Steps:**

1. **Update Dependencies**
```toml
[dependencies]
pallet-web5 = { version = "5.0.0-dev", default-features = false, path = "../../frame/web5" }
```

2. **Configure Web5 in Runtime**
```rust
impl pallet_web5::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxDidLength = ConstU32<128>;
    type MaxIdentityMetadataLength = ConstU32<1024>;
    type MaxCredentialsPerIdentity = ConstU32<100>;
}
```

3. **Create DID for Users**
```rust
// Create user identity
let user_did = format!("did:rechain:user:{}", user_id);
let public_keys = vec![user_public_key];

Web5::create_did(origin, user_did.clone(), public_keys, vec![])?;
```

4. **Issue DApp Credentials**
```rust
// Issue access credentials
let credential_id = format!("dapp-access-{}", dapp_name);
let credential_type = b"DAppAccessCredential";

Web5::issue_credential(
    origin,
    user_did,
    credential_id,
    credential_type,
    access_data,
    None
)?;
```

### Complete Web3 → Web4 → Web5 Migration

**Recommended Migration Path:**

1. **Phase 1: Web3 Setup** (Already implemented)
   - Deploy smart contracts
   - Set up DApp registry
   - Implement basic functionality

2. **Phase 2: Web4 Integration**
   - Add domain registration
   - Set up content hosting
   - Update user interfaces with readable URLs

3. **Phase 3: Web5 Enhancement**
   - Implement user identity
   - Add credential system
   - Enable advanced features

**Example Complete Migration:**

```rust
// 1. Set up Web3 DApp
Web3::register_dapp(origin, dapp_name, description)?;
Web3::deploy_contract(origin, dapp_name, contract_metadata, bytecode, abi, gas_opt)?;

// 2. Add Web4 domain
let domain = format!("{}.web4", dapp_name);
Web4::register_domain(origin, domain.clone(), content_hash)?;

// 3. Add Web5 identity
let dapp_did = format!("did:rechain:dapp:{}", dapp_name);
Web5::create_did(origin, dapp_did.clone(), public_keys, services)?;

// 4. Issue ownership credential
Web5::issue_credential(origin, dapp_did, ownership_credential_id, credential_type, data, None)?;
```

## Cross-Version Compatibility

### Shared Runtime Configuration

```rust
// Configure all web pallets in the same runtime
impl pallet_web3::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxDappNameLength = ConstU32<100>;
    type MaxContractMetadataLength = ConstU32<1024>;
    type MaxContractsPerDapp = ConstU32<50>;
}

impl pallet_web4::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxDomainLength = ConstU32<255>;
    type MaxContentLength = ConstU32<2048>;
}

impl pallet_web5::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxDidLength = ConstU32<128>;
    type MaxIdentityMetadataLength = ConstU32<1024>;
    type MaxCredentialsPerIdentity = ConstU32<100>;
}
```

### Runtime Pallet Declaration

```rust
construct_runtime!(
    pub struct Runtime {
        // System pallets
        System: frame_system,
        Balances: pallet_balances,

        // Web ecosystem pallets
        Web3: pallet_web3,
        Web4: pallet_web4,
        Web5: pallet_web5,

        // Other pallets
        Assets: pallet_assets,
        // ...
    }
);
```

## Migration Tools

### Automated Migration Script

```rust
/// Helper function to migrate DApp from Web3-only to full web ecosystem
pub fn migrate_dapp_to_full_web_ecosystem(
    origin: RuntimeOrigin,
    dapp_name: Vec<u8>,
    owner: AccountId,
) -> DispatchResult {
    // 1. Verify DApp exists in Web3
    ensure!(Web3::dapps(dapp_name.clone()).is_some(), "DApp not found in Web3");

    // 2. Create DID for DApp
    let dapp_did = format!("did:rechain:dapp:{}", String::from_utf8_lossy(&dapp_name));
    // ... Web5 DID creation logic

    // 3. Register Web4 domain
    let domain = format!("{}.web4", String::from_utf8_lossy(&dapp_name));
    // ... Web4 domain registration logic

    // 4. Issue ownership credentials
    // ... Web5 credential issuance logic

    Ok(())
}
```

### Data Migration Considerations

1. **Contract Addresses**: Remain unchanged in Web3 pallet
2. **User Identities**: Create new DIDs in Web5 pallet
3. **Content Hosting**: Migrate IPFS hashes to Web4 domains
4. **Access Control**: Replace address-based auth with credential-based auth

## Testing Migration

### Unit Tests

```rust
#[test]
fn test_web3_to_web4_migration() {
    new_test_ext().execute_with(|| {
        // Setup Web3 DApp
        assert_ok!(Web3::register_dapp(origin, dapp_name, description));

        // Migrate to Web4
        assert_ok!(migrate_dapp_to_web4(origin, dapp_name, domain_name));

        // Verify Web4 domain exists
        assert!(Web4::domains(domain_name).is_some());
    });
}
```

### Integration Tests

```rust
#[test]
fn test_full_web_ecosystem_integration() {
    new_test_ext().execute_with(|| {
        // Test complete workflow
        assert_ok!(setup_complete_web_ecosystem(
            origin,
            developer,
            "test-dapp",
            "test-dapp.web4",
            "did:rechain:developer:123"
        ));

        // Verify all components exist
        assert!(Web3::dapps(b"test-dapp").is_some());
        assert!(Web4::domains(b"test-dapp.web4").is_some());
        assert!(Web5::decentralized_ids(b"did:rechain:developer:123").is_some());
    });
}
```

## Best Practices

### 1. Gradual Migration
- Migrate one component at a time
- Test each migration step thoroughly
- Keep backward compatibility during transition

### 2. Data Integrity
- Validate all data before migration
- Maintain data consistency across pallets
- Implement rollback mechanisms

### 3. User Experience
- Communicate changes to users
- Provide migration tools for user data
- Maintain service availability during migration

### 4. Security Considerations
- Verify all authorization checks
- Maintain credential validity
- Audit migration logic thoroughly

## Troubleshooting

### Common Issues

1. **DID Already Exists**
   - Check if identity was already created
   - Verify DID format compliance

2. **Domain Registration Failed**
   - Ensure domain name format is valid
   - Check content hash format

3. **Credential Verification Failed**
   - Verify issuer authorization
   - Check credential expiration
   - Confirm subject DID exists

### Debugging Tools

```rust
/// Debug function to inspect web ecosystem state
pub fn debug_web_ecosystem_state(dapp_name: Vec<u8>) {
    // Check Web3 state
    if let Some(dapp) = Web3::dapps(dapp_name.clone()) {
        println!("Web3 DApp: Owner={:?}, Contracts={}", dapp.owner, dapp.contract_count);
    }

    // Check Web4 state
    let domain_name = format!("{}.web4", String::from_utf8_lossy(&dapp_name));
    if let Some(domain) = Web4::domains(domain_name.as_bytes()) {
        println!("Web4 Domain: Owner={:?}", domain.owner);
    }

    // Check Web5 state
    let did = format!("did:rechain:dapp:{}", String::from_utf8_lossy(&dapp_name));
    if let Some(identity) = Web5::decentralized_ids(did.as_bytes()) {
        println!("Web5 DID: Controller={:?}, Credentials={}",
                 identity.controller, identity.credential_count);
    }
}
```

## Support and Resources

- **Documentation**: https://docs.rechain.network
- **GitHub Issues**: https://github.com/REChain-Network-Solutions/SDK/issues
- **Discord**: [REChain Community](https://discord.gg/rechain)
- **Email**: info@rechain.network

For migration assistance, please create an issue on GitHub with the label `migration-help`.