# Environments: Devnet & Testnet

The vlayer network consists of services nodes: provers, indexers, notaries and proxies. This nodes are necessary for execution of vlayer smart contracts features like [time travel](/features/time-travel.html), [teleport](/features/teleport.html), [email](/features/email.html) / [web](/features/web.html) proofs.

There are two environments supported:
- **testnet** - default, predeployed and ready to use environment with public testnet support
- **devnet** - Docker Compose environment with local vlayer services and anvil devnet

## Testnet
Default vlayer prover is available at `https://test-prover.vlayer.xyz`. Prover operates in [`FAKE` mode](/advanced/dev-and-production.html#prover-modes) and works with following testnets:

| chain | time travel | teleport | email/web |
|---------|-------------|----------|-----------|
| sepolia | ✅         | ✅      | ✅         |
| optimismSepolia | ✅         | ✅      | ✅         |
| baseSepolia | ✅         | ✅      | ✅         |
| polygonAmoy | ✅         | ✅      | ✅         |
| arbitrumSepolia | ✅         | ✅      | ✅         |
| lineaSepolia | ✅         | ✅      | ✅         |
| worldchainSepolia | ✅         | ✅      | ✅         |
| zksyncSepoliaTestnet | ✅         | ✅      | ✅         |

## Devnet
Every vlayer project has `docker-compose.devnet.yaml` in the `${project}/vlayer` directory. This file contains all necessary services for local environment.

> Ensure [Docker](https://docs.docker.com/get-docker/) and [Docker Compose](https://docs.docker.com/compose/install/) are installed on your system.

### Start devnet
To pull the required images and start the containers in the background use this command:

```bash
cd ${project}/vlayer
bun run devnet
```

Optionally, one may run devnet locally outside of the vlayer project directory using vlayer [Docker Compose file](https://install.vlayer.xyz/devnet):
```sh
docker compose -f <(curl -L https://install.vlayer.xyz/devnet) up -d
```

### Access to services
- **anvil devnets** (useful for time travel / teleport testing) 
  - `anvil-a` is accessible at `http://127.0.0.1:8545`.
  - `anvil-b` is accessible at `http://127.0.0.1:8546`.
  - `anvil-c` is accessible at `http://127.0.0.1:8547`.
- **vlayer Prover** listens on `http://127.0.0.1:3000`.
- **vlayer Chain Server** is available at `http://127.0.0.1:3001`.
- **Websocket Proxy** accessible via `http://127.0.0.1:55688`.
 
### Stop and clean up
To stop all running services:
```bash
docker compose down
```

This will stop and remove all containers but preserve data in `./chain_db`. If you want to remove the data as well, delete the `./chain_db` directory.

## Prover modes
Prover server supports two proving modes:
- **FAKE**: Used for development and testing. It executes code and verifies the correctness of execution but does not perform actual proving. In this mode, the `Verifier` contract can confirm computations, but a malicious `Prover` could exploit the system.
- **GROTH16**: Intended for production and final testing, this mode performs real proving.

### FAKE Mode

Testnet and devnet provers are running in `FAKE` mode by default. Fake mode can be enabled explicitly by using the `--proof` argument:
```sh
serve --proof fake
```
> Note: FAKE mode is limited to test and dev chains to prevent accidental errors.

### GROTH16 Mode
`GROTH16` mode is slower than `FAKE` mode and requires significant computational resources. 

To speed up proof generation, vlayer supports the use of infrastructure like the [Bonsai](https://www.bonsai.xyz/) (and eventually [Boundless](https://beboundless.xyz/)) to offload heavy computations to high-performance machines.

To run prover in production mode, modify `docker-compose.devnet.yaml`:

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
