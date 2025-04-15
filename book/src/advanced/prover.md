# Prover

vlayer `Prover` contracts are almost the same as regular Solidity smart contracts, with two main differences:

- **Access to Off-Chain Data:** `Prover` contracts accept data from multiple sources through features such as [time travel](/features/time-travel.html), [teleport](/features/teleport.html), [email proofs](/features/email.html), and [web proofs](/features/web.html). This allows claims to be verified on-chain without exposing all input the data.

- **Execution Environment:** The `Prover` code executes on the vlayer zkEVM, where the proofs of computation are subsequently verified by the on-chain `Verifier` contract. Unlike the on-chain contract, the `Prover` does not have access to the current block. It can only access previously mined blocks. Under the hood, vlayer generates zero-knowledge proofs of the `Prover`'s execution.

## Prover in-depth

### Prover parent contract
Any contract function can be run in the vlayer prover, but to access the additional features listed above, the contract should inherit from the `Prover` contract and any function can be used as a proving function.

### Arguments and returned value
Arbitrary arguments can be passed to Prover functions. All arguments are private, meaning they are not visible on-chain; however, they are visible to the prover server.

All data returned by functions is public. To make an argument public on-chain, return it from the function.

### Limits

We impose the following restrictions on the proof:

- Calldata passed into the `Prover` cannot exceed 5 MB. 

### Proof

Once the `Prover` computation is complete, a proof is generated and made available along with the returned value. This output can then be consumed and cryptographically verified by the `Verifier` on-chain smart contract.

Note that all values returned from `Prover` functions becomes a public input for on-chain verification. Arguments passed to `Prover` functions remain private.

> The list of returned arguments must match the arguments used by the `Verifier` (see the [Verifier page](/advanced/verifier.html) for details).  
> 
> vlayer `Prover` must return a placeholder proof as the first argument to maintain consistency with `Verifier` arguments. Placeholder `Proof` returned by `Prover` is created by its method `proof()`, which is later replaced by the real proof, once it's generated.

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
