```
├── Cargo.lock [OUT OF SCOPE]
├── Cargo.toml [OUT OF SCOPE]
├── LICENSE [OUT OF SCOPE]
├── README.md [OUT OF SCOPE]
├── ansible [OUT OF SCOPE]
├── bash [OUT OF SCOPE]
├── book [OUT OF SCOPE]
├── bun.lockb [OUT OF SCOPE]
├── clippy.toml [OUT OF SCOPE]
├── contracts
│   ├── fixtures
│   │   ├── README.md
│   │   ├── foundry.toml
│   │   ├── package.json -> ../package.json
│   │   ├── remappings.txt
│   │   ├── soldeer.lock
│   │   └── src
│   │       └── vlayer
│   │           ├── ArithOpProver.sol
│   │           ├── EmailProver.sol
│   │           ├── ExampleProver.sol
│   │           ├── LotrApiProver.sol
│   │           ├── NoopProver.sol
│   │           ├── NoopWithCalldataProver.sol
│   │           └── utils
│   │               └── AddressParser.sol
│   ├── package.json
│   └── vlayer
│       ├── foundry.toml
│       ├── package.json -> ../package.json
│       ├── remappings.txt
│       ├── soldeer.lock
│       ├── src
│       │   ├── CallAssumptions.sol
│       │   ├── EmailProof.sol
│       │   ├── ImageID.sol -> ../../../target/assets/ImageID.sol
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
│       │   │   ├── FakeProofVerifier.sol
│       │   │   ├── Groth16ProofVerifier.sol
│       │   │   ├── IProofVerifier.sol
│       │   │   ├── ProofVerifierBase.sol
│       │   │   ├── ProofVerifierFactory.sol
│       │   │   └── ProofVerifierRouter.sol
│       │   └── testing
│       │       ├── VTest.sol
│       │       └── libraries
│       │           └── EmailTestUtils.sol
│       ├── test [OUT OF SCOPE]
├── docker [OUT OF SCOPE]
├── examples [OUT OF SCOPE]
├── flake.lock [OUT OF SCOPE]
├── flake.nix [OUT OF SCOPE]
├── nix [OUT OF SCOPE]
├── package.json
├── packages [OUT OF SCOPE]
├── rust
│   ├── block_header
│   │   ├── Cargo.toml [OUT OF SCOPE]
│   │   ├── src
│   │   │   ├── casting_utils.rs
│   │   │   ├── eth.rs
│   │   │   ├── forge.rs
│   │   │   └── lib.rs
│   │   └── testdata [OUT OF SCOPE]
│   ├── chain
│   │   ├── Cargo.toml [OUT OF SCOPE]
│   │   ├── chain_specs.toml
│   │   └── src
│   │       ├── config.rs
│   │       ├── eip1559.rs
│   │       ├── fork.rs
│   │       ├── lib.rs
│   │       └── spec.rs
│   ├── cli [OUT OF SCOPE]
│   ├── common
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── cache.rs
│   │       ├── cli.rs
│   │       ├── guest.rs
│   │       ├── hashable.rs
│   │       ├── lib.rs
│   │       ├── rpc.rs
│   │       └── trace.rs
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
│   │   │   └── test_utils.rs
│   │   └── testdata
│   │       ├── email.txt
│   │       ├── multipart_email.eml
│   │       ├── signed_email.eml
│   │       ├── signed_email_different_domains.txt
│   │       ├── signed_email_dkim_subdomain.txt
│   │       ├── signed_email_from_subdomain.txt
│   │       └── signed_email_modified_body.txt
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
│   │   ├── chain_guest_elf_id
│   │   ├── risc0_call_guest
│   │   │   ├── Cargo.lock
│   │   │   ├── Cargo.toml
│   │   │   ├── build.rs
│   │   │   └── src
│   │   │       └── main.rs
│   │   ├── risc0_chain_guest
│   │   │   ├── Cargo.lock
│   │   │   ├── Cargo.toml
│   │   │   ├── build.rs
│   │   │   └── src
│   │   │       └── main.rs
│   │   └── src
│   │       └── lib.rs
│   ├── host_utils [OUT OF SCOPE]
│   ├── key_value [OUT OF SCOPE]
│   ├── mpt
│   │   ├── Cargo.toml
│   │   ├── README.md
│   │   ├── clippy.toml [OUT OF SCOPE]
│   │   ├── images [OUT OF SCOPE]
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
│   │   │   │   │   ├── tests.rs
│   │   │   │   │   └── utils.rs
│   │   │   │   ├── insert.rs
│   │   │   │   ├── rlp.rs
│   │   │   │   └── size.rs
│   │   │   ├── node.rs
│   │   │   ├── node_ref.rs
│   │   │   ├── path.rs
│   │   │   ├── trie
│   │   │   │   ├── tests [OUT OF SCOPE]
│   │   │   │   ├── tests.rs [OUT OF SCOPE]
│   │   │   │   └── utils.rs
│   │   │   ├── trie.rs
│   │   │   └── utils.rs
│   │   └── tests [OUT OF SCOPE]
│   ├── provider
│   │   ├── Cargo.toml
│   │   ├── src
│   │   │   ├── cache
│   │   │   │   └── json.rs
│   │   │   ├── cache.rs
│   │   │   ├── default.rs
│   │   │   ├── ethers.rs
│   │   │   ├── factory.rs
│   │   │   ├── lib.rs
│   │   │   ├── multi.rs
│   │   │   ├── never.rs
│   │   │   ├── profiling.rs
│   │   │   ├── proof.rs
│   │   │   └── provider_ext.rs
│   │   └── testdata  [OUT OF SCOPE]
│   ├── range
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── lib.rs
│   │       ├── non_empty_range.rs
│   │       └── range.rs
│   ├── server_utils [OUT OF SCOPE]
│   ├── services
│   │   ├── call
│   │   │   ├── engine
│   │   │   │   ├── Cargo.toml  [OUT OF SCOPE]
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
│   │   │   │       │   ├── guest_input.rs
│   │   │   │       │   ├── tests
│   │   │   │       │   │   ├── chain_proof.rs
│   │   │   │       │   │   └── guest_input.rs
│   │   │   │       │   ├── tests.rs
│   │   │   │       │   └── zk_proof.rs
│   │   │   │       └── verifier.rs
│   │   │   ├── guest
│   │   │   │   ├── Cargo.toml  [OUT OF SCOPE]
│   │   │   │   └── src
│   │   │   │       ├── db
│   │   │   │       │   ├── state.rs
│   │   │   │       │   └── wrap_state.rs
│   │   │   │       ├── db.rs
│   │   │   │       ├── guest
│   │   │   │       │   ├── env.rs
│   │   │   │       │   └── tests.rs  [OUT OF SCOPE]
│   │   │   │       ├── guest.rs
│   │   │   │       └── lib.rs
│   │   │   ├── host [OUT OF SCOPE]
│   │   │   ├── precompiles
│   │   │   │   ├── Cargo.toml  [OUT OF SCOPE]
│   │   │   │   └── src
│   │   │   │       ├── json.rs
│   │   │   │       ├── lib.rs
│   │   │   │       ├── regex.rs
│   │   │   │       ├── url_pattern.rs
│   │   │   │       ├── verify_and_parse.rs
│   │   │   │       └── verify_and_parse_email.rs
│   │   │   ├── seal
│   │   │   │   ├── Cargo.toml  [OUT OF SCOPE]
│   │   │   │   └── src
│   │   │   │       └── lib.rs
│   │   │   ├── server [OUT OF SCOPE]
│   │   │   └── server_lib [OUT OF SCOPE]
│   │   ├── chain
│   │   │   ├── block_trie
│   │   │   │   ├── Cargo.toml  [OUT OF SCOPE]
│   │   │   │   └── src
│   │   │   │       └── lib.rs
│   │   │   ├── client [OUT OF SCOPE]
│   │   │   ├── common
│   │   │   │   ├── Cargo.toml  [OUT OF SCOPE]
│   │   │   │   └── src
│   │   │   │       └── lib.rs
│   │   │   ├── db [OUT OF SCOPE]
│   │   │   ├── guest
│   │   │   │   ├── Cargo.toml  [OUT OF SCOPE]
│   │   │   │   └── src
│   │   │   │       └── lib.rs
│   │   │   ├── host [OUT OF SCOPE]
│   │   │   ├── mock_server [OUT OF SCOPE]
│   │   │   ├── server [OUT OF SCOPE]
│   │   │   ├── server_lib [OUT OF SCOPE]
│   │   │   ├── test_utils [OUT OF SCOPE]
│   │   │   └── worker [OUT OF SCOPE]
│   │   └── dns
│   │       └── server
│   │           ├── Cargo.toml  [OUT OF SCOPE]
│   │           ├── README.md
│   │           └── src
│   │               ├── config.rs
│   │               ├── main.rs
│   │               ├── server
│   │               │   ├── handlers
│   │               │   │   ├── dns_query
│   │               │   │   │   └── types.rs
│   │               │   │   └── dns_query.rs
│   │               │   └── handlers.rs
│   │               └── server.rs
│   ├── test_runner [OUT OF SCOPE]
│   ├── verifiable_dns
│   │   ├── Cargo.toml  [OUT OF SCOPE]
│   │   ├── assets  [OUT OF SCOPE]
│   │   └── src
│   │       ├── common
│   │       │   ├── record.rs
│   │       │   ├── test_utils.rs
│   │       │   ├── to_payload.rs
│   │       │   └── types.rs
│   │       ├── common.rs
│   │       ├── dns_over_https
│   │       │   ├── external_provider.rs
│   │       │   ├── provider.rs
│   │       │   └── types.rs
│   │       ├── dns_over_https.rs
│   │       ├── lib.rs
│   │       ├── verifiable_dns
│   │       │   ├── resolver
│   │       │   │   └── responses_validation.rs
│   │       │   ├── resolver.rs
│   │       │   ├── sign_record.rs
│   │       │   ├── signer.rs
│   │       │   └── time.rs
│   │       ├── verifiable_dns.rs
│   │       └── verifier
│   │           └── mod.rs
│   ├── version [OUT OF SCOPE]
│   ├── web_proof
│   │   ├── Cargo.toml  [OUT OF SCOPE]
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
│   │   └── testdata [OUT OF SCOPE]
│   └── zkvm-benchmarks [OUT OF SCOPE]
├── rust-toolchain.toml  [OUT OF SCOPE]
└── rustfmt.toml  [OUT OF SCOPE]
```