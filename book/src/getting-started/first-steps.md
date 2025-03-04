# First steps with vlayer

## Creating a new project

Run this command to initialize a new vlayer project:
```bash
vlayer init project-name
```

It creates a folder with sample contracts.

### Adding to an existing project
Use the `--existing` flag to initialize vlayer within your existing [Foundry](https://getfoundry.sh/) project:
```bash
cd ./your-project && vlayer init --existing
```

### Example project

To initialize a vlayer project with example prover and verifier contracts, use the `--template` flag as shown below:

```bash
vlayer init simple --template simple
```

The following templates are available for quick project setup:

- `simple`: Prove an ERC20 token balance at a specific block number.
- `simple-email-proof`: Mint an NFT to the owner of an email address from a specific domain.
- `simple-teleport`: Prove a cross-chain ERC20 token balance.
- `simple-time-travel`: Prove the average ERC20 token balance across multiple block numbers.
- `simple-web-proof`: Mint an NFT to the owner of a specific X/Twitter handle using Web Proofs.

## Directory structure
The vlayer directory structure resembles a typical Foundry project but with two additional folders: `src/vlayer` and `vlayer`.
* `src/vlayer`: Contains the Prover and Verifier smart contracts.
* `vlayer`: Has contract deployment scripts, client SDK calls to the prover, and verifier transactions.
 
## Running examples

> ❗️ Make sure that you have [Bun](https://bun.sh/) installed in your system to build and run the examples.

First off, build the contracts by navigating to your project folder and running:
```bash
cd your-project
forge build
```
This compiles the smart contracts and prepares them for deployment and testing.

> Please note that `vlayer init` installs Solidity dependencies and generates `remappings.txt`. Running `forge soldeer install` is not needed to build the example and may overwrite remappings, which can cause build errors.

Then, install Typescript dependencies in vlayer folder by running:
```bash
cd vlayer
bun install
```

### Testnet
In order to use the testnet, you will need to provide a couple of secrets.

Firstly, create `vlayer/.env.testnet.local` - this is where you will put all your secret keys in.

Log in to your [vlayer account](https://accounts.vlayer.xyz/sign-in) next (if you don't yet have a vlayer account, see below) and in the vlayer dashboard, generate a new secret
API key and save it in `vlayer/.env.testnet.local` as

```sh
VLAYER_API_TOKEN=sk_...
```

> ❗️ We will be inviting new users periodically to our testnet. You can join the waitlist at [accounts.vlayer.xyz/waitlist](https://accounts.vlayer.xyz/waitlist).
>
> There are two steps to joining the waitlist:
>   * specify your email address
>   * fill in our typeform with some additional info about yourself
>
> We want to invite folks who are really driven members of our community and would really like to test our products and help us make them even better, therefore
> filling in the typeform will be a proof of your determination and a necessary ingredient to get you in through the door.

Next provide a private key for deploying example contracts and sending transactions to the verifier in the `vlayer/.env.testnet.local` file as

```sh
EXAMPLES_TEST_PRIVATE_KEY=0x....
```

By default, `optimismSepolia` is configured in the `vlayer/.env.testnet` file. However, you can override this setting to use [other testnets](/advanced/dev-and-production.html#testnet).

To change the desired network, set the `CHAIN_NAME` and `JSON_RPC_URL` environment variables in `vlayer/.env.testnet.local`.

Once configured, run the example from within the `vlayer` directory using:

```sh
bun run prove:testnet
```

### Local devnet
Running examples on a local devnet requires deploying a local instance of the prover and anvil.
If you want to run on local environment, use [Docker](/advanced/dev-and-production.html#devnet): 

```bash
$ bun run devnet
```

This command will start all required services in the background.

Once the devnet is up, run the example from within the `vlayer` directory:

```sh
bun run prove:dev
```

## Web Proof example

First, install the vlayer browser extension from the [Chrome Web Store](https://chromewebstore.google.com/detail/vlayer/jbchhcgphfokabmfacnkafoeeeppjmpl) (works with Chrome and Brave browsers).
For more details about the extension, see the [Web Proofs](../javascript/web-proofs.md) section.

Start web app on localhost:

```sh
cd vlayer
bun run web:dev
```
You can manually deploy `WebProofProver` and `WebProofVerifier` as well:

```sh
cd vlayer
bun run deploy:dev # deploy to local anvil
bun run deploy:testnet # deploy to testnet
```

> ❗️ Ensure the contracts are recompiled after every change with `forge build`

The app will be available at `http://localhost:5174` and will display buttons that will let you interact with the extension and vlayer server (open browser developer console to see the app activity).
