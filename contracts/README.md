# Solidity Contracts

This directory contains the Solidity contracts for an application with [RISC Zero] on Ethereum.
This contract holds a merkle tree root hash, guaranteed to be a valid computation result.

The Solidity libraries for RISC Zero can be found at [github.com/risc0/risc0-ethereum].

Contracts are built and tested with [forge], which is part of the [Foundry] toolkit.

## Generated Contracts

As part of the build process, this template generates the `ImageID.sol` and `Elf.sol` contracts.
Running `cargo build` will generate these contracts with up to date references to your guest code.

- `ImageID.sol`: contains the [Image IDs][image-id] for the guests implemented in the [methods] directory.
- `Elf.sol`: contains the path of the guest binaries implemented in the [methods] directory.
  This contract is saved in the `tests` directory in the root of this template.

[Foundry]: https://getfoundry.sh/
[RISC Zero]: https://risczero.com
[forge]: https://github.com/foundry-rs/foundry#forge
[github.com/risc0/risc0-ethereum]: https://github.com/risc0/risc0-ethereum/tree/main/contracts
[image-id]: https://dev.risczero.com/terminology#image-id
[methods]: ../methods/README.md
