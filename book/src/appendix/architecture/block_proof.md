# Block Proof

Vlayer executes Solidity code off-chain and proves the correctness of that execution on-chain. For that purpose, it fetches state and storage data and verifies it with storage proofs.

Storage proofs prove that a piece of storage is part of a block _with a specific hash_. We say the storage proof is 'connected' to a certain block hash.

However, the storage proof doesn't guarantee that the block with the specific hash actually exists on the chain. This verification needs to be done later with an on-chain **smart contract**.

## Motivation

Vlayer provides **time-travel functionality**. As a result, state and storage proofs are not *connected* to a single block hash, but to multiple block hashes. To ensure that all those hashes exist on the chain, it's enough to prove two things:

* **Coherence** - all the hashes belong to the same chain
* **Canonicity** - the last hash is a member of a canonical chain

![2-step verification](/images/architecture/block_proof/on-off-chain.png)

### Coherence

Will be proven using [**Block Proof Cache**](./block_proof/coherence.md) service.

It maintains a data structure that stores block hashes along with a zk-proof. The zk-proof proves that all the hashes contained by the data structure belong to the same chain.

### Canonicity

Since the latest hash needs to be verified on-chain, but generating proofs is a slow process; some fast chains might prune our latest block by the time we are ready to settle the proof. Proposed solution is described [here](./block_proof/canonicity.md).