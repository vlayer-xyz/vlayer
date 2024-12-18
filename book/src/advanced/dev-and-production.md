# Environments: Devnet & Testnet

The vlayer network consists of several types of nodes: provers, indexers, notaries, and proxies. These nodes are critical for executing vlayer smart contract features, including Time Travel, Teleport, and proofs for Email and Web.

Currently, there are two environments supported:
- **testnet** - public environment supporting multiple L1 and L2 testnets
- **devnet** - local environment that runs with Docker Compose, providing all necessary services for development

The production network release is scheduled for Q1 2025.

## Testnet
vlayer CLI, sdk and example apps are using it by default and there is no need for configuration to use it.

Test Prover operates in [`FAKE` mode](/advanced/dev-and-production.html#prover-modes) and works with following testnets:

| chain | time travel | teleport | email/web |
|---------|-------------|----------|-----------|
| sepolia | 🚧         | ✅      | ✅         |
| optimismSepolia | ✅         | ✅      | ✅         |
| baseSepolia | 🚧        | ✅      | ✅         |
| polygonAmoy |          |       | ✅         |
| arbitrumSepolia |          |       | ✅         |
| lineaSepolia |          |       | ✅         |
| worldchainSepolia |          |       | ✅         |
| zksyncSepoliaTestnet |          |       | ✅         |

✅ supported, 🚧 in progress
## Devnet
Docker Compose allows running the full stack locally, including anvil devnets and all required vlayer nodes.

### Prerequisites
- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)

### Starting devnet
#### From vlayer project
Navigate to the `${project}/vlayer` directory and start services in the background:
```bash
cd ${project}/vlayer
bun run devnet
```
#### Outside of vlayer project
Use vlayer [Docker Compose file](https://install.vlayer.xyz/devnet) to start services:
```sh
docker compose -f <(curl -L https://install.vlayer.xyz/devnet) up -d
```

### Available services
| Service                  | Endpoint                 | Description                               |
|--------------------------|--------------------------|-------------------------------------------|
| anvil-A      | `http://127.0.0.1:8545` | Local devnet        |
| anvil-B      | `http://127.0.0.1:8546` | Secondary devnet (for time travel/teleport testing)                          |
| anvil-C      | `http://127.0.0.1:8547` | Tertiary devnet (for time travel/teleport testing)                           |
| prover         | `http://127.0.0.1:3000` | zkEVM prover for vlayer contracts             |
| indexer   | `http://127.0.0.1:3001` | Storage proof indexer               |
| notary   | `http://127.0.0.1:7047` | TLS Notary server               |
| websocket proxy       | `http://127.0.0.1:55688`| Proxying websocket connections            |
 
### Stopping devnet
Stop all running services:
```bash
docker compose down
```
### Clear cache
Cached proofs for time-travel and teleport are stored in `./chain_db` and can be deleted manually:
```bash
rm -rf ./chain_db
```

## Prover modes
Prover server supports two proving modes:
- **FAKE**: Designed for development and testing purposes, this mode executes code and verifies its correctness without performing actual proving. While the Verifier contract can confirm computations in this mode, it is susceptible to exploitation by a malicious Prover.```
- **GROTH16**: Intended for production and final testing, this mode performs real proving.

### FAKE Mode

Testnet and devnet provers are running in `FAKE` mode by default.
> Note: FAKE mode is limited to dev and test chains to prevent accidental errors.

### GROTH16 Mode
`GROTH16` mode is slower than `FAKE` mode and requires significant computational resources. 

To speed up proof generation, vlayer supports the use of infrastructure like the [Bonsai](https://www.bonsai.xyz/) (and eventually [Boundless](https://beboundless.xyz/)) to offload heavy computations to high-performance machines.

To run prover node in production mode, download and modify `docker-compose.devnet.yaml`:

```yaml
# rest of config
vlayer:
    # existing vlayer config
    environment:
      # other envs...
      BONSAI_API_URL: https://api.bonsai.xyz
      BONSAI_API_KEY: api_key_goes_here
    command: "serve --proof groth16 ...other_args"
```

You can request a `BONSAI_API_KEY` [here](https://docs.google.com/forms/d/e/1FAIpQLSf9mu18V65862GS4PLYd7tFTEKrl90J5GTyzw_d14ASxrruFQ/viewform).

> Note: Protocols should be designed with proving execution times in mind, as it may take a few minutes to generate proof.
