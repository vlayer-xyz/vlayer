.
├── .cargo
│   └── config.toml
├── **.dockerignore [??]**
├── **.git-branches.toml [OUT OF SCOPE]**
├── .**github [??]**
│   ├── **actionlint.yaml [OUT OF SCOPE]**
│   ├── actions
│   │   ├── **build-extension [OUT OF SCOPE]**
│   │   ├── build-guest-artifacts
│   │   │   └── action.yml
│   │   ├── contracts-prerequisites
│   │   │   └── action.yml
│   │   ├── darwin-prerequisites
│   │   │   └── action.yml
│   │   ├── **example-deploy [OUT OF SCOPE]**
│   │   ├── package-release-binaries
│   │   │   └── action.yml
│   │   ├── replace-guest-artifacts
│   │   │   └── action.yml
│   │   ├── rust-prerequisites
│   │   │   └── action.yml
│   │   └── **ts-prerequisites [OUT OF SCOPE]**
│   ├── **docs [OUT OF SCOPE]**
│   ├── **nix [??]**
│   │   ├── flake.lock
│   │   └── flake.nix
│   ├── **quarantine [OUT OF SCOPE]**
│   ├── **teams.yml [OUT OF SCOPE]**
│   └── **workflows [??]**
│       ├── build_docker.yaml
│       ├── **build_examples.yaml [OUT OF SCOPE]**
│       ├── **build_extension.yaml [OUT OF SCOPE]**
│       ├── **build_gas_benchmarks.yaml [OUT OF SCOPE]**
│       ├── build_guest_artifacts.yaml
│       ├── build_rust_release.yaml
│       ├── **build_sdk.yaml [OUT OF SCOPE]**
│       ├── **deploy_book.yaml [OUT OF SCOPE]**
│       ├── deploy_chain_services.yaml
│       ├── deploy_provers.yaml
│       ├── **deploy_simple_email_proof.yaml [OUT OF SCOPE]**
│       ├── **deploy_simple_web_proof.yaml [OUT OF SCOPE]**
│       ├── **lint_bash.yaml [OUT OF SCOPE]**
│       ├── **lint_contracts.yaml [OUT OF SCOPE]**
│       ├── **lint_json.yaml [OUT OF SCOPE]**
│       ├── **lint_rust.yaml [OUT OF SCOPE]**
│       ├── **lint_ts.yaml [OUT OF SCOPE]**
│       ├── **lint_workflows.yaml [OUT OF SCOPE]**
│       ├── **performance_rust_compilation.yaml [OUT OF SCOPE]**
│       ├── **pr_team_labeler.yaml [OUT OF SCOPE]**
│       ├── release.yaml
│       ├── **test_contracts.yaml [OUT OF SCOPE]**
│       ├── **test_e2e_devnet.yaml [OUT OF SCOPE]**
│       ├── **test_e2e_testnet.yaml [OUT OF SCOPE]**
│       ├── **test_e2e_web_apps.yaml [OUT OF SCOPE]**
│       ├── **test_e2e_web_flow.yaml [OUT OF SCOPE]**
│       ├── **test_js.yaml [OUT OF SCOPE]**
│       ├── test_release.yaml
│       ├── test_rust.yaml
│       └── test_vlayer.yaml
├── **.gitignore [OUT OF SCOPE]**
├── **.shellcheckrc [OUT OF SCOPE]**
├── **.solhint.json [OUT OF SCOPE]**
├── **.vscode [OUT OF SCOPE]**
├── **Cargo.lock [OUT OF SCOPE]**
├── Cargo.toml
├── **LICENSE [OUT OF SCOPE]**
├── **README.md [OUT OF SCOPE]**
├── **ansible [??]**
│   ├── README.md
│   ├── ansible.cfg
│   ├── chain_service.yml
│   ├── group_vars
│   │   ├── all.yml
│   │   ├── chain_services.yml
│   │   ├── provers.yml
│   │   └── verifiable_dns_services.yml
│   ├── host_vars
│   │   ├── prod_fake_chain_service.yml
│   │   ├── prod_fake_prover.yml
│   │   ├── prod_groth16_prover.yml
│   │   ├── prod_monitoring.yml
│   │   └── verifiable_dns_service.yml
│   ├── hosts.yml
│   ├── monitoring.yml
│   ├── prover.yml
│   ├── requirements.yml
│   ├── roles
│   │   ├── chain_server
│   │   │   ├── README.md
│   │   │   ├── defaults
│   │   │   │   └── main.yml
│   │   │   ├── files
│   │   │   │   ├── vlayer-logrotate.conf
│   │   │   │   └── vlayer-rsyslog.conf
│   │   │   ├── handlers
│   │   │   │   └── main.yml
│   │   │   ├── tasks
│   │   │   │   ├── logs.yml
│   │   │   │   └── main.yml
│   │   │   └── templates
│   │   │       └── vlayer-chain-server.service.j2
│   │   ├── chain_service_nginx
│   │   │   ├── README.md
│   │   │   ├── defaults
│   │   │   │   └── main.yml
│   │   │   ├── files
│   │   │   │   ├── chainservice.vlayer.xyz.key
│   │   │   │   └── chainservice.vlayer.xyz.pem
│   │   │   ├── handlers
│   │   │   │   └── main.yml
│   │   │   ├── tasks
│   │   │   │   └── main.yml
│   │   │   └── templates
│   │   │       └── vlayer-chainservice.conf.j2
│   │   ├── chain_worker
│   │   │   ├── README.md
│   │   │   ├── handlers
│   │   │   │   └── main.yml
│   │   │   ├── tasks
│   │   │   │   ├── logs.yml
│   │   │   │   └── main.yml
│   │   │   └── templates
│   │   │       ├── vlayer-chain-worker.service.j2
│   │   │       ├── vlayer-logrotate.conf
│   │   │       └── vlayer-rsyslog.conf
│   │   ├── monitoring
│   │   │   └── templates
│   │   │       └── prometheus.yml.j2
│   │   ├── prover
│   │   │   ├── README.md
│   │   │   ├── files
│   │   │   │   ├── vlayer-logrotate.conf
│   │   │   │   └── vlayer-rsyslog.conf
│   │   │   ├── handlers
│   │   │   │   └── main.yml
│   │   │   ├── tasks
│   │   │   │   ├── logs.yml
│   │   │   │   └── main.yml
│   │   │   └── templates
│   │   │       └── vlayer.service.j2
│   │   ├── prover_nginx
│   │   │   ├── README.md
│   │   │   ├── defaults
│   │   │   │   └── main.yml
│   │   │   ├── files
│   │   │   │   ├── prover.vlayer.xyz.key
│   │   │   │   └── prover.vlayer.xyz.pem
│   │   │   ├── handlers
│   │   │   │   └── main.yml
│   │   │   ├── tasks
│   │   │   │   └── main.yml
│   │   │   └── templates
│   │   │       └── vlayer-prover.conf.j2
│   │   ├── risc0
│   │   │   ├── README.md
│   │   │   └── tasks
│   │   │       └── main.yml
│   │   ├── verifiable_dns
│   │   │   ├── README.md
│   │   │   ├── files
│   │   │   │   ├── verifiable-dns-logrotate.conf
│   │   │   │   └── verifiable-dns-rsyslog.conf
│   │   │   ├── handlers
│   │   │   │   └── main.yml
│   │   │   ├── tasks
│   │   │   │   ├── logs.yml
│   │   │   │   ├── main.yml
│   │   │   │   └── prerequisites.yml
│   │   │   └── templates
│   │   │       └── verifiable-dns.service.j2
│   │   └── verifiable_dns_nginx
│   │       ├── README.md
│   │       ├── defaults
│   │       │   └── main.yml
│   │       ├── files
│   │       │   ├── dns.vlayer.xyz.key
│   │       │   └── dns.vlayer.xyz.pem
│   │       ├── handlers
│   │       │   └── main.yml
│   │       ├── tasks
│   │       │   └── main.yml
│   │       └── templates
│   │           └── verifiable-dns.conf.j2
│   └── verifiable_dns_service.yml
├── **bash [OUT OF SCOPE]**
│   ├── **CODESTYLE.md [OUT OF SCOPE]**
│   ├── build-ts-types.sh
│   ├── check-elf-id.sh
│   ├── check-llvm-clang.sh
│   ├── common.sh
│   ├── deploy-to-vercel.sh
│   ├── e2e
│   │   └── lib.sh
│   ├── e2e-test.sh
│   ├── e2e-web-apps-test.sh
│   ├── e2e-webproof-test.sh
│   ├── generate-extension-id.sh
│   ├── lib
│   │   ├── examples.sh
│   │   └── io.sh
│   ├── lint-fix.sh
│   ├── lint-json.sh
│   ├── lint-solidity-examples.sh
│   ├── lint-ts.sh
│   ├── **mock-imageid.sh [OUT OF SCOPE]**
│   ├── **pack-examples.sh [OUT OF SCOPE]**
│   ├── playwright-test.sh
│   ├── run-services.sh
│   ├── run-web-example.sh
│   ├── **run_services [OUT OF SCOPE]**
│   ├── **test-js-sdk-release.sh [OUT OF SCOPE]**
│   ├── test-release-local-prover.sh
│   ├── test-release-remote-prover.sh
│   ├── tsc-examples.sh
│   ├── tsc.sh
│   ├── version-test.sh
│   ├── vlayer-init-test.sh
│   ├── vlayer-test-examples.sh
│   └── vlayerup
│       ├── install
│       └── vlayerup
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
│       │   ├── ImageID.sol
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
│       │   │   ├── ImageIdRepository.sol
│       │   │   ├── ProofVerifierBase.sol
│       │   │   ├── ProofVerifierFactory.sol
│       │   │   └── ProofVerifierRouter.sol
│       │   └── testing
│       │       ├── VTest.sol
│       │       └── libraries
│       │           └── EmailTestUtils.sol
│       ├── test
│       │   ├── Seal.t.sol
│       │   ├── Verifier.t.sol
│       │   ├── helpers
│       │   │   ├── Groth16VerifierSelector.sol
│       │   │   └── TestHelpers.sol
│       │   ├── integration
│       │   │   ├── ProofVerifier.t.sol
│       │   │   └── VTest.t.sol
│       │   ├── proof_verifier
│       │   │   ├── FakeProofVerifier.t.sol
│       │   │   ├── Groth16ProofVerifier.t.sol
│       │   │   ├── ImageIdRepository.t.sol
│       │   │   ├── ProofVerifier.t.sol
│       │   │   ├── ProofVerifierFactory.t.sol
│       │   │   └── ProofVerifierRouter.t.sol
│       │   └── vlayer
│       │       ├── EmailProofLib.t.sol
│       │       ├── RegexLib.t.sol
│       │       ├── UrlPatternLib.t.sol
│       │       ├── WebLib.t.sol
│       │       └── WebProofLib.t.sol
│       └── **testdata [OUT OF SCOPE]**
├── docker
│   ├── anvil.yaml
│   ├── call_server
│   │   └── Dockerfile.nightly
│   ├── chain_server
│   │   └── Dockerfile.nightly
│   ├── chain_worker
│   │   └── Dockerfile.nightly
│   ├── docker-compose.devnet.yaml
│   ├── json-server
│   │   └── Dockerfile
│   ├── vlayer
│   │   └── Dockerfile.nightly
│   ├── web-proof
│   │   ├── docker-compose-release.yaml
│   │   └── notary-config
│   │       └── config.yaml
│   └── web.yaml
├── **examples [OUT OF SCOPE]**
├── **flake.lock [OUT OF SCOPE]**
├── **flake.nix [OUT OF SCOPE]**
├── **nix [OUT OF SCOPE]**
├── package.json
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
│   ├── host_utils
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── config.rs
│   │       ├── lib.rs
│   │       ├── prover.rs
│   │       └── proving.rs
│   ├── key_value
│   │   ├── Cargo.toml
│   │   ├── benches
│   │   │   └── mdbx_insert.rs
│   │   └── src
│   │       ├── in_memory.rs
│   │       ├── lib.rs
│   │       ├── mdbx
│   │       │   └── tests.rs
│   │       └── mdbx.rs
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
│   │   │   │   │   ├── tests.rs
│   │   │   │   │   └── utils.rs
│   │   │   │   ├── insert.rs
│   │   │   │   ├── rlp.rs
│   │   │   │   └── size.rs
│   │   │   ├── node.rs
│   │   │   ├── node_ref.rs
│   │   │   ├── path.rs
│   │   │   ├── trie
│   │   │   │   ├── tests
│   │   │   │   │   ├── from_rlp_nodes.rs
│   │   │   │   │   ├── get.rs
│   │   │   │   │   ├── hashable.rs
│   │   │   │   │   └── insert.rs
│   │   │   │   ├── tests.rs
│   │   │   │   └── utils.rs
│   │   │   ├── trie.rs
│   │   │   └── utils.rs
│   │   └── **tests [OUT OF SCOPE]**
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
│   │   └── **testdata [OUT OF SCOPE]**
│   ├── range
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── lib.rs
│   │       ├── non_empty_range.rs
│   │       └── range.rs
│   ├── server_utils
│   │   ├── Cargo.toml
│   │   └── src
│   │       ├── field_validation.rs
│   │       ├── json_rpc.rs
│   │       ├── layers
│   │       │   ├── request_id.rs
│   │       │   └── trace.rs
│   │       ├── layers.rs
│   │       ├── lib.rs
│   │       ├── proof_mode.rs
│   │       ├── rpc.rs
│   │       └── **test_utils.rs [OUT OF SCOPE]**
│   ├── services
│   │   ├── call
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
│   │   │   │       │   ├── tests
│   │   │   │       │   │   ├── chain_proof.rs
│   │   │   │       │   │   └── time_travel.rs
│   │   │   │       │   ├── tests.rs
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
│   │   │   │       │   └── tests.rs
│   │   │   │       ├── guest.rs
│   │   │   │       └── lib.rs
│   │   │   ├── host
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── src
│   │   │   │   │   ├── db
│   │   │   │   │   │   ├── proof.rs
│   │   │   │   │   │   └── provider.rs
│   │   │   │   │   ├── db.rs
│   │   │   │   │   ├── evm_env
│   │   │   │   │   │   └── factory.rs
│   │   │   │   │   ├── evm_env.rs
│   │   │   │   │   ├── host
│   │   │   │   │   │   ├── builder.rs
│   │   │   │   │   │   ├── config.rs
│   │   │   │   │   │   ├── error.rs
│   │   │   │   │   │   ├── prover.rs
│   │   │   │   │   │   ├── **tests [OUT OF SCOPE]**
│   │   │   │   │   ├── host.rs
│   │   │   │   │   ├── into_input.rs
│   │   │   │   │   ├── lib.rs
│   │   │   │   │   ├── test_harness
│   │   │   │   │   │   ├── contracts.rs
│   │   │   │   │   │   ├── rpc.rs
│   │   │   │   │   │   └── types.rs
│   │   │   │   │   └── test_harness.rs
│   │   │   │   └── **test_data [OUT OF SCOPE]**
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
│   │   │   ├── server
│   │   │   │   ├── Cargo.toml
│   │   │   │   └── src
│   │   │   │       ├── main.rs
│   │   │   │       └── version.rs
│   │   │   └── server_lib
│   │   │       ├── Cargo.toml
│   │   │       ├── **README.md [OUT OF SCOPE]**
│   │   │       ├── src
│   │   │       │   ├── chain_proof.rs
│   │   │       │   ├── config.rs
│   │   │       │   ├── gas_meter.rs
│   │   │       │   ├── handlers
│   │   │       │   │   ├── v_call
│   │   │       │   │   │   └── types.rs
│   │   │       │   │   ├── v_call.rs
│   │   │       │   │   ├── v_get_proof_receipt
│   │   │       │   │   │   └── types.rs
│   │   │       │   │   ├── v_get_proof_receipt.rs
│   │   │       │   │   └── v_versions.rs
│   │   │       │   ├── handlers.rs
│   │   │       │   ├── lib.rs
│   │   │       │   ├── metrics.rs
│   │   │       │   ├── preflight.rs
│   │   │       │   ├── proof.rs
│   │   │       │   ├── proving.rs
│   │   │       │   ├── ser.rs
│   │   │       │   └── server.rs
│   │   │       ├── **testdata [OUT OF SCOPE]**
│   │   │       └── **tests [OUT OF SCOPE]**
│   │   ├── chain
│   │   │   ├── block_trie
│   │   │   │   ├── Cargo.toml
│   │   │   │   └── src
│   │   │   │       └── lib.rs
│   │   │   ├── client
│   │   │   │   ├── Cargo.toml
│   │   │   │   └── src
│   │   │   │       ├── fake.rs
│   │   │   │       ├── lib.rs
│   │   │   │       ├── rpc.rs
│   │   │   │       └── **tests.rs [OUT OF SCOPE]**}
│   │   │   ├── common
│   │   │   │   ├── Cargo.toml
│   │   │   │   └── src
│   │   │   │       └── lib.rs
│   │   │   ├── db
│   │   │   │   ├── Cargo.toml
│   │   │   │   └── src
│   │   │   │       ├── chain_trie.rs
│   │   │   │       ├── db_node.rs
│   │   │   │       ├── error.rs
│   │   │   │       ├── lib.rs
│   │   │   │       ├── proof_builder.rs
│   │   │   │       └── **tests.rs [OUT OF SCOPE]**
│   │   │   ├── guest
│   │   │   │   ├── Cargo.toml
│   │   │   │   └── src
│   │   │   │       └── lib.rs
│   │   │   ├── host
│   │   │   │   ├── Cargo.toml
│   │   │   │   └── src
│   │   │   │       ├── host
│   │   │   │       │   ├── block_fetcher.rs
│   │   │   │       │   ├── config.rs
│   │   │   │       │   ├── error.rs
│   │   │   │       │   ├── prover.rs
│   │   │   │       │   └── strategy.rs
│   │   │   │       ├── host.rs
│   │   │   │       └── lib.rs
│   │   │   ├── mock_server
│   │   │   │   ├── Cargo.toml
│   │   │   │   └── src
│   │   │   │       └── lib.rs
│   │   │   ├── server
│   │   │   │   ├── Cargo.toml
│   │   │   │   └── src
│   │   │   │       └── main.rs
│   │   │   ├── server_lib
│   │   │   │   ├── Cargo.toml
│   │   │   │   ├── src
│   │   │   │   │   ├── config.rs
│   │   │   │   │   ├── error.rs
│   │   │   │   │   ├── handlers
│   │   │   │   │   │   ├── chain_proof.rs
│   │   │   │   │   │   └── status.rs
│   │   │   │   │   ├── handlers.rs
│   │   │   │   │   ├── lib.rs
│   │   │   │   │   └── server.rs
│   │   │   │   └── **tests [OUT OF SCOPE]**
│   │   │   ├── **test_utils [OUT OF SCOPE]**
│   │   │   └── worker
│   │   │       ├── Cargo.toml
│   │   │       └── src
│   │   │           ├── main.rs
│   │   │           └── retry.rs
│   │   └── **dns [OUT OF SCOPE]**
│   ├── **test_runner [OUT OF SCOPE]**
│   ├── verifiable_dns
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
│   ├── version
│   │   ├── Cargo.toml
│   │   ├── build.rs
│   │   └── src
│   │       └── lib.rs
│   ├── web_proof
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
├── rust-toolchain.toml
└── **rustfmt.toml [OUT OF SCOPE]**
