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
* Verify proofs for selected values (also exclusion proofs)
* Takes up less space than N proofs (because top-level nodes are not duplicated)
* Becomes even more efficient when keys are close to each other

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


## Internal structure

### Node
`MerkleTrie` is a handle to a root `Node`. `Node` type is defined in [node.rs](./src/node.rs).
```rs
enum Node {
    Null,
    Leaf(Nibbles, Box<[u8]>),
    Extension(Nibbles, Box<Node>),
    Branch([Option<Box<Node>>; 16]),
    Digest(B256),
}
```

`Node::Digest` is a special type of node that does not contain a sub-trie. It is a cut off point that contains a hash of the sub-trie. Therefore we can't get proofs from keys in that segment, but can still compute the root hash.

`Node` methods:
* `get(&self, key_nibs: &[u8]) -> Option<&[u8]>`
    * Has similar semantics as `MerkleTrie.get` as one calls another
* `size(&self) -> usize`
    * Returns the number of full nodes in the trie
    * Full nodes are nodes that need to be hashed to compute the root hash
    * This is useful for performance analysis
* RLP encoding/decoding
    * `rlp_encoded(&self) -> Vec<u8>`
    * `impl rlp::Decodable for Node`

### NodeRef

`Node` is a recursive data structure that contains other nodes. RLP encoding of the node is not recursive and either contains other nodes inline (for nodes shorter than 32 bytes) or contains their hashes.

`NodeRef` is a helper struct that represents the way in which a node is referenced from within another node.
```rs
pub(crate) enum NodeRef {
    Empty,
    Digest(B256),
    Node(Vec<u8>), // This is encoded differently based on the length of the encoded node
}
```

NodeRef is RLP encodable ()`impl Encodable for NodeRef`).

### Path

The least trivial part of MPT is how it encodes path nibbles in `Leaf` and `Extension` nodes. There are two problems that this encoding solves.
* Both `Leaf` and `Extension` are two elements RLP arrays. We need a way to get back type info
* Odd and even number of nibbles look the same when compact-encoded as bytes

MPT solves both with prefixes. [Detailed explanation](https://ethereum.org/pl/developers/docs/data-structures-and-encoding/patricia-merkle-trie/#specification)

We use `Path` struct to perform this parsing and represent results.

```rs
pub enum PathKind {
    Leaf,
    Extension,
}

pub struct Path {
    pub nibbles: Nibbles,
    pub kind: PathKind,
}

impl From<impl AsRef<[u8]>> for Path
```
## Tests

There are unit tests for specific functions of specific structs and two [e2e tests](./tests/).
* Test that our implementation produces the same roots as `alloy_trie`
* Test that we are able to parse results of `eth_getProof`

## Acknowledgements

Inspired by [risc0 steel](https://github.com/risc0/risc0-ethereum/blob/main/steel/src/mpt.rs)