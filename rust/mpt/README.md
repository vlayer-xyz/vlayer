# Merkle Patricia Trie

## Abstract

This is the implementation of **sparse** Merkle Patricia Trie (MPT).

We use it in two places:
* In **EVM execution** - we construct the state and storage tries to provide guest with verifiable source of state and storage data
* In **Block Proofs** - we store mapping from block number to block hash in a trie. This allows us to generate merkle proofs that blocks belong to the same chain

You can read about this data structure [here](https://docs.alchemy.com/docs/patricia-merkle-tries) (more general explanation) and [here](https://ethereum.org/pl/developers/docs/data-structures-and-encoding/patricia-merkle-trie/) (technical details and examples).

There are plenty of libraries that implement MPT, but their interface usually allows you to construct the trie and get single proofs as arrays of nodes. For our use-case we need to efficiently store and verify multiple proofs. Multiple paths merged together form a sparse trie. Sparse trie is a trie which has some nodes replaced with their hashes.

Functionality of sparse MPT:
* Compute the root hash
* Verify inclusion proofs for selected values (and handling exclusion proofs as well)

The advantages of using sparse MPT are:
* Top level nodes are not duplicated. Therefore the more proofs you store, the less memory is needed for proof
* Key prefix proximity escalates the abovementioned effect

## Usage

The main struct is `MerkleTrie(Node)` in [trie.rs](./src/trie.rs) which contains following methods:

`MerkleTrie`
* `from_rlp_nodes(nodes: ...) -> Result<Self, ParseNodeError>`
    * Constructs MPT from nodes. Nodes usually come as a result of concatenation of multiple `eth_getProof`'s. Consult function docs for more details
* `hash_slow(&self) -> B256`
    * Returns the hash of the trie's root node
* `get(&self, key: impl AsRef<[u8]>) -> Option<&[u8]>`
    * Returns a reference to the byte value corresponding to the key
    * For successful exclusion proof - None is returned
    * Panics when neither inclusion nor exclusion of the key can be guaranteed
    * If the value is RLP decodable - you can use `get_rlp`
* `insert(&mut self, key: impl AsRef<[u8]>, value: impl AsRef<[u8]>)`
    * Updates the trie, adding a new node with the given (key, value) pair
    * If the key already exists in the trie, it returns DuplicateKey error

## Internal structure
More details regarding the Merkle Patricia Trie implementation can be found in [Node Internal Structure](./docs/node_internal_structure.md) documentation.

## Acknowledgements

Inspired by [risc0 steel](https://github.com/risc0/risc0-ethereum/blob/main/steel/src/mpt.rs)