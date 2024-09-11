# Node Internal Structure

`MerkleTrie` is a handle to a root `Node`, hence most of the logic occurs here.

## Node
`Node` type is defined in [node.rs](./src/node.rs).
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
    * If we try to insert into Digest node, it panics
* `size(&self) -> usize`
    * Returns the number of full nodes in the trie
    * Full nodes are nodes that need to be hashed to compute the root hash
    * This is useful for performance analysis
* RLP encoding/decoding
    * `rlp_encoded(&self) -> Vec<u8>`
    * `impl rlp::Decodable for Node`

More details regarding insert implementation can be found [here](./insert.md).

## NodeRef

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

## Path

The least trivial part of MPT is how it encodes path nibbles in `Leaf` and `Extension` nodes. There are two problems that this encoding solves.
* Both `Leaf` and `Extension` are two-element RLP arrays. We need a way to get back type info
* An odd and even number of nibbles look the same when compact-encoded as bytes

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

Unit tests exist for functions of the specific structs.

There are also four [e2e tests](../tests/):
* Testing that our implementation produces the same roots as `alloy_trie`
* Verifying that we can parse the results of `eth_getProof`
* Ensuring that after inserting a large number of elements into the MPT, we can later retrieve them using the get method, even when the structure becomes complex
* Testing that when we insert a large number of elements in two different orders, the resulting structure is the same (i.e., insertion is commutative)