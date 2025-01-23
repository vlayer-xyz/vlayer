.
├── .cargo
│   └── config.toml
├── .dockerignore
├── .git-branches.toml
├── .github
│   ├── actionlint.yaml
│   ├── actions
│   │   ├── build-extension
│   │   │   └── action.yml
│   │   ├── build-guest-artifacts
│   │   │   └── action.yml
│   │   ├── contracts-prerequisites
│   │   │   └── action.yml
│   │   ├── darwin-prerequisites
│   │   │   └── action.yml
│   │   ├── example-deploy
│   │   │   └── action.yaml
│   │   ├── package-release-binaries
│   │   │   └── action.yml
│   │   ├── replace-guest-artifacts
│   │   │   └── action.yml
│   │   ├── rust-prerequisites
│   │   │   └── action.yml
│   │   └── ts-prerequisites
│   │       └── action.yml
│   ├── docs
│   │   └── README.md
│   ├── nix
│   │   ├── flake.lock
│   │   └── flake.nix
│   ├── quarantine
│   │   └── lint_ansible.yaml
│   ├── teams.yml
│   └── workflows
│       ├── build_docker.yaml
│       ├── build_examples.yaml
│       ├── build_extension.yaml
│       ├── build_gas_benchmarks.yaml
│       ├── build_guest_artifacts.yaml
│       ├── build_rust_release.yaml
│       ├── build_sdk.yaml
│       ├── deploy_book.yaml
│       ├── deploy_chain_services.yaml
│       ├── deploy_provers.yaml
│       ├── deploy_simple_email_proof.yaml
│       ├── deploy_simple_web_proof.yaml
│       ├── lint_bash.yaml
│       ├── lint_contracts.yaml
│       ├── lint_json.yaml
│       ├── lint_rust.yaml
│       ├── lint_ts.yaml
│       ├── lint_workflows.yaml
│       ├── performance_rust_compilation.yaml
│       ├── pr_team_labeler.yaml
│       ├── release.yaml
│       ├── test_contracts.yaml
│       ├── test_e2e_devnet.yaml
│       ├── test_e2e_testnet.yaml
│       ├── test_e2e_web_apps.yaml
│       ├── test_e2e_web_flow.yaml
│       ├── test_js.yaml
│       ├── test_release.yaml
│       ├── test_rust.yaml
│       └── test_vlayer.yaml
├── .gitignore
├── .shellcheckrc
├── .solhint.json
├── .vscode
│   ├── extensions.json
│   ├── ltex.dictionary.en-US.txt
│   └── settings.json
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
├── ansible
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
├── bash
│   ├── CODESTYLE.md
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
│   ├── mock-imageid.sh
│   ├── pack-examples.sh
│   ├── playwright-test.sh
│   ├── run-services.sh
│   ├── run-web-example.sh
│   ├── run_services
│   │   ├── chain_worker.sh
│   │   ├── cleanup.sh
│   │   ├── config.sh
│   │   └── lib.sh
│   ├── test-js-sdk-release.sh
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
├── book
│   ├── book.toml
│   ├── mermaid-init.js
│   ├── mermaid.min.js
│   ├── src
│   │   ├── SUMMARY.md
│   │   ├── advanced
│   │   │   ├── dev-and-production.md
│   │   │   ├── prover-global-variables.md
│   │   │   ├── prover.md
│   │   │   ├── tests.md
│   │   │   └── verifier.md
│   │   ├── api
│   │   │   └── auth.js
│   │   ├── appendix
│   │   │   ├── api.md
│   │   │   ├── architecture
│   │   │   │   ├── block_proof
│   │   │   │   │   ├── canonicity.md
│   │   │   │   │   └── coherence.md
│   │   │   │   ├── block_proof.md
│   │   │   │   ├── multi.md
│   │   │   │   ├── overview.md
│   │   │   │   ├── prover.md
│   │   │   │   ├── releasing.md
│   │   │   │   └── solidity.md
│   │   │   ├── contributing
│   │   │   │   ├── book.md
│   │   │   │   ├── extension.md
│   │   │   │   ├── javascript.md
│   │   │   │   ├── overview.md
│   │   │   │   └── rust.md
│   │   │   └── proof_composition.md
│   │   ├── coming.html
│   │   ├── features
│   │   │   ├── email.md
│   │   │   ├── json-and-regex.md
│   │   │   ├── teleport.md
│   │   │   ├── time-travel.md
│   │   │   └── web.md
│   │   ├── getting-started
│   │   │   ├── first-steps.md
│   │   │   ├── how-it-works.md
│   │   │   └── installation.md
│   │   ├── images
│   │   │   ├── architecture
│   │   │   │   ├── block_proof
│   │   │   │   │   ├── chain_proof.png
│   │   │   │   │   ├── naive_chain_proof.png
│   │   │   │   │   └── on-off-chain.png
│   │   │   │   ├── guest-output.png
│   │   │   │   ├── mmr.png
│   │   │   │   ├── overview.png
│   │   │   │   ├── prover-verifier-data-ecoding.png
│   │   │   │   ├── prover.png
│   │   │   │   └── releasing.png
│   │   │   ├── cover.jpg
│   │   │   ├── offchain-execution.png
│   │   │   └── vlayer-browser-extension.jpg
│   │   ├── introduction.md
│   │   ├── javascript
│   │   │   ├── email-proofs.md
│   │   │   ├── javascript.md
│   │   │   ├── react-hooks.md
│   │   │   └── web-proofs.md
│   │   ├── middleware.js
│   │   ├── package-lock.json
│   │   ├── package.json
│   │   └── static
│   │       ├── appendixMenuV3.js
│   │       ├── solidity.min.js
│   │       └── vlayer-eml-1.gif
│   └── theme
│       ├── asidev2.css
│       ├── favicon.png
│       ├── favicon.svg
│       ├── head.hbs
│       ├── menuv3.css
│       ├── menuv3.js
│       ├── tabs.css
│       └── tabs.js
├── bun.lockb
├── clippy.toml
├── contracts
│   ├── fixtures
│   │   ├── README.md
│   │   ├── foundry.toml
│   │   ├── package.json
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
│       ├── package.json
│       ├── remappings.txt
│       ├── soldeer.lock
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
│       └── testdata
│           ├── verify_vlayer.eml
│           ├── web_proof.json
│           ├── web_proof_invalid_notary_pub_key.json
│           └── web_proof_missing_part.json
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
├── flake.lock
├── flake.nix
├── nix
│   └── risc0.nix
├── package.json
├── packages
│   ├── browser-extension
│   │   ├── assets
│   │   │   └── separator.svg
│   │   ├── package.json
│   │   ├── public
│   │   │   ├── bottomlogo.svg
│   │   │   ├── box.svg
│   │   │   ├── fonts
│   │   │   │   ├── Sora-Regular.ttf
│   │   │   │   └── roobert.woff2
│   │   │   ├── icon
│   │   │   │   ├── 128px.png
│   │   │   │   ├── 16px.png
│   │   │   │   ├── 32px.png
│   │   │   │   ├── 48px.png
│   │   │   │   └── 96px.png
│   │   │   ├── livebuoy.svg
│   │   │   ├── logo.png
│   │   │   ├── separator.svg
│   │   │   └── vlayer_logo.svg
│   │   ├── src
│   │   │   ├── background.test.ts
│   │   │   ├── background.ts
│   │   │   ├── components
│   │   │   │   ├── atoms
│   │   │   │   │   ├── Button.module.css
│   │   │   │   │   ├── Button.tsx
│   │   │   │   │   ├── Circle.module.css
│   │   │   │   │   ├── Circle.tsx
│   │   │   │   │   ├── Separator.module.css
│   │   │   │   │   ├── Separator.tsx
│   │   │   │   │   ├── VlayerBottomLogo.tsx
│   │   │   │   │   └── index.ts
│   │   │   │   ├── framer.ts
│   │   │   │   ├── index.ts
│   │   │   │   ├── molecules
│   │   │   │   │   ├── AnimationContainer.tsx
│   │   │   │   │   ├── EmptyFlow
│   │   │   │   │   │   ├── Card.tsx
│   │   │   │   │   │   ├── CardContent.module.css
│   │   │   │   │   │   ├── CardContent.tsx
│   │   │   │   │   │   ├── index.ts
│   │   │   │   │   │   └── types.ts
│   │   │   │   │   ├── Step.module.css
│   │   │   │   │   ├── Step.tsx
│   │   │   │   │   ├── StepActions
│   │   │   │   │   │   ├── ExpectUrl
│   │   │   │   │   │   │   ├── ExpectUrlStepActions.tsx
│   │   │   │   │   │   │   └── index.ts
│   │   │   │   │   │   ├── Notarize
│   │   │   │   │   │   │   ├── FinishCallout.tsx
│   │   │   │   │   │   │   ├── GenerateProofButton.tsx
│   │   │   │   │   │   │   ├── NotarizeStepActions.hooks.tsx
│   │   │   │   │   │   │   ├── NotarizeStepActions.test.tsx
│   │   │   │   │   │   │   ├── NotarizeStepActions.tsx
│   │   │   │   │   │   │   ├── ProvingProgress.hooks.ts
│   │   │   │   │   │   │   ├── ProvingProgress.test.tsx
│   │   │   │   │   │   │   ├── ProvingProgress.tsx
│   │   │   │   │   │   │   ├── RedirectCallout.test.tsx
│   │   │   │   │   │   │   ├── RedirectCallout.tsx
│   │   │   │   │   │   │   ├── index.ts
│   │   │   │   │   │   │   └── types.ts
│   │   │   │   │   │   ├── StartPage
│   │   │   │   │   │   │   ├── StartPageStepActions.tsx
│   │   │   │   │   │   │   └── index.ts
│   │   │   │   │   │   ├── StepActions.tsx
│   │   │   │   │   │   └── index.ts
│   │   │   │   │   ├── StepCircle
│   │   │   │   │   │   ├── CompletedStepCircle.tsx
│   │   │   │   │   │   ├── CurrentStepCircle.tsx
│   │   │   │   │   │   ├── FurtherStepCircle.tsx
│   │   │   │   │   │   ├── StepCircle.tsx
│   │   │   │   │   │   └── index.ts
│   │   │   │   │   └── index.ts
│   │   │   │   ├── organisms
│   │   │   │   │   ├── HelpSection.module.css
│   │   │   │   │   ├── HelpSection.tsx
│   │   │   │   │   ├── Steps.tsx
│   │   │   │   │   └── index.ts
│   │   │   │   └── pages
│   │   │   │       ├── SidePanel.module.css
│   │   │   │       ├── SidePanel.tsx
│   │   │   │       ├── SidePanelContent.tsx
│   │   │   │       ├── StepPanelContent.test.tsx
│   │   │   │       └── index.ts
│   │   │   ├── constants
│   │   │   │   ├── defaults.ts
│   │   │   │   ├── index.ts
│   │   │   │   └── step.ts
│   │   │   ├── hooks
│   │   │   │   ├── tlsnProve
│   │   │   │   │   ├── redaction
│   │   │   │   │   │   ├── body
│   │   │   │   │   │   │   ├── index.ts
│   │   │   │   │   │   │   ├── tlsn.response.body.ranges.test.ts
│   │   │   │   │   │   │   └── tlsn.response.body.ranges.ts
│   │   │   │   │   │   ├── headers
│   │   │   │   │   │   │   ├── index.ts
│   │   │   │   │   │   │   ├── tlsn.headers.ranges.test.ts
│   │   │   │   │   │   │   └── tlsn.headers.ranges.ts
│   │   │   │   │   │   ├── query
│   │   │   │   │   │   │   ├── index.ts
│   │   │   │   │   │   │   ├── tlsn.request.query.ranges.test.ts
│   │   │   │   │   │   │   └── tlsn.request.query.ranges.ts
│   │   │   │   │   │   ├── redact.test.ts
│   │   │   │   │   │   ├── redact.ts
│   │   │   │   │   │   ├── tlsn.ranges.test.fixtures.ts
│   │   │   │   │   │   ├── tlsn.request.ranges.ts
│   │   │   │   │   │   ├── tlsn.response.ranges.ts
│   │   │   │   │   │   ├── types.ts
│   │   │   │   │   │   └── utils
│   │   │   │   │   │       ├── encodeString
│   │   │   │   │   │       │   ├── EncodedString.test.ts
│   │   │   │   │   │       │   ├── EncodedString.ts
│   │   │   │   │   │       │   ├── Encoding.test.ts
│   │   │   │   │   │       │   ├── Encoding.ts
│   │   │   │   │   │       │   ├── index.ts
│   │   │   │   │   │       │   ├── parseHttpMessage.test.ts
│   │   │   │   │   │       │   └── parseHttpMessage.ts
│   │   │   │   │   │       ├── error.ts
│   │   │   │   │   │       ├── getStringPaths
│   │   │   │   │   │       │   ├── getStringPaths.test.ts
│   │   │   │   │   │       │   ├── getStringPaths.ts
│   │   │   │   │   │       │   └── index.ts
│   │   │   │   │   │       ├── index.ts
│   │   │   │   │   │       └── queryParams
│   │   │   │   │   │           └── index.ts
│   │   │   │   │   ├── tlsnProve.ts
│   │   │   │   │   └── tlsnWorker.ts
│   │   │   │   ├── useBrowsingHistory.ts
│   │   │   │   ├── useProvenUrl.test.ts
│   │   │   │   ├── useProvenUrl.ts
│   │   │   │   ├── useProvingSessionConfig.ts
│   │   │   │   ├── useSteps.test.data.ts
│   │   │   │   ├── useSteps.test.helpers.ts
│   │   │   │   ├── useSteps.test.ts
│   │   │   │   ├── useSteps.ts
│   │   │   │   ├── useTlsnProver.tsx
│   │   │   │   ├── useTrackHistory.ts
│   │   │   │   ├── useZkProvingState.test.ts
│   │   │   │   └── useZkProvingState.ts
│   │   │   ├── lib
│   │   │   │   ├── formatTlsnHeaders.ts
│   │   │   │   └── sendMessageToServiceWorker.ts
│   │   │   ├── manifest.json
│   │   │   ├── state
│   │   │   │   ├── history.ts
│   │   │   │   ├── store.ts
│   │   │   │   ├── webProverSessionContext.ts
│   │   │   │   └── zkProvingStatusStore.ts
│   │   │   ├── templates
│   │   │   │   └── sidepanel
│   │   │   │       ├── sidepanel.css
│   │   │   │       ├── sidepanel.html
│   │   │   │       └── sidepanel.tsx
│   │   │   ├── vite-env.d.ts
│   │   │   └── web-proof-commons
│   │   ├── tsconfig.json
│   │   ├── vite.config.ts
│   │   ├── vitest
│   │   │   ├── custom.matchers.ts
│   │   │   └── setup.ts
│   │   └── vitest.d.ts
│   ├── bun.lockb
│   ├── eslint.config.js
│   ├── extension-hooks
│   │   ├── README.md
│   │   ├── package.json
│   │   ├── src
│   │   │   ├── constants.ts
│   │   │   ├── createStorageHook.ts
│   │   │   ├── index.ts
│   │   │   ├── storage.test.ts
│   │   │   ├── useLocalStorage.ts
│   │   │   ├── useSessionStorage.ts
│   │   │   └── useSyncStorage.ts
│   │   ├── storage.setup.ts
│   │   ├── tsconfig.json
│   │   └── vitest.config.ts
│   ├── gas-benchmarks
│   │   ├── README.md
│   │   ├── eslint.config.ts
│   │   ├── package.json
│   │   ├── src
│   │   │   ├── bench.ts
│   │   │   ├── benches
│   │   │   │   ├── arith_ops.ts
│   │   │   │   ├── noop.ts
│   │   │   │   └── noop_with_calldata.ts
│   │   │   └── types.ts
│   │   └── tsconfig.json
│   ├── package.json
│   ├── playwright-tests
│   │   ├── config.ts
│   │   ├── email.e2e.spec.ts
│   │   ├── fixtures
│   │   │   └── verify_vlayer.eml
│   │   ├── helpers.ts
│   │   ├── sidepanel.e2e.spec.ts
│   │   └── tsconfig.json
│   ├── playwright.config.ts
│   ├── sdk
│   │   ├── CHANGELOG.md
│   │   ├── README.md
│   │   ├── eslint.config.ts
│   │   ├── package.json
│   │   ├── src
│   │   │   ├── api
│   │   │   │   ├── email
│   │   │   │   │   ├── dnsResolver.test.ts
│   │   │   │   │   ├── dnsResolver.ts
│   │   │   │   │   ├── parseEmail.test.ts
│   │   │   │   │   ├── parseEmail.ts
│   │   │   │   │   ├── preverify.test.ts
│   │   │   │   │   ├── preverify.ts
│   │   │   │   │   └── testdata
│   │   │   │   │       ├── test_email.txt
│   │   │   │   │       ├── test_email_multiple_dkims.txt
│   │   │   │   │       ├── test_email_subdomain.txt
│   │   │   │   │       └── test_email_unknown_domain.txt
│   │   │   │   ├── lib
│   │   │   │   │   ├── client.test.ts
│   │   │   │   │   ├── client.ts
│   │   │   │   │   ├── errors.ts
│   │   │   │   │   └── types
│   │   │   │   │       ├── ethereum.ts
│   │   │   │   │       ├── index.ts
│   │   │   │   │       ├── viem.ts
│   │   │   │   │       ├── vlayer.ts
│   │   │   │   │       └── webProofProvider.ts
│   │   │   │   ├── prover.ts
│   │   │   │   ├── utils
│   │   │   │   │   ├── prefixAllButNthSubstring.test.ts
│   │   │   │   │   ├── prefixAllButNthSubstring.ts
│   │   │   │   │   ├── versions.test.ts
│   │   │   │   │   └── versions.ts
│   │   │   │   ├── v_call.ts
│   │   │   │   ├── v_getProofReceipt.ts
│   │   │   │   ├── v_versions.ts
│   │   │   │   └── webProof
│   │   │   │       ├── createWebProofRequest.ts
│   │   │   │       ├── index.ts
│   │   │   │       ├── providers
│   │   │   │       │   ├── extension.test.ts
│   │   │   │       │   ├── extension.ts
│   │   │   │       │   └── index.ts
│   │   │   │       ├── redactionFunctions.test.ts
│   │   │   │       ├── redactionFunctions.ts
│   │   │   │       └── steps
│   │   │   │           ├── expectUrl.ts
│   │   │   │           ├── index.ts
│   │   │   │           ├── notarize.ts
│   │   │   │           └── startPage.ts
│   │   │   ├── config
│   │   │   │   ├── createContext.ts
│   │   │   │   ├── deploy.ts
│   │   │   │   ├── getChainConfirmations.ts
│   │   │   │   ├── getConfig.ts
│   │   │   │   ├── index.ts
│   │   │   │   ├── types.ts
│   │   │   │   └── writeEnvVariables.ts
│   │   │   ├── index.ts
│   │   │   ├── testHelpers
│   │   │   │   └── readFile.ts
│   │   │   └── web-proof-commons
│   │   ├── tsconfig.base.json
│   │   ├── tsconfig.build.json
│   │   ├── tsconfig.json
│   │   ├── vite.config.ts
│   │   └── vitest.setup.ts
│   ├── sdk-hooks
│   │   ├── README.md
│   │   ├── bun.lockb
│   │   ├── eslint.config.ts
│   │   ├── package.json
│   │   ├── src
│   │   │   ├── context.test.tsx
│   │   │   ├── context.tsx
│   │   │   ├── defaults.ts
│   │   │   ├── index.ts
│   │   │   ├── interface.test.ts
│   │   │   ├── types.ts
│   │   │   ├── useCallProver
│   │   │   │   ├── useCallProver.test.ts
│   │   │   │   └── useCallProver.ts
│   │   │   ├── useWaitForProvingResult
│   │   │   │   ├── useWaitForProvingResult.test.ts
│   │   │   │   └── useWaitForProvingResult.ts
│   │   │   └── useWebproof
│   │   │       ├── extension.mock.ts
│   │   │       ├── useWebProof.test.tsx
│   │   │       └── useWebProof.ts
│   │   ├── tsconfig.base.json
│   │   ├── tsconfig.build.json
│   │   ├── tsconfig.json
│   │   └── vitest.config.ts
│   ├── test-json-server
│   │   ├── README.md
│   │   ├── certs
│   │   │   ├── lotr-api_online.crt
│   │   │   └── lotr-api_online.key
│   │   ├── package.json
│   │   ├── src
│   │   │   └── index.ts
│   │   └── tsconfig.json
│   ├── test-web-app
│   │   ├── deploy.ts
│   │   ├── index.html
│   │   ├── package.json
│   │   ├── src
│   │   │   ├── Dapp.tsx
│   │   │   ├── DappProveWeb.tsx
│   │   │   ├── Dashboard.tsx
│   │   │   ├── Email.tsx
│   │   │   ├── Login.tsx
│   │   │   ├── Profile.tsx
│   │   │   ├── main.css
│   │   │   └── main.tsx
│   │   ├── tsconfig.json
│   │   ├── vite-env.d.ts
│   │   └── vite.config.ts
│   ├── tsconfig.base.json
│   ├── tsconfig.json
│   ├── web-components
│   │   ├── README.md
│   │   ├── package.json
│   │   ├── src
│   │   │   ├── components
│   │   │   │   ├── index.ts
│   │   │   │   ├── theme.css
│   │   │   │   └── theme.tsx
│   │   │   └── index.ts
│   │   └── tsconfig.json
│   └── web-proof-commons
│       ├── index.ts
│       ├── types
│       │   └── message.ts
│       └── utils.ts
├── rust
│   ├── block_header
│   │   ├── Cargo.toml
│   │   ├── src
│   │   │   ├── casting_utils.rs
│   │   │   ├── eth.rs
│   │   │   ├── forge.rs
│   │   │   └── lib.rs
│   │   └── testdata
│   │       ├── invalid_header.json
│   │       └── mainnet_rpc_cache.json
│   ├── chain
│   │   ├── Cargo.toml
│   │   ├── chain_specs.toml
│   │   └── src
│   │       ├── config.rs
│   │       ├── eip1559.rs
│   │       ├── fork.rs
│   │       ├── lib.rs
│   │       └── spec.rs
│   ├── cli
│   │   ├── Cargo.toml
│   │   ├── deny.toml
│   │   ├── src
│   │   │   ├── commands
│   │   │   │   ├── args.rs
│   │   │   │   ├── common
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   └── soldeer.rs
│   │   │   │   ├── init.rs
│   │   │   │   ├── test.rs
│   │   │   │   └── update.rs
│   │   │   ├── commands.rs
│   │   │   ├── errors.rs
│   │   │   ├── main.rs
│   │   │   ├── test_utils.rs
│   │   │   └── utils
│   │   │       ├── mod.rs
│   │   │       ├── parse_toml.rs
│   │   │       └── path.rs
│   │   └── test_static
│   │       └── contracts.tar.gz
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
│   │   ├── README.md
│   │   ├── clippy.toml
│   │   ├── images
│   │   │   ├── into_branch_0.png
│   │   │   ├── into_branch_1.png
│   │   │   ├── into_branch_2.png
│   │   │   ├── into_extension_0.png
│   │   │   ├── into_extension_1.png
│   │   │   ├── into_extension_2.png
│   │   │   ├── into_leaf_0.png
│   │   │   ├── into_leaf_1.png
│   │   │   └── into_leaf_2.png
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
│   │   └── tests
│   │       ├── consistent_with_alloy_trie.rs
│   │       ├── insert_and_get.rs
│   │       ├── insert_is_commutative.rs
│   │       ├── parse_eth_get_proof.rs
│   │       └── utils.rs
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
│   │   └── testdata
│   │       └── cache.json
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
│   │       └── test_utils.rs
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
│   │   │   │   │   │   ├── tests
│   │   │   │   │   │   │   ├── number_of_rpc_calls.rs
│   │   │   │   │   │   │   ├── preflight.rs
│   │   │   │   │   │   │   ├── snapshots
│   │   │   │   │   │   │   │   ├── call_host__host__tests__number_of_rpc_calls__time_travel.snap
│   │   │   │   │   │   │   │   └── call_host__host__tests__number_of_rpc_calls__usdt_erc20_balance_of.snap
│   │   │   │   │   │   │   └── with_guest.rs
│   │   │   │   │   │   └── tests.rs
│   │   │   │   │   ├── host.rs
│   │   │   │   │   ├── into_input.rs
│   │   │   │   │   ├── lib.rs
│   │   │   │   │   ├── test_harness
│   │   │   │   │   │   ├── contracts.rs
│   │   │   │   │   │   ├── rpc.rs
│   │   │   │   │   │   └── types.rs
│   │   │   │   │   └── test_harness.rs
│   │   │   │   └── test_data
│   │   │   │       ├── simple
│   │   │   │       │   └── op_sepolia.json
│   │   │   │       ├── teleport
│   │   │   │       │   ├── anvil.json
│   │   │   │       │   └── mainnet.json
│   │   │   │       ├── teleport_v2
│   │   │   │       │   ├── anvil.json
│   │   │   │       │   └── op_anvil.json
│   │   │   │       ├── time_travel
│   │   │   │       │   └── op_sepolia.json
│   │   │   │       ├── uniswap_factory_owner
│   │   │   │       │   └── mainnet.json
│   │   │   │       ├── usdt_erc20_balance_of
│   │   │   │       │   ├── mainnet.json
│   │   │   │       │   └── op_mainnet.json
│   │   │   │       ├── view_blockhash
│   │   │   │       │   └── sepolia.json
│   │   │   │       ├── view_call_eoa
│   │   │   │       │   └── sepolia.json
│   │   │   │       ├── view_chainid
│   │   │   │       │   └── sepolia.json
│   │   │   │       ├── view_eoa_account
│   │   │   │       │   └── sepolia.json
│   │   │   │       ├── view_multi_contract_calls
│   │   │   │       │   └── sepolia.json
│   │   │   │       ├── view_nonexistent_account
│   │   │   │       │   └── sepolia.json
│   │   │   │       └── view_precompile
│   │   │   │           └── sepolia.json
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
│   │   │       ├── README.md
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
│   │   │       ├── testdata
│   │   │       │   └── ExampleProver.json
│   │   │       └── tests
│   │   │           ├── integration_tests.rs
│   │   │           └── test_helpers
│   │   │               └── mod.rs
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
│   │   │   │       └── tests.rs
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
│   │   │   │       └── tests.rs
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
│   │   │   │   └── tests
│   │   │   │       └── integration_tests.rs
│   │   │   ├── test_utils
│   │   │   │   ├── Cargo.toml
│   │   │   │   └── src
│   │   │   │       └── lib.rs
│   │   │   └── worker
│   │   │       ├── Cargo.toml
│   │   │       └── src
│   │   │           ├── main.rs
│   │   │           └── retry.rs
│   │   └── dns
│   │       └── server
│   │           ├── Cargo.toml
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
│   ├── test_runner
│   │   ├── Cargo.toml
│   │   ├── src
│   │   │   ├── cheatcode_inspector.rs
│   │   │   ├── cheatcodes.rs
│   │   │   ├── composite_inspector.rs
│   │   │   ├── forked
│   │   │   │   ├── cli.rs
│   │   │   │   ├── filter.rs
│   │   │   │   ├── install.rs
│   │   │   │   ├── multi_runner.rs
│   │   │   │   ├── progress.rs
│   │   │   │   ├── runner.rs
│   │   │   │   ├── summary.rs
│   │   │   │   └── test_executor.rs
│   │   │   ├── forked.rs
│   │   │   ├── init_global.rs
│   │   │   ├── lib.rs
│   │   │   ├── preverify_email.rs
│   │   │   ├── proof.rs
│   │   │   └── providers
│   │   │       ├── mod.rs
│   │   │       ├── pending_state_provider.rs
│   │   │       └── test_provider.rs
│   │   └── testdata
│   │       └── dumped_evm_state.json
│   ├── verifiable_dns
│   │   ├── Cargo.toml
│   │   ├── assets
│   │   │   ├── private_key.pem
│   │   │   └── public_key.pem
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
│   │   └── testdata
│   │       ├── chunked_response.txt
│   │       ├── many_headers_sent_request.txt
│   │       ├── no_body_response.txt
│   │       ├── no_headers_response.txt
│   │       ├── received_response.txt
│   │       ├── redacted_received_response.txt
│   │       ├── redacted_sent_request.txt
│   │       ├── sent_request.txt
│   │       └── web_proof.json
│   └── zkvm-benchmarks
│       ├── benchmarks
│       │   ├── Cargo.toml
│       │   ├── assets
│       │   │   ├── email.eml
│       │   │   ├── rsa2048-priv.pem
│       │   │   ├── rsa2048-pub.pem
│       │   │   └── rsa3072-priv.pem
│       │   └── src
│       │       ├── benchmarks
│       │       │   ├── accelerators
│       │       │   │   ├── hash
│       │       │   │   │   ├── keccak.rs
│       │       │   │   │   └── sha2.rs
│       │       │   │   ├── hash.rs
│       │       │   │   └── rsa.rs
│       │       │   ├── accelerators.rs
│       │       │   ├── block_trie.rs
│       │       │   ├── mpt.rs
│       │       │   ├── precompiles
│       │       │   │   ├── email.rs
│       │       │   │   └── url_pattern.rs
│       │       │   └── precompiles.rs
│       │       ├── benchmarks.rs
│       │       └── lib.rs
│       └── runner
│           ├── Cargo.toml
│           ├── build.rs
│           ├── risc0_guest
│           │   ├── Cargo.lock
│           │   ├── Cargo.toml
│           │   └── src
│           │       └── main.rs
│           └── src
│               ├── cycle.rs
│               ├── guest.rs
│               ├── main.rs
│               ├── row.rs
│               ├── runner.rs
│               └── tolerance.rs
├── rust-toolchain.toml
└── rustfmt.toml

384 directories, 1109 files
