# GitHub CI

Welcome to the GitHub CI configuration!

Here is an overview of what can be found inside.

## Composite Actions

Contains [composite actions](https://docs.github.com/en/actions/sharing-automations/avoiding-duplication) which are used in our workflows.

| Action | Purpose |
| --- | --- |
| [Build extension](/.github/actions/build-extension/action.yml) | Builds the browser extension. |
| [Build guest artifacts](/.github/actions/build-guest-artifacts/action.yml) | Compiles and uploads guest artifacts. |
| [Contracts prerequisites](/.github/actions/contracts-prerequisites/action.yml) | Installs Foundry for contracts compilation. |
| [Darwin prerequisites](/.github/actions/darwin-prerequisites/action.yml) | Contains steps specific to Darwin machines. |
| [Package release binaries](/.github/actions/package-release-binaries/action.yml) | Zips the Rust release binaries. |
| [Replace guest artifacts](/.github/actions/package-release-binaries/action.yml) | Replaces path in prebuild guest artifacts. |
| [Rust prerequisites](/.github/actions/rust-prerequisites/action.yml) | Installs Rust, toolchains and other tools. |
| [Test E2E devnet](/.github/actions/test-e2e-devnet/action.yml) | Runs an E2E test against devnet for a given example. |
| [Test E2E testnet](/.github/actions/test-e2e-testnet/action.yml) | Runs an E2E test against testnet for a given example. |
| [TS prerequisites](/.github/actions/ts-prerequisites/action.yml) | Installs TypeScript prerequisites. |

## Workflows

Our workflows fall into the following categories, grouped by a common prefix:

- Build - Verify that the code or artifacts build correctly.
- Lint - Verify that the code passes linting and formatting rules.
- Test - Runs tests, including E2E and post-release tests.
- Performance - Relates to performance stats such us compilation time.
- Release - Produces and uploads artifacts for public.
- Deploy - Deploys artifacts.

| Workflow | Purpose |
| --- | --- |
| [Build docker](/.github/workflows/build_docker.yaml) | Verifies that the docker images build. |
| [Build examples](/.github/workflows/build_examples.yaml) | Verifies that the examples build. |
| [Build extension](/.github/workflows/build_extension.yaml) | Verifies that the browser extension builds. |
| [Build guest artifacts](/.github/workflows/build_guest_artifacts.yaml) | Verifies that the guest artifacts build. |
| [Build SDK](/.github/workflows/build_sdk.yaml) | Verifies that the SDK builds. |
| [Deploy book](/.github/workflows/deploy_book.yaml) | Deploys the Book to Vercel. |
| [Deploy provers](/.github/workflows/deploy_provers.yaml) | Deploys prover servers. |
| [Deploy chain services](/.github/workflows/deploy_chain_services.yaml) | Deploys chain workers and chain servers. |
| [Generate test chain DB](./github/workflows/generate_test_chain_db.yaml) | Generates a chain DB which is used by worker migration tests. |
| [Lint Ansible](/.github/workflows/lint_ansible.yaml) | Lint Ansible code. |
| [Lint Contracts](/.github/workflows/lint_contracts.yaml) | Lint Smart Contracts. |
| [Lint Rust](/.github/workflows/lint_rust.yaml) | Lint Rust code. |
| [Lint TS](/.github/workflows/lint_ts.yaml) | Lint TypeScript code. |
| [Lint Bash](/.github/workflows/lint_bash.yaml) | Lint Bash code. |
| [Lint Workflows](/.github/workflows/lint_workflows.yaml) | Lint GitHub Actions workflows. |
| [Lint PR Title](/.github/workflows/lint_pr_title.yaml) | Validates Pull Request title against the [Conventional Commits](https://www.conventionalcommits.org/) specification. |
| [Rust compilation performance](/.github/workflows/performance_rust_compilation.yaml) | Checks Rust incremental compilation performance. |
| [PR team labeler](/.github/workflows/pr_team_labeler.yaml) | Adds team labels to PRs. |
| [Release](/.github/workflows/release.yaml) | Reusable workflow to publish artifacts to npm, GitHub Releases, S3. |
| [Release nightly](/.github/workflows/release_nightly.yaml) | Uses the reusable release workflow to publish a nightly release. |
| [Release browser extension](/.github/workflows/release_browser_extension.yaml) | Uploads the extension to Chrome Web Store. |
| [Test contracts](/.github/workflows/test_contracts.yaml) | Test and lint contracts. |
| [Test chain worker migration](./github/workflows/test_chain_worker_migration.yaml) | Verifies that a new version of chain worker can generate new ZK proofs based on existing proofs. |
| [Test E2E devnet](/.github/workflows/test_e2e_devnet.yaml) | E2E test against devnet (Anvil). |
| [Test E2E testnet](/.github/workflows/test_e2e_testnet.yaml) | E2E test against testnet (Sepolia). |
| [Test E2E web flow](/.github/workflows/test_e2e_web_flow.yaml) | E2E test of the web flow. |
| [Test JS](/.github/workflows/test_js.yaml) | Run TS/JS unit tests. |
| [Test release](/.github/workflows/test_release.yaml) | Runs tests against released artifacts. |
| [Test Rust](/.github/workflows/test_rust.yaml) | Run Rust code. |
| [Web Prover Integration Test](/.github/workflows/test_int_web_prover.yaml) | Run Web Prover integration tests. |
| [Test vlayer](/.github/workflows/test_vlayer.yaml) | Run vlayer tests in contracts and examples. |

## Running jobs only when specified paths are changed

There is a [native option](https://docs.github.com/en/actions/writing-workflows/workflow-syntax-for-github-actions#onpushpull_requestpull_request_targetpathspaths-ignore) to make jobs run only when a configured set of paths has changed.
It is typically used for long-running jobs - to skip them, if we only change unrelated code or documentation.

However, this approach is incompatible with required checks - for merging PRs and for the merge queue.
If a job does not start at all, it does not match the requirement and the PR cannot be merged.

This is solved by a [non-native approach](https://github.com/dorny/paths-filter) to path filters. The job does start, but exits shortly.
This way, we achieve a skipped job, while still making it possible to mark the job as required.

Look at [this workflow](../workflows/test_e2e_devnet.yaml) for reference.
