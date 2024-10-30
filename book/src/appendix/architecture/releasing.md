# Deployment and Release

## Environments

The process of releasing vlayer spans across four environments:

1. **User Environment** - web browsers
2. **Developer Environment** - Tools and libraries on the developer's local machine
3. **vlayer Node Infrastructure** - Consists of various server types
4. **Blockchain Networks** - Smart contracts deployed across multiple chains

The diagram below illustrates these environments, along with associated artifacts and key interdependencies:

![Schema](/images/architecture/releasing.png)

## Artifacts and Deployment Cycles

Each environment includes multiple artifacts, each with distinct deployment cycle limitations, as detailed below.

### User Environment (Web Browser)

- **Extension**
    - **Release**: vlayer manually releases updates to the Chrome Web Store and other extension platforms. Although automated releases are technically feasible, the store acceptance process introduces some unpredictability.
    - **Installation**: Users install extensions manually from the store.
    - **Updates**: Browsers typically handle automatic updates, additionally users can be encouraged or enforced to update manually if needed.

- **SDK**
    - **Release**: vlayer releases new SDK versions daily.
    - **Installation**: Developers add the SDK to their project dependencies.
    - **Updates**: Neither vlayer nor the user can enforce SDK version updates, making SDK updates the least controllable in terms of version management on the user's end.

### Developer Environment (Command Line Tools)

- **vlayer Command Line Tool** - Used in different contexts:
    - With `init` and `test` flags, tightly integrated with Foundry
    - With `prover`, an optional dependency for local development
- **Local Development SDK**
- **vlayer Smart Contracts** - Managed via Soldeer
- **Foundry** - An external dependency requiring updates synchronized with vlayer to:
    - Ensure `test` and `init` commands operate in the same directory as `forge` and other tools
    - Support the latest REVM (Rust Ethereum Virtual Machine) changes, including hard-fork and network compatibility

Updating these artifacts is encouraged or enforced through vlayer CLI commands (`test`, `init`, `prove`) and is executable via `vlayer update`.

### Blockchain Networks (Smart Contracts)

- **User’s Smart Contract** - Derived from the `Verifier` base class, with deployment managed externally
- **Verifier Helper Smart Contract** - Often deployed daily

### vlayer Node Infrastructure (Servers)

- **User Dashboard** - A user interface for managing proof history and purchasing
- **vlayer Prover** - A server for executing `Prover` operations
- **Chain Proves Cache** - A server for pre-proving on-chain data, including a JSON RPC server and worker components
- **Notary** - Manages notarization in the Web Proofs context, deployed as needed
- **WebSocket Proxy** - Handles TCP/IP connection proxying for Web Proofs, deployed as required
- **Additional Components** - Includes monitoring infrastructure and networked proving systems

All server infrastructure may undergo daily deployments to accommodate updates.