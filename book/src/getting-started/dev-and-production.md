# Devnet, Testnet & Mainnet

The vlayer network consists of several types of nodes: provers, indexers, notaries, and proxies. These nodes are essential for executing vlayer smart contract features, including Time Travel, Teleport, and proofs for Email and Web.

Currently, vlayer supports the following environments:
- **testnet**: public environment supporting multiple L1 and L2 testnets.
- **devnet**: local environment that runs with Docker Compose, providing all necessary services for development.
- **mainnet**: main public network supporting Ethereum Mainnet, Base, Optimism, and Arbitrum.

## Testnet

By default, vlayer CLI, SDK, and example apps use the testnet environment, with no additional configuration required.

The Test Prover operates in [`FAKE` mode](/getting-started/dev-and-production.html#prover-modes) and works with the following testnets:

| chain | time travel | teleport | email/web |
|---------|-------------|----------|-----------|
| baseSepolia | 🚧        | ✅      | ✅         |
| sepolia | 🚧         | ✅      | ✅         |
| optimismSepolia | ✅         | ✅      | ✅         |
| polygonAmoy |          |       | ✅         |
| arbitrumSepolia |          |       | ✅         |
| lineaSepolia |          |       | ✅         |
| worldchainSepolia |          |       | ✅         |
| zksyncSepoliaTestnet |          |       | ✅         |

✅ Supported, 🚧 In progress

### Public Testnet Services

| Service            | Endpoint                         | Description                                  |
|--------------------|----------------------------------|----------------------------------------------|
| Prover             | `https://stable-fake-prover.vlayer.xyz` | zkEVM prover for vlayer contracts     |
| Nightly Prover     | `https://nightly-fake-prover.vlayer.xyz` | Nightly version of the zkEVM prover  |
| Indexer            | `https://test-chainservice.vlayer.xyz` | Storage proof indexer                |
| Notary             | `https://test-notary.vlayer.xyz` | TLS Notary server                            |
| WebSocket Proxy    | `wss://test-wsproxy.vlayer.xyz`| Proxying websocket connections for TLS Notary |

## Devnet

Devnet allows you to run the full stack locally, including anvil and all required vlayer nodes.

### Starting Devnet

#### Prerequisites
- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)

#### From the vlayer Project

Navigate to the vlayer project directory and start services in the background:
```bash
cd ${project}/vlayer
bun run devnet:up
```

You can stop devnet anytime with:
```bash
cd ${project}/vlayer
bun run devnet:down
```
It is useful in case of any Docker configuration change. 

### Available Services

| Service            | Endpoint                       | Description                                 |
|--------------------|--------------------------------|---------------------------------------------|
| Anvil-A            | `http://127.0.0.1:8545`        | Local devnet                                |
| Anvil-B            | `http://127.0.0.1:8546`        | Secondary devnet (for time travel/teleport testing) |
| Anvil-C            | `http://127.0.0.1:8547`        | Tertiary devnet (for time travel/teleport testing) |
| Prover             | `http://127.0.0.1:3000`        | zkEVM prover for vlayer contracts           |
| Indexer            | `http://127.0.0.1:3001`        | Storage proof indexer                       |
| Notary             | `http://127.0.0.1:7047`        | TLS Notary server                           |
| WebSocket Proxy    | `ws://127.0.0.1:3003`       | Proxying websocket connections              |

### Clearing Cache

Cached proofs for time travel and teleport are stored in `./chain_db` and can be deleted manually:
```bash
rm -rf ./chain_db
```

## Mainnet
The Mainnet Prover operates in [`GROTH16` mode](/getting-started/dev-and-production.html#prover-modes) and works with the following chains:

| chain | time travel | teleport | email/web |
|---------|-------------|----------|-----------|
| mainnet | ✅        | ✅      | ✅         |
| base | ✅         | ✅      | ✅         |
| optimism | ✅         | ✅      | ✅         |

✅ Supported, 🚧 In progress

### Public Mainnet Services

| Service            | Endpoint                         | Description                                  |
|--------------------|----------------------------------|----------------------------------------------|
| Prover             | `https://stable-prod-prover.vlayer.xyz` | zkEVM prover for vlayer contracts     |
| Indexer            | `https://chainservice.vlayer.xyz` | Storage proof indexer                |
| Notary             | `https://notary.vlayer.xyz` | TLS Notary server                            |
| WebSocket Proxy    | `wss://wsproxy.vlayer.xyz`| Proxying websocket connections for TLS Notary |

## Prover Modes

The prover server supports two proving modes:
- **FAKE**: Designed for development and testing purposes, this mode executes code and verifies its correctness without performing actual proving. While the Verifier contract can confirm computations in this mode, it is vulnerable to exploitation by a malicious Prover.
- **GROTH16**: Intended for production and final testing, this mode performs real proving.

### FAKE Mode

Testnet and devnet provers run in `FAKE` mode by default.

> **Note**: FAKE mode is limited to dev and test chains to prevent accidental errors.

### GROTH16 Mode

`GROTH16` mode is slower than `FAKE` mode and requires significant computational resources.

To speed up proof generation, vlayer supports the use of infrastructure like [Bonsai](https://www.bonsai.xyz/) (and eventually [Boundless](https://beboundless.xyz/)) to offload heavy computations to high-performance machines.

To run a prover node in production mode, download and modify `call_server/service.yaml`:

```yaml
vlayer-call-server:
    # existing vlayer config
    environment:
      # other env variables...
      BONSAI_API_URL: https://api.bonsai.xyz
      BONSAI_API_KEY: api_key_goes_here
    command: "--proof groth16 ...other_args"
```

You can request a `BONSAI_API_KEY` [here](https://docs.google.com/forms/d/e/1FAIpQLSf9mu18V65862GS4PLYd7tFTEKrl90J5GTyzw_d14ASxrruFQ/viewform).

> **Note**: Protocols should be designed with proving execution times in mind, as generating a proof may take several minutes.
