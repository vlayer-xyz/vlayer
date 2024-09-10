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
* Verify inclusion proofs for selected values (also exclusion proofs)

Advantages of using sparse MPT:
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
* `insert(self, key: impl AsRef<[u8]>, value: impl AsRef<[u8]>,)`
    * Has similar semantics as `MerkleTrie.insert` as one calls another
    * It doesn't modify existing node, but returns a new, updated one
* `size(&self) -> usize`
    * Returns the number of full nodes in the trie
    * Full nodes are nodes that need to be hashed to compute the root hash
    * This is useful for performance analysis
* RLP encoding/decoding
    * `rlp_encoded(&self) -> Vec<u8>`
    * `impl rlp::Decodable for Node`

### Insert
Depending on the type of the node, insert method works differently.

The (key, value) pair is often represented in the Node::insert function using the Entry structure to facilitate efficient insertion into a trie. The Entry struct provides a convenient way to encapsulate both the key and value, allowing the Node::insert function to handle them as a single unit. The From<(K, V)> for Entry implementation allows for easy creation of an Entry from various types, enabling the Node::insert function to accept a wide range of input formats. Entry also enables creating Node directly from it, transforming self into Branch with value and empty children (in the case of empty keys) or Leaf (if key is not empty).

Insert function works differently depending on the type of node we are inserting to:

1. Node::Null
This happens only during the first insert into the trie. Node::Null is not created later during insertion in any way.

When we insert into Node::Null, we replace null node by Branch with value if inserted value key is empty or Leaf otherwise.

2. Node::Digest
Insert into Digest node shouldn't even happen so we just panic if we trie to insert into Digest.

2. Node::Leaf
In order to simplify the number of cases we handle, we convert leaf key and value into the Entry and replace it with the new node created using `from_two_entries` function. This way we treat old entry and the new one symmetricaly and have less cases to consider.




Co wytlumaczyc:
* czym jest entry - DONE
* dlaczego entry jest potrzebne - DONE
* jak dziala wkladanie do nulla  
* jak dziala wkladanie do digesta
* jak dziala wkladanie do leafa
* jak dziala wkladanie do brancha
* jak dziala wkladanie do extension


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

### Insert