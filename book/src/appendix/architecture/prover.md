# Call Prover architecture

vlayer enables three key functionalities: **_accessing_** different sources of verifiable data, **_aggregating_** this data in a verifiable way to obtain verifiable result and **_using the verifiable result on-chain_**.


It supports accessing verifiable data from three distinct sources: HTTP requests, emails and EVM state and storage. For each source, a proof of validity can be generated:
- HTTP requests can be verified using a Web Proof, which includes information about the TLS session, a transcript of the HTTP request and response signed by a *TLS Notary*
- Email contents can be proven by verifying DKIM signatures and checking the sender domain
- EVM state and storage proofs can be verified against the block hash via Merkle Proofs

Before vlayer, ZK programs were application-specific and proved a single source of data. vlayer allows you to write a Solidity smart contract (called **Prover**) that acts as a glue between all three possible data sources and enables you to **aggregate** this data in a verifiable way - we not only prove that the data we use is valid but also that it was processed correctly by the **Prover**.

### Aggregation examples

- **Prover** computing average _ERC20_ balance of addresses
- **Prover** returning _true_ if someone starred a _GitHub Org_ by verifying a Web Proof

> **_Note:_** Despite being named "Prover", the **Prover** contract does not compute the proof itself. Instead, it is executed inside the _zkEVM_, which produces the proof of the correctness of its execution.

**_Call Proof_** is a proof that we correctly executed the **Prover** smart contract and got the given result.

It can be later verified by a deployed **Verifier** contract to **use the verifiable result on-chain**.

But how are Call Proofs obtained?

## Call Prover

To obtain Call Proofs, the **Call Prover** is used. It is a Rust server that exposes the [`v_call`](../api.md#v_call) JSON-RPC endpoint. The three key components of the prover are the **Guest**, **Host**, and **Engine**. The **Guest** executes code within the *zkEVM* to generate a proof of execution. The **Host** prepares the necessary data and sends it to the Guest. The **Engine**, responsible for executing the EVM, runs in both the Guest and Host environments. 

Their structure and responsibilities are as follows:

- **Guest**: Performs execution of the code inside zkEVM. Consists of three crates:
  - `guest` (in `services/call/guest`): Library that contains code for EVM execution and input validation.
  - `risc0_guest` (in `guest_wrapper/risc0_call_guest`): Thin wrapper that uses RISC0 ZKVM I/O and delegates work to `guest`.
  - `guest_wrapper` (in `guest_wrapper`): Compiles `risc0_guest` (using cargo build scripts) to a binary format (ELF) using [RISC Zero](https://doc.rust-lang.org/rustc/platform-support/riscv32im-risc0-zkvm-elf.html) target.
- **Host** (in `services/call/host`): Runs a **_preflight_**, during which it collects all the data required by the guest. It retrieves data from online sources (RPC clients) and then triggers guest execution (which is done offline).
- **Engine** (in `services/call/engine`): Sets up and executes the EVM, which executes `Prover` smart contract (including calling custom [Precompiles](#precompiles)). It executes exactly the same code in _preflight_ and Guest execution.

Our architecture is heavily inspired by RISC Zero [steel](https://github.com/risc0/risc0-ethereum/tree/main/steel).

Currently, the Guest is compiled with Risc0, but we aim to build vendor-lock free solutions working on multiple zk stacks, like [sp-1](https://github.com/succinctlabs/sp1) or [Jolt](https://github.com/a16z/jolt).

### Execution and proving

The Host passes arguments to the Guest via standard input (stdin), and similarly, the Guest returns values via standard output (stdout). zkVM works in isolation, without access to a disk or network.

On the other hand, when executing Solidity code in the Guest, it needs access to the Ethereum state and storage. The state consist of Ethereum accounts (i.e. balances, contracts code and nonces) and the storage consist of smart contract variables. Hence, all the state and storage needs to be passed via input.

However, all input should be considered insecure. Therefore, validity of all the state and storage needs to be proven.

> **_Note_:** In off-chain execution, the notion of the current block doesn't exist, hence we always access Ethereum at a specific historical block. The block number doesn't have to be the latest mined block available on the network. This is different than the current block inside on-chain execution, which can access the state at the moment of execution of the given transaction.

To deliver all necessary proofs, the following steps are performed:

1. In preflight, we execute Solidity code on the host. Each time the database is called, the value is fetched via Ethereum JSON RPC and the proof is stored in it. This database is called `ProofDb`
2. Serialized content of `ProofDb` is passed via stdin to the `guest`
3. `guest` deserializes content into a `StateDb`
4. Validity of the data gathered during the preflight is verified in `guest`
5. Solidity code is executed inside revm using `StateDb`

Since that Solidity execution is deterministic, database in the guest has exactly the data it requires.

![Schema](/images/architecture/prover.png)

### Databases

revm requires us to provide a DB which implements `DatabaseRef` trait (i.e. can be asked about accounts, storage, block hashes).

It's a common pattern to compose databases to orthogonalize the implementation.

We have **Host** and **Guest** databases

- **Host** - runs `CacheDB<ProofDb<ProviderDb>>`:
  - `ProviderDb`- queries Ethereum RPC Provider (i.e. Alchemy, Infura, Anvil);
  - `ProofDb` - records all queries aggregates them and collects EIP1186 (`eth_getProof`) proofs;
  - `CacheDB` - stores trusted seed data to minimize the number of RPC requests. We seed caller account and some Optimism system accounts.
- **Guest** - runs `CacheDB<WrapStateDb<StateDb>>`:
  - `StateDb` consists of state passed from the host and has only the content required to be used by deterministic execution of the Solidity code in the guest. Data in the `StateDb` is stored as [sparse Merkle Patricia Tries](https://github.com/vlayer-xyz/vlayer/tree/main/rust/mpt), hence access to accounts and storage serves as verification of state and storage proofs;
  - `WrapStateDb` is an [adapter](https://en.wikipedia.org/wiki/Adapter_pattern) for `StateDb` that implements `Database` trait. It additionally does caching of the accounts, for querying storage, so that the account is only fetched once for multiple storage queries;
  - `CacheDB` - has the same seed data as it's Host version.

### EvmEnv and EvmInput

vlayer enables aggregating data from multiple blocks and multiple chains. We call these features **Time Travel** and **Teleport**. To achieve that, we span multiple revm instances during Engine execution. Each revm instance corresponds to a certain block number on a certain chain.

`EvmEnv` struct represents a configuration required to create a revm instance. Depending on the context, it might be instantiated with `ProofDB` (Host) or `WrapStateDB` (Guest).

It is also implicitly parameterized via dynamic dispatch by the `Header` type, which may differ for various hard forks or networks.

```rust
pub struct EvmEnv<DB> {
    pub db: DB,
    pub header: Box<dyn EvmBlockHeader>,
    ...
}
```

The serializable input we pass between host and guest is called `EvmInput`. `EvmEnv` can be obtained from it.

```rust
pub struct EvmInput {
    pub header: Box<dyn EvmBlockHeader>,
    pub state_trie: MerkleTrie,
    pub storage_tries: Vec<MerkleTrie>,
    pub contracts: Vec<Bytes>,
    pub ancestors: Vec<Box<dyn EvmBlockHeader>>,
}
```

Because we may have multiple blocks and chains, we also have structs `MultiEvmInput` and `MultiEvmEnv`, mapping `ExecutionLocation`s to `EvmInput`s or `EvmEnv`s equivalently.

```rust
pub struct ExecutionLocation {
    pub chain_id: ChainId,
    pub block_tag: BlockTag,
}
```

### CachedEvmEnv

`EvmEnv` instances are accessed in both Host and Guest using [`CachedEvmEnv`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/engine/src/evm/env/cached.rs#L27) structure. However, the way `CachedEvmEnv` is constructed differs between these two contexts.

#### Structure

```rust
pub struct CachedEvmEnv<D: RevmDB> {
    cache: MultiEvmEnv<D>,
    factory: Mutex<Box<dyn EvmEnvFactory<D>>>,
}
```

- **cache**: `HashMap` (aliased as `MultiEvmEnv<D>`) that stores `EvmEnv` instances, keyed by their `ExecutionLocation`
- **factory**: used to create new `EvmEnv` instances

#### On Host

On Host, `CachedEvmEnv` is created using [`from_factory`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/engine/src/evm/env/cached.rs#L41) function. It initializes `CachedEvmEnv` with an empty cache and a factory of type [`HostEvmEnvFactory`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/host/src/evm_env/factory.rs#L19) that is responsible for creating `EvmEnv` instances.

```rust
pub(crate) struct HostEvmEnvFactory {
    providers: CachedMultiProvider,
}
```

`HostEvmEnvFactory` uses a `CachedMultiProvider` to fetch necessary data (such as block headers) and create new `EvmEnv` instances on demand.

#### On Guest

On Guest, `CachedEvmEnv` is created using [`from_envs`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/engine/src/evm/env/cached.rs#L49). This function takes a pre-populated cache of `EvmEnv` instances (created on Host) and initializes the `factory` field with [`NullEvmEnvFactory`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/engine/src/evm/env/cached.rs#L17).

`NullEvmEnvFactory` is a dummy implementation that returns an error when its `create` method is called. This is acceptable because, in Guest context, there is no need to create new environments — only the cached ones are used.

### Verifying data

Guest is required to verify all data provided by the Host. Initial validation of its coherence is done in two places:

- [`multi_evm_input.assert_coherency`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/engine/src/evm/input.rs#L46) verifies:

  - Equality of subsequent `ancestor` block hashes
  - Equality of `header.state_root` and actual `state_root`

- When we create `StateDb` in Guest with [`StateDb::new`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/guest/src/db/state.rs#L51), we compute hashes for `storage_tries` roots and `contracts` code. When we later try to access storage (using the [`WrapStateDb::basic_ref`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/guest/src/db/wrap_state.rs#L39) function) or contract code (using the [`WrapStateDb::code_by_hash_ref`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/guest/src/db/wrap_state.rs#L70) function), we know this data is valid because the hashes were computed properly. If they weren't, we wouldn't be able to access the given storage or code. Thus, storage verification is done indirectly.

Above verifications are not enough to ensure validity of Time Travel (achieved by [Chain Proofs](./chain_proof.md)) and Teleport. Travel call verification is described in the [next](./time_travel_teleport.md) section.

## Precompiles

As shown in the diagram in the [Execution and Proving](#execution-and-proving) section, the **Engine** executes the EVM, which in turn runs the Solidity `Prover` smart contract. During execution, the contract may call custom precompiles available within the vlayer *zkEVM*, enabling various advanced features.

The list, configuration, and addresses of these precompiles are defined in `services/call/precompiles`. These precompiles can be easily accessed within Solidity `Prover` contracts using libraries included in the vlayer Solidity smart contracts package.

### Available precompiles and their functionality
- **`WebProof.verify`** (via `WebProofLib`):  
  Verifies a `WebProof` and returns a `Web` object containing:
  - `body` (HTTP response body)
  - `notaryPubKey` (TLS Notary’s public key that signed the Web Proof)
  - `url` (URL of the HTTP request)  
  
  See [Web Proof](../../features/web.md) for details.
  
- **`Web.jsonGetString`**, **`Web.jsonGetInt`**, **`Web.jsonGetBool`**, **`Web.jsonGetFloatAsInt`** (via `WebLib`):  
  Parses JSON from an HTTP response body (`Web.body`).  
  See [JSON Parsing](../../features/json-and-regex.md#json-parsing) for more information.
  
- **`UnverifiedEmail.verify`** (via `EmailProofLib`):  
  Verifies an `UnverifiedEmail` and returns a `VerifiedEmail` object containing:
  - `from` (sender's email address)
  - `to` (recipient's email address)
  - `subject` (email subject)
  - `body` (email body)  
  
  See [Email Proof](../../features/email.md).
  
- **`string.capture`**, **`string.match`** (via `RegexLib`):  
  Performs regex operations on strings.  
  
  See [Regular Expressions](../../features/json-and-regex.md#regular-expressions).
  
- **`string.test`** (via `URLPatternLib`):  
  Used within `WebProof.verify` to validate `Web.url` against a given URL pattern.  
  
  See [Web Proof](../../features/web.md).

### Error handling	

Error handling is done via `HostError` enum type, which is converted into http code and a human-readable string by the server.

Instead of returning a result, to handle errors, `Guest` panics. It does need to panic with a human-readable error, which should be converted on `Host` to a semantic `HostError` type. As execution on `Guest` is deterministic and should never fail after a successful preflight, the panic message should be informative for developers.
