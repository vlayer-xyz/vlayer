# Block proof cache

The vlayer infrastructure enables the creation of proofs to verify the current and historical state of Ethereum and layer 2 chains. To understand how this system works under the hood, it is essential to become familiar with the concepts of block proofs, storage proofs and understand the difference between recent and historical blocks. Then we will see how historical blockchain state data is preserved in the verifiable way.

## Block inclusion proofs primer

### Block and Storage proofs

A **block proof** verifies that a particular block belongs to a specific blockchain, ensuring the block's authenticity and its place in the chain. A \*\*storage proof, on the other hand, specifically verifies that a piece of data, such as an account balance or a smart contract variable, is stored within a particular state in a specific block.

To ensure that a piece of state belongs to a certain chain, it is essential to provide both types of proofs. A storage proof demonstrates that the data is part of a specific block, while a block proof confirms that this block is indeed a legitimate part of the blockchain.

### Recent and historical blocks

One way to prove that a block of a certain hash belongs to a chain is to run the Solidity `blockhash(uint)` function. It returns the hash of a block for a given number.
To perform a check, we need to hash a block with a certain state root and compare it with the result of the function.

However, this method is limited, as it only works for most recent 256 blocks on a given chain.
Therefore, we need another way to prove inclusion of older blocks in the chain.

We use the following naming in this document:

- **recent blocks** - any of the most recent 256 blocks (relative to the current `block_no`)
- **historical blocks** - blocks older than 256

### Naive block inclusion proofs

To prove inclusion of certain **historical blocks** in a chain, we will prove that:

1. Some **recent block** belong to the chain
2. Both **historical block** and **recent block** belong to the same chain

A naive way to prove the inclusion proof of two blocks in the same chain is to hash all subsequent blocks from **historical block** to **recent block** and verify that each blockhash is equal to the **prevHash** value of the subsequent block.

See the diagram below for the visual.

![Schema](/images/architecture/block-proof.png)

Unfortunately, this is a slow process, especially if the blocks are far away from each other on the time scale. Fortunately, there is a way to cache all proofs in advance. For this purpose, we cache block proofs in the way, that enables us to verify its correctness reliably and quickly.

## Block Proof Cache

The Block Proof Cache stores `<block_number, blockhash>` mapping for historical blocks. It is implemented using a Merkle Patricia Trie, where block numbers are the keys and blockhashes are the values. The construction is inductive, in which we preserve the invariant that each block stored in the Block Proof Cache has an immediate parent or child block. In order to prove correctness of construction with `N+1` nodes we verify that a ZK proof of construction with `N` nodes is correct. We do this by recursively by verifying ZK proof inside ZK proof. When this is verified, we check `N+1` step, by ensuring new block data fits existing structure. Then we generate a new ZK proof for computation appending or prepending the next block hash to the trie.

The following functions written in pseudocode provide more details on the Block Proof Cache implementation.

### Implementation

#### Initialize

The initialize function is used to create Block Proof Cache as a Merkle Patricia Trie and insert the initial block's hash into it. It takes the following arguments:

- **elf_id**: a hash of the guest binary.
- **block**: the block header to be added.

It calculates the hash of the block using the keccak256 function on the RLP-encoded block. Then it inserts this hash into the MPT at the position corresponding to the block number. Notice that no invariants about neighbours are checked as there are no neighbours yet.

```rs
fn initialize(elf_id: Hash, block: Block) -> (MptRoot, elf_id) {
    let block_hash = keccak256(rlp(block));
    let mpt = new SparseMpt();
    mpt.insert(block.number, block_hash);
    (mpt.root, elf_id)
}
```

#### Append

The append function is used to add a new block to the Merkle Patricia Trie. It takes the following arguments:

- **elf_id**: a hash of the guest binary.
- **block**: the block header to be added.
- **mpt**: a sparse MPT containing two paths: one from the root to the parent block and one from the root to the node where the new block will be inserted.
- **proof**: a zero-knowledge proof (zk-proof) that verifies the correctness of the MPT so far.
  The function ensures that the new block correctly follows the previous block by checking the parent block's hash. If everything is correct, it inserts the new block's hash into the trie.

```rs
fn append(elf_id: Hash, block: BlockHeader, mpt: SparseMpt<ParentBlockIdx, NewBlockIdx>, proof: ZkProof) -> (MptRoot, elf_id) {
    risc0_std::verify_zk_proof(proof, mpt.root, elf_id);
    let parent_block_idx = block.number - 1;
    let parent_block_hash = mpt.get(parent_block_idx);
    assert_eq(parent_block_hash, block.parent_hash, "Block hash mismatch");
    let block_hash = keccak256(rlp(block));
    let new_mpt = mpt.insert(block.number, block_hash);
    (new_mpt.root, elf_id)
}
```

#### Prepend

The prepend function is used to add a new oldest block to the Merkle Patricia Trie (MPT). It takes the following arguments:

- **elf_id**: a hash of the guest binary.
- **child_block**: the full data of the currently oldest block already stored in the MPT.
- **mpt**: a sparse MPT containing the path from the root to the child block and the new block's intended position.
- **proof**: a zero-knowledge proof (zk-proof) that verifies the correctness of the MPT so far.
  The function verifies the proof to ensure the full data from the child block fits the MPT we have so far. If the verification succeeds, it takes the parent_hash from the currently oldest block and inserts it with the corresponding number into the MPT. Note that we don't need to pass the full parent block as the trie only store hashes. We will need to pass it next time we want to prepend though.

```rs
fn prepend(elf_id: Hash, child_block: BlockHeader, mpt: SparseMpt<ChildBlockIdx, NewBlockIdx>, proof: ZkProof) -> (MptRoot, elf_id) {
    risc0_std::verify_zk_proof(proof, mpt.root, elf_id);
    let child_block_hash = mpt.get(child_block.number);
    assert_eq(child_block_hash, keccak256(rlp(child_block)), "Block hash mismatch");
    let new_mpt = mpt.insert(child_block.number - 1, child_block.parent_hash);
    (new_mpt.root, elf_id)
}
```

### Block Proof Cache server

Block Proof Cache are stored in a distinct type of vlayer node, specifically a JSON-RPC server. It consists mainly of a single call `v_getBlockProofs(block_no: int[])`. This call takes one argument: an array of block numbers for the requested proofs. It returns a triplet: an array of Merkle proofs for each requested block, the root hash of the Merkle Patricia Trie structure, and π - a zk-proof of the correctness of the constructed MPT.

An example call could look like this:

```json
{
  "method": "v_getBlockProofs",
  "params": [1231, 123123123, 312312]
}
```

And the response:

```json
{
    "result": [
        [
            [...],
            [...],
            [...]
        ],
        "0xe32ddb9c538f04c994e2e802237fa5f4c4e7e2643ab48bd8535b1c7009c8aa81",
        "0x9c538f04c994e2e802237fa5f4c4e7e2643ab48bd8535b1c7009c8aa81e32ddb"
    ]
}
```
