.
├── **.cargo [OUT OF SCOPE]**
├── **.dockerignore [OUT OF SCOPE]**
├── **.git-branches.toml [OUT OF SCOPE]**
├── .**github [OUT OF SCOPE]**
├── **.gitignore [OUT OF SCOPE]**
├── **.shellcheckrc [OUT OF SCOPE]**
├── **.solhint.json [OUT OF SCOPE]**
├── **.vscode [OUT OF SCOPE]**
├── **Cargo.lock [OUT OF SCOPE]**
├── Cargo.toml
├── **LICENSE [OUT OF SCOPE]**
├── **README.md [OUT OF SCOPE]**
├── **ansible [OUT OF SCOPE]**
├── **bash [OUT OF SCOPE]**
├── **book [OUT OF SCOPE]**
├── **bun.lockb [OUT OF SCOPE]**
├── **clippy.toml [OUT OF SCOPE]**
├── contracts
│   ├── **fixtures [OUT OF SCOPE]**
│   ├── **package.json [OUT OF SCOPE]**
│   └── vlayer
│       ├── foundry.toml
│       ├── **package.json [OUT OF SCOPE]**
│       ├── **remappings.txt [OUT OF SCOPE]**
│       ├── **soldeer.lock [OUT OF SCOPE]**
│       ├── src
│       │   ├── CallAssumptions.sol
│       │   ├── EmailProof.sol
│       │   ├── **ImageID.sol [OUT OF SCOPE]**
│       │   ├── PrecompilesAddresses.sol
│       │   ├── Proof.sol
│       │   ├── Prover.sol
│       │   ├── Regex.sol
│       │   ├── Seal.sol
│       │   ├── URLPattern.sol
│       │   ├── Verifier.sol
│       │   ├── WebProof.sol
│       │   ├── proof_verifier
│       │   │   ├── ChainId.sol
│       │   │   ├── **FakeProofVerifier.sol [OUT OF SCOPE]**
│       │   │   ├── Groth16ProofVerifier.sol
│       │   │   ├── IProofVerifier.sol
│       │   │   ├── ImageIdRepository.sol
│       │   │   ├── ProofVerifierBase.sol
│       │   │   ├── **ProofVerifierFactory.sol [OUT OF SCOPE]**
│       │   │   └── **ProofVerifierRouter.sol [OUT OF SCOPE]**
│       │   └── **testing [OUT OF SCOPE]**
│       ├── **test [OUT OF SCOPE]**
│       └── **testdata [OUT OF SCOPE]**
├── **docker [OUT OF SCOPE]**
├── **examples [OUT OF SCOPE]**
├── **flake.lock [OUT OF SCOPE]**
├── **flake.nix [OUT OF SCOPE]**
├── **nix [OUT OF SCOPE]**
├── **package.json [OUT OF SCOPE]**
├── **packages [OUT OF SCOPE]**
├── rust
│   ├── block_header
│   │   ├── Cargo.toml
│   │   ├── src
│   │   │   ├── casting_utils.rs
│   │   │   ├── eth.rs
│   │   │   ├── forge.rs
│   │   │   └── lib.rs
│   │   └── **testdata [OUT OF SCOPE]**
│   ├── chain
│   │   ├── Cargo.toml
│   │   ├── chain_specs.toml
│   │   └── src
│   │       ├── config.rs
│   │       ├── eip1559.rs
│   │       ├── fork.rs
│   │       ├── lib.rs
│   │       └── spec.rs
│   ├── **cli [OUT OF SCOPE]**
│   ├── common
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── cache.rs
│   │       ├── **cli.rs [OUT OF SCOPE]**
│   │       ├── guest.rs
│   │       ├── hashable.rs
│   │       ├── lib.rs
│   │       ├── **rpc.rs [OUT OF SCOPE]**
│   │       └── **trace.rs [OUT OF SCOPE]**
│   ├── email_proof
│   │   ├── Cargo.toml
│   │   ├── src
│   │   │   ├── dkim.rs
│   │   │   ├── dns.rs
│   │   │   ├── email
│   │   │   │   └── sol.rs
│   │   │   ├── email.rs
│   │   │   ├── email_address.rs
│   │   │   ├── errors.rs
│   │   │   ├── from_header.rs
│   │   │   ├── lib.rs
│   │   │   └── **test_utils.rs [OUT OF SCOPE]**
│   │   └── **testdata [OUT OF SCOPE]**
│   ├── guest_wrapper
│   │   ├── Cargo.toml
│   │   ├── artifacts
│   │   │   └── chain_guest
│   │   │       ├── CHANGELOG.md
│   │   │       ├── elf_id
│   │   │       └── elf_id_history
│   │   ├── build.rs
│   │   ├── build_utils
│   │   │   ├── Cargo.toml
│   │   │   └── src
│   │   │       ├── chain_guest_id.rs
│   │   │       ├── data_layout.rs
│   │   │       ├── lib.rs
│   │   │       └── risc0_builder.rs
│   │   ├── **chain_guest_elf_id [OUT OF SCOPE]**
│   │   ├── risc0_call_guest
│   │   │   ├── **Cargo.lock [OUT OF SCOPE]**
│   │   │   ├── Cargo.toml
│   │   │   ├── build.rs
│   │   │   └── src
│   │   │       └── main.rs
│   │   ├── risc0_chain_guest
│   │   │   ├── **Cargo.lock [OUT OF SCOPE]**
│   │   │   ├── Cargo.toml
│   │   │   ├── build.rs
│   │   │   └── src
│   │   │       └── main.rs
│   │   └── src
│   │       └── lib.rs
│   ├── **host_utils [OUT OF SCOPE]**
│   ├── **key_value [OUT OF SCOPE]**
│   ├── mpt
│   │   ├── Cargo.toml
│   │   ├── **README.md [OUT OF SCOPE]**
│   │   ├── **clippy.toml [OUT OF SCOPE]**
│   │   ├── **images [OUT OF SCOPE]**
│   │   ├── src
│   │   │   ├── hash.rs
│   │   │   ├── key_nibbles.rs
│   │   │   ├── lib.rs
│   │   │   ├── node
│   │   │   │   ├── constructors.rs
│   │   │   │   ├── insert
│   │   │   │   │   ├── entry.rs
│   │   │   │   │   ├── from_two_entries.rs
│   │   │   │   │   ├── insert_entry_into_branch.rs
│   │   │   │   │   ├── insert_entry_into_extension
│   │   │   │   │   │   └── from_extension_and_entry_empty_common_prefix.rs
│   │   │   │   │   ├── insert_entry_into_extension.rs
│   │   │   │   │   ├── **tests.rs [OUT OF SCOPE]**
│   │   │   │   │   └── utils.rs
│   │   │   │   ├── insert.rs
│   │   │   │   ├── rlp.rs
│   │   │   │   └── size.rs
│   │   │   ├── node.rs
│   │   │   ├── node_ref.rs
│   │   │   ├── path.rs
│   │   │   ├── trie
│   │   │   │   ├── **tests [OUT OF SCOPE]**
│   │   │   │   ├── **tests.rs [OUT OF SCOPE]**
│   │   │   │   └── utils.rs
│   │   │   ├── trie.rs
│   │   │   └── utils.rs
│   │   └── **tests [OUT OF SCOPE]**
│   ├── **provider [OUT OF SCOPE]**
│   ├── **range [OUT OF SCOPE]**
│   ├── **server_utils [OUT OF SCOPE]**
│   ├── services
│   │   ├── **call [TODO: Leo]**
│   │   │   ├── engine
│   │   │   │   ├── Cargo.toml
│   │   │   │   └── src
│   │   │   │       ├── config.rs
│   │   │   │       ├── consts.rs
│   │   │   │       ├── db.rs
│   │   │   │       ├── evm
│   │   │   │       │   ├── env
│   │   │   │       │   │   ├── cached.rs
│   │   │   │       │   │   ├── factory.rs
│   │   │   │       │   │   └── location.rs
│   │   │   │       │   ├── env.rs
│   │   │   │       │   ├── execution_result.rs
│   │   │   │       │   └── input.rs
│   │   │   │       ├── evm.rs
│   │   │   │       ├── inspector.rs
│   │   │   │       ├── io.rs
│   │   │   │       ├── lib.rs
│   │   │   │       ├── sol
│   │   │   │       │   ├── call_assumptions.rs
│   │   │   │       │   ├── proof.rs
│   │   │   │       │   └── seal.rs
│   │   │   │       ├── sol.rs
│   │   │   │       ├── travel_call_executor.rs
│   │   │   │       ├── utils
│   │   │   │       │   ├── cache.rs
│   │   │   │       │   └── evm_call.rs
│   │   │   │       ├── utils.rs
│   │   │   │       ├── verifier
│   │   │   │       │   ├── chain_proof.rs
│   │   │   │       │   ├── **tests [OUT OF SCOPE]**
│   │   │   │       │   ├── **tests.rs [OUT OF SCOPE]**
│   │   │   │       │   ├── time_travel.rs
│   │   │   │       │   ├── travel_call.rs
│   │   │   │       │   └── zk_proof.rs
│   │   │   │       └── verifier.rs
│   │   │   ├── guest
│   │   │   │   ├── Cargo.toml
│   │   │   │   └── src
│   │   │   │       ├── db
│   │   │   │       │   ├── state.rs
│   │   │   │       │   └── wrap_state.rs
│   │   │   │       ├── db.rs
│   │   │   │       ├── guest
│   │   │   │       │   ├── env.rs
│   │   │   │       │   └── **tests.rs [OUT OF SCOPE]**
│   │   │   │       ├── guest.rs
│   │   │   │       └── lib.rs
│   │   │   ├── **host [OUT OF SCOPE]**
│   │   │   ├── precompiles
│   │   │   │   ├── Cargo.toml
│   │   │   │   └── src
│   │   │   │       ├── json.rs
│   │   │   │       ├── lib.rs
│   │   │   │       ├── regex.rs
│   │   │   │       ├── url_pattern.rs
│   │   │   │       ├── verify_and_parse.rs
│   │   │   │       └── verify_and_parse_email.rs
│   │   │   ├── seal
│   │   │   │   ├── Cargo.toml
│   │   │   │   └── src
│   │   │   │       └── lib.rs
│   │   │   ├── **server [OUT OF SCOPE]**
│   │   │   └── **server_lib [OUT OF SCOPE]**
│   │   ├── **chain [TODO: Leo]**
│   │   │   ├── block_trie
│   │   │   │   ├── Cargo.toml
│   │   │   │   └── src
│   │   │   │       └── lib.rs
│   │   │   ├── **client [OUT OF SCOPE]**
│   │   │   ├── common
│   │   │   │   ├── Cargo.toml
│   │   │   │   └── src
│   │   │   │       └── lib.rs
│   │   │   ├── **db [OUT OF SCOPE]**
│   │   │   ├── guest
│   │   │   │   ├── Cargo.toml
│   │   │   │   └── src
│   │   │   │       └── lib.rs
│   │   │   ├── **host [OUT OF SCOPE]**
│   │   │   ├── **mock_server [OUT OF SCOPE]**
│   │   │   ├── **server [OUT OF SCOPE]**
│   │   │   ├── **server_lib [OUT OF SCOPE]**
│   │   │   ├── **test_utils [OUT OF SCOPE]**
│   │   │   └── **worker [OUT OF SCOPE]**
│   │   └── **dns [OUT OF SCOPE]**
│   ├── **test_runner [OUT OF SCOPE]**
│   ├── **verifiable_dns [TODO: Piotr]**
│   │   ├── Cargo.toml
│   │   ├── **assets [OUT OF SCOPE]**
│   │   └── src
│   │       ├── common
│   │       │   ├── record.rs
│   │       │   ├── **test_utils.rs [OUT OF SCOPE]**
│   │       │   ├── to_payload.rs
│   │       │   └── types.rs
│   │       ├── common.rs
│   │       ├── **dns_over_https [OUT OF SCOPE]**
│   │       ├── dns_over_https.rs
│   │       ├── lib.rs
│   │       ├── **verifiable_dns [OUT OF SCOPE]**
│   │       ├── verifiable_dns.rs
│   │       └── verifier
│   │           └── mod.rs
│   ├── **version [OUT OF SCOPE]**
│   ├── **web_proof [TODO: Wiktor]**
│   │   ├── Cargo.toml
│   │   ├── src
│   │   │   ├── errors.rs
│   │   │   ├── fixtures
│   │   │   │   └── tlsn_core_types.rs
│   │   │   ├── fixtures.rs
│   │   │   ├── lib.rs
│   │   │   ├── redaction.rs
│   │   │   ├── request_transcript.rs
│   │   │   ├── response_transcript.rs
│   │   │   ├── transcript_parser.rs
│   │   │   ├── utils
│   │   │   │   ├── bytes.rs
│   │   │   │   └── json.rs
│   │   │   ├── utils.rs
│   │   │   ├── verifier.rs
│   │   │   ├── web.rs
│   │   │   └── web_proof.rs
│   │   └── **testdata [OUT OF SCOPE]**
│   └── **zkvm-benchmarks [OUT OF SCOPE]**
├── **rust-toolchain.toml [OUT OF SCOPE]**
└── **rustfmt.toml [OUT OF SCOPE]**
