<div align="center">

![SDK Logo](./docs/images/Rechain_Logo_Horizontal_Pink_White.png#gh-dark-mode-only)
![SDK Logo](./docs/images/Rechain_Logo_Horizontal_Pink_Black.png#gh-light-mode-only)

# Rechain SDK

![GitHub stars](https://img.shields.io/github/stars/REChain-Network-Solutions/SDK)&nbsp;&nbsp;![GitHub
forks](https://img.shields.io/github/forks/REChain-Network-Solutions/SDK)

<!-- markdownlint-disable-next-line MD013 -->
[![StackExchange](https://img.shields.io/badge/StackExchange-Community%20&%20Support-222222?logo=stackexchange)](https://substrate.stackexchange.com/)&nbsp;&nbsp;![GitHub contributors](https://img.shields.io/github/contributors/paritytech/rechain-sdk)&nbsp;&nbsp;![GitHub commit activity](https://img.shields.io/github/commit-activity/m/paritytech/rechain-sdk)&nbsp;&nbsp;![GitHub last commit](https://img.shields.io/github/last-commit/paritytech/rechain-sdk)

> The Rechain SDK repository provides all the components needed to start building on the
> [Rechain](https://rechain.network) network, a multi-chain blockchain platform that enables
> different blockchains to interoperate and share information in a secure and scalable way.

</div>

## ⚡ Quickstart
If you want to get an example node running quickly you can execute the following getting started script:

```
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/REChain-Network-Solutions/SDK/master/scripts/getting-started.sh | bash
```

## 📚 Documentation

* [Rechain Documentation Portal](https://docs.rechain.network)
* [🦀 rust-docs](https://REChain-Network-Solutions.github.io/SDK/master/rechain_sdk_docs/index.html): Where we keep track of
the API docs of our Rust crates. Includes:
  * [Introduction](https://REChain-Network-Solutions.github.io/SDK/master/rechain_sdk_docs/rechain_sdk/index.html)
to each component of the Rechain SDK: Substrate, FRAME, Cumulus, and XCM
  * [Guides](https://REChain-Network-Solutions.github.io/SDK/master/rechain_sdk_docs/guides/index.html),
namely how to build your first FRAME pallet
  * [Templates](https://REChain-Network-Solutions.github.io/SDK/master/rechain_sdk_docs/rechain_sdk/templates/index.html)
    for starting a new project.
  * [External Resources](https://REChain-Network-Solutions.github.io/SDK/master/rechain_sdk_docs/external_resources/index.html)

## 🚀 Releases

<!-- markdownlint-disable-next-line MD013 -->
![Current Stable Release](https://raw.githubusercontent.com/paritytech/release-registry/main/badges/rechain-sdk-latest.svg)&nbsp;&nbsp;![Next Stable Release](https://raw.githubusercontent.com/paritytech/release-registry/main/badges/rechain-sdk-next.svg)

The Rechain SDK is released every three months as a `stableYYMM` release. They are supported for
one year with patches. See the next upcoming versions in the [Release
Registry](https://github.com/paritytech/release-registry/) and more docs in [RELEASE.md](./docs/RELEASE.md).

You can use [`psvm`](https://github.com/paritytech/psvm) to update all dependencies to a specific
version without needing to manually select the correct version for each crate.

## 🛠️ Tooling

[Rechain SDK Version Manager](https://github.com/REChain-Network-Solutions/REChain-SDK-Manager):
A simple tool to manage and update the Rechain SDK dependencies in any Cargo.toml file.
It will automatically update the Rechain SDK dependencies to their correct crates.io version.

## 🔐 Security

The security policy and procedures can be found in
[docs/contributor/SECURITY.md](./docs/contributor/SECURITY.md).

## 🤍 Contributing & Code of Conduct

Ensure you follow our [contribution guidelines](./docs/contributor/CONTRIBUTING.md). In every
interaction and contribution, this project adheres to the [Contributor Covenant Code of
Conduct](./docs/contributor/CODE_OF_CONDUCT.md).

### 👾 Ready to Contribute?

Take a look at the issues labeled with [`mentor`](https://github.com/REChain-Network-Solutions/SDK/labels/C1-mentor)
(or alternatively [this](https://mentor.tasty.limo/) page, created by one of the maintainers) label to get started!
We always recognize valuable contributions by proposing an on-chain tip to the Rechain network as a token of our
appreciation.

## Polkadot Fellowship

Development in this repo is led by REChain Network Solutions LLC. In short,
this repository provides all the SDK pieces needed to build both Rechain and its parachains.
The Rechain runtime is developed and maintained by REChain Network Solutions LLC.

## History

This repository represents the Rechain SDK developed by REChain Network Solutions LLC,
built upon the Substrate framework and related technologies to provide a complete blockchain
development platform.
