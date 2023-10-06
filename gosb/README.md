# GOSB

Grid Operating Systems Blockchain Node Project.

## Getting Started

Head to [`docs.substrate.io`](https://docs.substrate.io) and follow the [installation](https://docs.substrate.io/install/)
instructions.  Then try out one of the [tutorials](https://docs.substrate.io/tutorials/).  Refer to the [Docker
instructions](./docker/README.md) to quickly run Substrate, Substrate Node Template, Subkey, or to build a chain spec.

## Community & Support

Join the highly active and supportive community on the [Substrate Stack Exchange](https://substrate.stackexchange.com/)
to ask questions about use and problems you run into using this software. Please do report bugs and [issues
here](https://github.com/gridoperatingsystems/gos-sdk/issues) for anything you suspect requires action in the source.

## Contributions & Code of Conduct

Please follow the contributions guidelines as outlined in
[`docs/CONTRIBUTING.md`](https://github.com/gridoperatingsystems/gos-sdk/tree/main/docs/CONTRIBUTING.md). In all
communications and contributions, this project follows the [Contributor Covenant Code of
Conduct](https://github.com/gridoperatingsystems/gos-sdk/tree/main/docs/CODE_OF_CONDUCT.md).

## Security

The security policy and procedures can be found in
[`docs/SECURITY.md`](https://github.com/gridoperatingsystems/gos-sdk/tree/main/docs/SECURITY.md).

## License

- Substrate Primitives (`sp-*`), Frame (`frame-*`) and the pallets (`pallets-*`), binaries (`/bin`) and all other
utilities are licensed under [Apache 2.0](LICENSE-APACHE2).  - Substrate Client (`/client/*` / `sc-*`) is licensed under
[GPL v3.0 with a classpath linking exception](LICENSE-GPL3).

The reason for the split-licensing is to ensure that for the vast majority of teams using Substrate to create
feature-chains, then all changes can be made entirely in Apache2-licensed code, allowing teams full freedom over what
and how they release and giving licensing clarity to commercial teams.

In the interests of the community, we require any deeper improvements made to Substrate's core logic (e.g. Substrate's
internal consensus, crypto or database code) to be contributed back so everyone can benefit.
