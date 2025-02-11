# Proof composition

Proof composition is explained in RISC Zero documentation, the verifiable computation tooling used by vlayer. For more details, refer to their resources:
* [terminology](https://dev.risczero.com/terminology#composition)
* [receipts](https://dev.risczero.com/api/zkvm/receipts)
* [composition](https://dev.risczero.com/api/zkvm/composition)

This page aims to describe it from a practical perspective focusing on our use-case.

## Usage

We use proof composition in Chain Proofs. [The trie is correct if](./architecture/chain_proof/coherence.md#adding-hashes-to-the-bpc-structure-and-maintaining-𝜋):
* the previous trie was correct;
* the operation executed is correct.

In order to verify first point - we need to verify a ZK proof (correctness of the previous step) from within a ZK proof (correctness of this step).

## Implementation

Proofs that we store in the DB are `bincode` serialized `Receipts`.

`Receipt` contains:
* `Journal` - proof output: Bytes
* `Inner receipt` - polymorphic receipt

```rs
enum InnerReceipt {
    /// Linear size receipt. We don't use that
    Composite,
    /// Constant size STARK receipt
    Succinct,
    /// Constant size SNARK receipt
    Groth16,
    /// Fake receipt
    Fake,
}
```

In order to use one proof within another in ZKVM - we need to convert a `Receipt` into an [`Assumption`](https://dev.risczero.com/terminology#assumption).
This is trivial as `AssumptionReceipt` implements `From<Receipt>`.
```rs
executor_env_builder.add_assumption(receipt.into());
```

Within Guest - one should use [env::verify](https://docs.rs/risc0-zkvm/1.1.2/risc0_zkvm/guest/env/fn.verify.html) function:
```rs
use risc0_zkvm::guest::env;

env::verify(HELLO_WORLD_ID, b"journal".as_slice()).unwrap();
```

This function accepts guest ID, journal and not the proof as all the available proofs are stored within `env`.

**Important**
Proof composition only works on `Succinct` proofs and not `Groth16` proofs.

In Chain Proofs - we store all proofs as `Succinct` receipts. Chain Proof gets injected into Call Proof as `Succinct` receipt. In the end Call Proof gets converted into a `Groth16` receipt to be verified in a Smart Contract