# solc-metadata-indexer #

An [ExEx](https://www.paradigm.xyz/2024/05/reth-exex) that indexes smart contract [metadata](https://docs.soliditylang.org/en/latest/metadata.html) produced by the Solidity compiler, `solc`.

## Background ##

By default, the [Solidity reference implementation](https://github.com/ethereum/solidity) (affectionately known as `solc`) [appends certain metadata](https://docs.soliditylang.org/en/latest/metadata.html) about compilation to its output bytecode. Specifically, this metadata is a CBOR-encoded dictionary which contains various fields -- although this is almost always just a reference to some out-of-band solution for a much larger metadata file. Interestingly, this "reference" is, [since 0.6.0](https://github.com/ethereum/solidity/blob/develop/Changelog.md#060-2019-12-17), a [CIDv0](https://github.com/multiformats/cid/blob/master/README.md#cidv0) (or an "IPFS hash", colloquially). Even *more* interestingly, this used to be a hash that uniquely identified an object on the [Swarm network](https://www.ethswarm.org/swarm-whitepaper.pdf) - a relatively esoteric (and long since forgotten) distributed filesystem. For the keenly interested reader, there's a [fairly useful hosted tool](https://playground.sourcify.dev) by [Sourcify](https://sourcify.dev).

The actual encoding itself is somewhat ad hoc. The length of the CBOR bytes is encoded in the *final two octets* of the entire bytecode sequence.

```
+-----------~~~-----------+-----------------------+---------------------------------------+
|                         |                       |                                       |
| rest of the bytecode... | CBOR-encoded metadata | Length of CBOR-encoded data (2 bytes) |∅
|                         |                       |                                       |
+-----------~~~-----------+-----------------------+---------------------------------------+
```

So to retrieve the CBOR data we need to walk backwards from the end of the entire bytecode sequence by $n-2$ bytes and then read the following $n$ bytes.

## Usage ##

```
Extracts Solidity metadata from contract bytecode

Usage: solc-metadata-indexer [OPTIONS]

Options:
  -l, --live                 Activates and installs the ExEx into a running Reth instance
  -r, --raw                  Interpret input from standard input as literal bytes
  -m, --metadata             Print metadata representation to standard output
  -b, --bytecode <BYTECODE>  Provide file path to a file containing bytecode (interpretation depends on `--raw`)
  -h, --help                 Print help
  -V, --version              Print version
```

