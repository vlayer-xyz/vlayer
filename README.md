# vlayer
vlayer enables developers to extract, verify and integrate real-world data into Ethereum smart contracts. Our technology is powered by Zero Knowledge Proofs (ZKP) and Multi-Party Computation (MPC), allowing you to securely verify private data without exposing sensitive information. 

Our four core features include: 
- [Web Proofs](https://book.vlayer.xyz/features/web.html): Access verified web data, including APIs and websites, in your smart contracts 
- [Email Proofs](https://book.vlayer.xyz/features/email.html): Tap into email content from your smart contracts and use it on-chain
- [Time Travel](https://book.vlayer.xyz/features/time-travel.html): Leverage historical on-chain data in your smart contracts
- [Teleport](https://book.vlayer.xyz/features/teleport.html): Execute a smart contract across different EVM-comptable blockchain networks

## Quick Start

### Prerequisites
- **Rust** - Install via [rustup](https://rustup.rs/): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Foundry** - Install via [foundryup](https://getfoundry.sh/): `curl -L https://foundry.paradigm.xyz | bash`
- **Bun** - Install via `curl -fsSL https://bun.sh/install | bash`

### Try Your First Example
```bash
# 1. Clone and setup
git clone https://github.com/vlayer-xyz/vlayer.git
cd vlayer

# 2. Build contracts (required first)
bash/mock-imageid.sh          # Generate required ImageID.sol
bash/build-all-contracts.sh   # Build all contracts

# 3. Try the simple example
cd examples/simple/vlayer
bun install
VLAYER_ENV=dev bun run prove  # Note: requires services running (see below)
```

### Local Development Setup
For full local development, you'll need to run these services:

```bash
# Terminal 1: Start local blockchain
anvil

# Terminal 2: Start vlayer prover (requires full setup)
bash/run-services.sh

# Terminal 3: Run example
cd examples/simple/vlayer
VLAYER_ENV=dev bun run prove
```

## Project Structure

```
vlayer/
├── rust/           # Core ZK proving services (61% of codebase)
│   ├── services/   # Backend services (call, chain, dns)
│   └── web_proof/  # Web data verification
├── contracts/      # Solidity smart contracts
│   └── vlayer/     # Core vlayer contracts
├── packages/       # TypeScript SDK and tools
│   └── sdk/        # Main developer SDK
├── examples/       # Working examples
│   ├── simple/     # Basic prover/verifier example
│   ├── simple-web-proof/    # Web data verification
│   └── simple-email-proof/  # Email verification
└── book/          # Documentation source
```

## Getting Started

### 🚀 I want to understand vlayer
- Read the [vlayer book](https://book.vlayer.xyz/introduction.html)
- Check out the [examples](./examples/) directory
- Review the [Quick Start](#quick-start) above

### 🛠️ I want to try vlayer locally
1. **Quick setup**: Follow the [Quick Start](#quick-start) above
2. **Full setup**: Use the [vlayerup installer](https://book.vlayer.xyz/getting-started/first-steps.html)

### 💻 I want to build with vlayer
- **Contracts**: Extend `Prover` and `Verifier` base contracts
- **SDK**: Use `@vlayer/sdk` for TypeScript integration
- **Examples**: Start with [simple example](./examples/simple/)

### 🔧 I want to contribute
- See [Contributing Guide](https://book.vlayer.xyz/appendix/contributing/overview.html)
- Check [development commands](./CLAUDE.md#development-commands)

## Development Workflow

### Building the Project
```bash
# Build everything
bash/build-all-contracts.sh   # Contracts + examples
bun run lint                  # Lint TypeScript
bash/lint/rust.sh            # Lint Rust
```

### Running Tests
```bash
cargo test              # Rust tests
forge test             # Solidity tests
bun run test:unit      # TypeScript tests
```

## Troubleshooting

### Build Issues
- **Missing ImageID.sol**: Run `bash/mock-imageid.sh` first
- **Soldeer dependencies**: Run `forge soldeer install` in contract directories
- **Build order**: Build core contracts before examples

### Runtime Issues  
- **Connection refused (port 8545)**: Start Anvil with `anvil`
- **Connection refused (port 3000)**: Start vlayer prover services
- **Environment variables**: Set `VLAYER_ENV=dev` for local development

### Common Commands
```bash
# Setup
bash/mock-imageid.sh          # Generate ImageID.sol
bash/build-all-contracts.sh   # Build all contracts
bun install                   # Install dependencies

# Development
bash/lint.sh                  # Lint everything
bash/e2e-test.sh             # Run E2E tests
bash/tsc-examples.sh         # Type check examples
```

## Contributing

We're excited you're interested in contributing to vlayer. This [contributing](https://book.vlayer.xyz/appendix/contributing/overview.html) section in vlayer book outlines the process to get involved.

## License
vlayer v1.0 uses a dual licensing model, in which vlayer Labs Ltd. (core contributor of vlayer) is designated for commercialization, while the code is licensed to the general public under Business Source License (BSL) 1.1, which shall convert to the permissive MIT license after three years. 

This approach ensures that our technology remains accessible for broad community innovation, initially protecting project integrity while ultimately supporting widespread collaboration and adoption by developers across the globe.

By contributing, you agree that your contributions will be licensed under the Business Source License (BSL) 1.1. 

## Acknowledgements

We would like to acknowledge the following open-source projects that inspired and provided a foundation for this work:

* [Steel](https://crates.io/crates/risc0-steel) - Hardened off-chain Execution for EVM dapps

## Security Audits
- ✅ [Veridise Audit Report (Q2 2025)](./audits/audit-2025-q2-veridise.pdf)
