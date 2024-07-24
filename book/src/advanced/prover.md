# Prover
vlayer `Prover` contracts are almost the same as regular Solidity smart contracts, with two main differences:

- **Access to off-chain data.** `Prover` contracts accept data from multiple sources (via features such as [time travel](/features/time-travel.html), [teleport](/features/teleport.html), [email proofs](/features/email.html), and [web proofs](/features/web.html)) and allow claims to be proven without exposing all the data on chain.

- **Execution environment.** The `Prover` code runs on the [vlayer zkEVM](/appendix/architecture/prover.html) and proofs of computation are later verified by the on-chain `Verifier` contract. Unlike the on-chain contract, prover don't have access to the current block. It have access to already mined blocks only. Under the hood, vlayer generates zero-knowledge proofs of the `Prover` execution. 

## Prover in-depth

### Prover parent contract
Any contract function can be run in prover, but to get access to the additional features listed in the section above, it must inherit from `Prover'.

### Arguments and returned value
Arbitrary arguments can be passed to `Prover` functions. All arguments are private, which means, they are not visible on-chain. However, keep in mind, they are visible to the prover server. 

All data returned by functions is public. To make an argument public on the chain, return it from the function. 

### Injected values
Two additional variables may be injected to function body: `web` and `mail`. These variables allows to access data like email or web content and use them in the contract logic.     

See the example code below:
```solidity
contract WebProver is Prover {
    function main() public pure returns (string memory) {
        require(web.url.equal("https://api.x.com"), "Invalid URL");
        return (web.content);
    }
}
```

### Proof

Once the `WebProver` computation is complete, a proof is generated and made available along with the returned value (e.g., `web.content` in our example). This output can then be consumed and cryptographically verified by the `Verifier` on-chain smart contract.

Note that `web.content` becomes a public input for on-chain verification because it was returned from the `main` function. All other input variables remain private.

> The list of returned arguments must match the arguments used by the `Verifier` (see the [Verifier page](/advanced/verifier.html) for details).

## Deployment 
The `Prover` contract code must be deployed before use. To do so, just use regular [Foundry](https://book.getfoundry.sh/tutorials/solidity-scripting) workflow. 

Prepare deployment script:
```solidity
contract SimpleScript is Script {
    function setUp() public {}

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("DEPLOYER_PRIV");
        vm.startBroadcast(deployerPrivateKey);

        SimpleProver simpleProver = new SimpleProver();
        console2.log("SimpleProver contract deployed to:", address(simpleProver));
    }
}
```

### Local environment
In the separate terminal, run the local Ethereum test node:
```sh
anvil
```

Then save and execute it: 
```sh
DEPLOYER_PRIV=PRIVATE_KEY forge script path/to/Script.s.sol --rpc-url http://127.0.0.1:8545
```

The above command deploys the `SimpleProver` contract code to local network. 

If successful, the above command returns the contract address and the `Prover` is ready for generating proofs.

> For production use proper RPC url and [encrypt private key](https://book.getfoundry.sh/reference/cast/cast-wallet-new) instead of using it via plain text

## Prover server
The vlayer node is an HTTP server that acts as a prover. By default, it accepts JSON-RPC client requests on port `3000`. 

You can start it with the following command:
```sh
vlayer serve
```

See [JSON-RPC API appendix](/appendix/api.md) for more detailed call specification.

## Proving Modes

The vlayer node provides two proving modes:

- `DEVELOPMENT` - For development and testing only. It executes code and verifies the correctness of the execution, but doesn't perform any actual proving. In this mode, the `Verifier` contract verifies the correctness of computations, but it can be cheated by a malicious `Prover`.
- `PRODUCTION` - Intended for production and final testing. It performs the actual proving.

> By default, the vlayer node operates in the `DEVELOPMENT` mode.
> Note that the `PRODUCTION` mode is much slower than the `DEVELOPMENT` mode. It is important to design protocols with proving execution time in mind, which takes at least a few minutes.
> The `DEVELOPMENT` mode only works on development and test chains to avoid accidental errors.
