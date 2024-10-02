# First steps with vlayer

## Creating new project

Run this command to initialize a new vlayer project:
```bash
$ vlayer init project-name
```

It creates a folder with sample contracts.

### Adding to an existing project
Use the `--existing` flag to initialize vlayer within your existing [Foundry](https://getfoundry.sh/) project:
```bash
cd ./your-project && vlayer init --existing
```

### Example project
To initialize vlayer project with example prover and verifier contracts use `--template` flag:
```bash
vlayer init my-airdrop --template private-airdrop
``` 

## Directory structure
The vlayer directory structure resembles a typical Foundry project but with two additional folders: `src/vlayer` and `vlayer`.
* `src/vlayer`: Contains the Prover and Verifier smart contracts.
* `vlayer`: Has contract deployment scripts, client SDK calls to the prover, and verifier transactions.
 

## Runing examples locally
To run vlayer examples locally, first build the contracts by navigating to your project folder and running:
```bash
cd your-project
forge build
```
This compiles the smart contracts and prepares them for deployment and testing.

Then launch a local Ethereum node:
```bash
$ anvil 
```
and in a separate terminal start the [Prover server](/advanced/prover.html#prover-server):

```bash
vlayer serve
```
For Provers using functionalities like teleports or time travel, configure the appropriate JSON-RPC URLs for each chain used:
```bash
vlayer serve \
  --rpc-url '100002:http://localhost:8546' \
  --rpc-url '100001:http://localhost:8545' \
  --rpc-url '11155111:https://eth-sepolia.g.alchemy.com/v2/{ALCHEMY_KEY}' \
  --rpc-url '1:https://eth-mainnet.g.alchemy.com/v2/{ALCHEMY_KEY}' \
  --rpc-url '8453:https://base-mainnet.g.alchemy.com/v2/{ALCHEMY_KEY}' \
  --rpc-url '10:https://opt-mainnet.g.alchemy.com/v2/{ALCHEMY_KEY}'
```