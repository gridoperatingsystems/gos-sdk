# GOS SDK

![](https://www.gridoperatingsystems.com/_app/immutable/assets/gos-grid-operating-systems-icon-black.0a9d183a.png)

[![StackExchange](https://img.shields.io/badge/StackExchange-Community%20&%20Support-222222?logo=stackexchange)](https://substrate.stackexchange.com/)


## Introduction

The thesis of [Grid Operating Systems](https://www.gridoperatingsystems.com/) in the [White Paper](https://www.gridoperatingsystems.com/whitepaper) is to introduce a decentralized infrastructure managed by intelligent agents, utilizing a multilayer network of decentralized components that exchange secure data and computation through a signal broadcasting system. The goal is to enhance redundancy and optimize replication services for maximum 24/7 data delivery, while also tokenizing content, storage, and computation to settle service fees using smart contracts. Ultimately, the aim is to cultivate digital economies that combine the benefits of Web2 and Web3.

The GOS SDK repository provides all the resources needed to start building on the Grid Operating Systems network.

## Projects

The Grid Operating Systems (GOS) ecosystem is composed of decentralized components that encompass storage, computation, and blockchain services. It also includes storage and computation agent networks that manage the flow of services and information.

- gcli : GCLI is a command line interface program that allows users to manage and interact with the GOS Blockchain and GOS Network Services. It provides a familiar experience for developers by drawing inspiration from the Linux terminal and its operating system structure. GCLI offers a range of commands for navigation, file and directory operations, utilities and monitoring, help and documentation, keychain management, networking and provider management, and data integrity and blockchain management.

- gosb : Blockchain Node is a hybrid blockchain architecture that combines Proof of Authority (PoA) and Nominated Proof of Stake (nPoS) consensus mechanisms. It serves as the \"source of truth\" for the GOS network and provides flexible and scalable storage and computation solutions.

- gosf : Fusion Node is a specialized server that serves as a consolidated source of truth for each data network within the GOS Ecosystem. It integrates with Veilid to establish a \"Veilid network\" similar to IPFS and Tor but with superior speed and design. The Fusion Node ensures data integrity by documenting all storage, computation transactions, and node operations on its internal blockchain. It acts as a controller between the GOS CLI, Web Console or API access, and the GOS static or dynamic infrastructure.

- gosa : Agent Node is a node on the Relay Network that distributes and rebalances the infrastructure of Compute and Storage Drives. It learns from logs and event streams and provides recommendations on operations and orchestration within a Proof-Of-Stake consensus network.

## Upstream Dependencies

Below are the primary upstream dependencies utilized in this project:

- [`parity-scale-codec`](https://crates.io/crates/parity-scale-codec)
- [`parity-db`](https://crates.io/crates/parity-db)
- [`parity-common`](https://github.com/paritytech/parity-common)
- [`trie`](https://github.com/paritytech/trie)

## Security

The security policy and procedures can be found in [docs/SECURITY.md](./docs/SECURITY.md).

## Contributing & Code of Conduct

Ensure you follow our [contribution guidelines](./docs/CONTRIBUTING.md). In every interaction and contribution, this
project adheres to the [Contributor Covenant Code of Conduct](./docs/CODE_OF_CONDUCT.md).

