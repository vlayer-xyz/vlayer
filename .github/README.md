# GitHub CI

Welcome to the GitHub CI configuration!

Here is an overview of what can be found inside.

## Composite Actions

Contains [composite actions](https://docs.github.com/en/actions/sharing-automations/avoiding-duplication) which are used in our workflows.

| Action | Purpose |
| --- | --- |
| [Contracts prerequisites](./actions/contracts-prerequisites/action.yml) | Installs Foundry for contracts compilation. |
| [Darwin prerequisites](./actions/darwin-prerequisites/action.yml) | Contains steps specific to Darwin machines. |
| [Rust prerequisites](./actions/rust-prerequisites/action.yml) | Installs Rust, toolchains and other tools. |
| [TS prerequisites](./actions/ts-prerequisites/action.yml) | Installs TypeScript prerequisites. |

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
| [Build docker](./workflows/build_docker.yaml) | Verifies that the docker images build. |
| [Build examples](./workflows/build_examples.yaml) | Verifies that the examples build. |
| [Build extension](./workflows/build_extension.yaml) | Verifies that the browser extension builds. |
| [Build SDK](./workflows/build_sdk.yaml) | Verifies that the SDK builds. |
| [Deploy book](./workflows/deploy_book.yaml) | Deploys the Book to Vercel. |
| [Deploy provers](./workflows/deploy_provers.yaml) | Deploys prover servers. |
| [Lint Ansible](./workflows/lint_ansible.yaml) | Lint Ansible code. |
| [Lint Contracts](./workflows/lint_contracts.yaml) | Lint Smart Contracts. |
| [Lint Rust](./workflows/lint_rust.yaml) | Lint Rust code. |
| [Lint TS](./workflows/lint_ts.yaml) | Lint TypeScript code. |
| [Rust compilation performance](./workflows/performance_rust_compilation.yaml) | Checks Rust incremental compilation performance. |
| [PR team labeler](./workflows/pr_team_labeler.yaml) | Adds team labels to PRs. |
| [Release vlayer artifacts](./workflows/release.yaml) | Release artifacts to npm, GitHub Releases, S3. |
| [Test contracts](./workflows/test_contracts.yaml) | Test and lint contracts. |
| [Test E2E devnet](./workflows/test_e2e_devnet.yaml) | E2E test against devnet (Anvil). |
| [Test E2E testnet](./workflows/test_e2e_testnet.yaml) | E2E test against testnet (Sepolia). |
| [Test extension](./workflows/test_extension.yaml) | Test extension with playwright. |
| [Test JS](./workflows/test_js.yaml) | Run TS/JS unit tests. |
| [Test release](./workflows/test_release.yaml) | Runs tests against released artifacts. |
| [Test Rust](./workflows/test_rust.yaml) | Run Rust code. |
| [Test vlayer](./workflows/test_vlayer.yaml) | Run vlayer tests in contracts and examples. |
