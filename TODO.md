# Rebranding Polkadot to Rechain SDK

## Overview
This task involves rebranding the Polkadot SDK to Rechain SDK, including renaming directories, updating crate names, documentation, and code references.

## Steps

### 1. Rename Directory
- [x] Rename `polkadot/` directory to `rechain/`

### 2. Update Root Cargo.toml
- [ ] Update all workspace members starting with `polkadot/` to `rechain/`
- [ ] Update all dependency paths from `polkadot/` to `rechain/`
- [ ] Update default-members from `polkadot` to `rechain`

### 3. Update Crate Names in Rechain Directory
- [ ] Update `rechain/Cargo.toml` name from `polkadot` to `rechain`
- [ ] Update all sub-crate names in `rechain/*/Cargo.toml` from `polkadot-*` to `rechain-*`

### 4. Update Dependencies in Cargo.toml Files
- [ ] Update all references to `polkadot-*` crates to `rechain-*` in all Cargo.toml files across the repo

### 5. Update Documentation
- [ ] Update `README.md`: Change "Polkadot SDK" to "Rechain SDK", "Polkadot" to "Rechain" in descriptions
- [ ] Update docs/ files: Replace "Polkadot" with "Rechain"
- [ ] Update repository URLs from polkadot-sdk to rechain-sdk

### 6. Update Code References
- [ ] Replace "polkadot" with "rechain" in code strings, comments, and variable names where appropriate
- [ ] Update binary names, CLI commands, etc.

### 7. Update Images and Logos
- [ ] Rename logo files from Polkadot_Logo to Rechain_Logo
- [ ] Update image paths in README and docs

### 8. Update CI and Scripts
- [ ] Update .gitlab-ci.yml and scripts/ for new names
- [ ] Update getting-started.sh script

### 9. Test Build
- [ ] Run `cargo check` to ensure no compilation errors
- [ ] Test basic functionality

### 10. Final Checks
- [ ] Verify all references are updated
- [ ] Update any remaining hardcoded strings
