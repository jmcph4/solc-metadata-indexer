# solc-metadata-indexer #

An [ExEx](https://www.paradigm.xyz/2024/05/reth-exex) that indexes smart contract [metadata](https://docs.soliditylang.org/en/latest/metadata.html) produced by the Solidity compiler, `solc`.

## Background ##

By default, the [Solidity reference implementation](https://github.com/ethereum/solidity) (affectionately known as `solc`) [appends certain metadata](https://docs.soliditylang.org/en/latest/metadata.html) about compilation to its output bytecode. Specifically, this metadata is a CBOR-encoded dictionary which contains various fields -- although this is almost always just a reference to some out-of-band solution for a much larger metadata file. Interestingly, this "reference" is, [since 0.6.0](https://github.com/ethereum/solidity/blob/develop/Changelog.md#060-2019-12-17), a [CIDv0](https://github.com/multiformats/cid/blob/master/README.md#cidv0) (or an "IPFS hash", colloquially). Even *more* interestingly, this used to be a hash that uniquely identified an object on the [Swarm network](https://www.ethswarm.org/swarm-whitepaper.pdf) - a relatively esoteric (and long since forgotten) distributed filesystem. For the keenly interested reader, there's a [fairly useful hosted tool](https://playground.sourcify.dev) by [Sourcify](https://sourcify.dev).

## Usage ##

