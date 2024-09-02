# Block Proof Cache

vlayer executes Solidity code off-chain and proves the correctness of that execution on-chain. For that purpose, it fetches (state and) storage data and verifies it with storage proofs. Storage proofs prove that a piece of storage is part of a block with a specific hash. Hence, the storage proof is 'connected' to a certain block hash. However, they don't guarantee that the block with the specific hash actually exists on the chain. This verification needs to be done later with an on-chain smart contract.

vlayer provides time-travel functionality. As a result, state and storage proofs are not 'connected' to a single block hash, but to multiple block hashes. To ensure that these hashes exist on the chain, two things need to be done:

1. First, it needs to be proven that all the hashes belong to the same chain. However, the blocks might belong to an imaginary chain and not a real one. That's why a second step is needed.
2. Second, the latest hash needs to be verified on-chain.

The **Block Proof Cache** service allows for proving **the first point** by maintaining a data structure that stores block hashes, along with a ZK proof that all the hashes it contains belong to the same chain. Below we provide more details.

## Before diving into Block Proof Cache

Before diving into the details of Block Proof Cache, it is recommended to go through, or at least glance over, the two topics below.

### Recent and historical blocks

As mentioned, it is essential to be able to verify a hash on-chain. The way to do this is to run the Solidity `blockhash(uint)` function, which returns the corresponding hash for a given block number. The hash to be verified needs to be compared to the result of the function (with the block number taken from the storage proof).

However, this method is limited, as it only works for the most recent 256 blocks on a given chain. That is why we need to ensure that the latest hash to be verified on-chain is a hash of a recent block. If it is not, it needs to be added to the set of hashes.

We use the following terminology in this document:

- **recent blocks** - any of the most recent 256 blocks (relative to the current block number)
- **historical blocks** - blocks older than 256

### Naive chain proofs

Returning to the first point from the introduction, we need a way to prove that a set of hashes belongs to the same chain. A naive way to do this is to hash all subsequent blocks from the oldest to the most recent and verify that each block hash is equal to the **prevHash** value of the subsequent block. If all the hashes from our set appear along the way, then they all belong to the same chain.

See the diagram below for a visual representation.

![Schema](/images/architecture/block-proof.png)

Unfortunately, this is a slow process, especially if the blocks are far apart on the time scale. Fortunately, with the help of Block Proof Cache, this process can be sped up to logarithmic time.

## Block Proof Cache

The Block Proof Cache service maintains two things:
- a Block Proof Cache structure (Merkle Patricia Trie) that stores block hashes,
- a ZK proof 𝜋 that all these hashes belong to the same chain.

Given these two elements, it is easy to prove that a set of hashes belongs to the same chain.
1. It needs to be verified that all the hashes are part of the Block Proof Cache structure.
2. 𝜋 needs to be verified.

### Block Proof Cache (BPC) structure

The Block Proof Cache structure is a dictionary that stores a `<block_number, block_hash>` mapping. It is implemented using a Merkle Patricia Trie. This enables us to prove that a set of hashes is part of the structure (point 1 from the previous paragraph) by supplying their corresponding Merkle proofs.

#### Adding hashes to the BPC structure and maintaining 𝜋

At all times, the blocks stored in the BPC structure form a consistent subchain. In other words, we preserve the invariant that for every pair of block numbers `i, i+1` contained in the structure, `block(i + 1).parentHash = hash(block(i))`.

Each time a block is added, 𝜋 is updated. To prove that after adding a new block, all the blocks in the BPC structure belong to the same chain, two things must be done:
- The previous 𝜋 must be verified.
- It must be ensured that the hash of the new block 'links' to either the oldest or the most recent block.

The following functions, written in pseudocode, provide more details on the Block Proof Cache implementation.

### Implementation

#### Initialize

The initialize function is used to create Block Proof Cache as a Merkle Patricia Trie and insert the initial block's hash into it. It takes the following arguments:

- **elf_id**: a hash of the guest binary.
- **block**: the block header of the block to be added.

It calculates the hash of the block using the keccak256 function on the RLP-encoded block. Then it inserts this hash into the MPT at the position corresponding to the block number. Notice that no invariants about neighbours are checked as there are no neighbours yet.

```rs
fn initialize(elf_id: Hash, block: BlockHeader) -> (MptRoot, elf_id) {
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
