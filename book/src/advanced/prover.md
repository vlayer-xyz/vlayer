# Prover
vlayer introduces `Prover` contracts that allow the generation of zero-knowledge proofs. These contracts enable proving claims without exposing secret data to the public. 

## Proving
Proving claims is based on data such as trusted on-chain records, web payloads, or email content. From developer perspective, proofs are output of running the `Prover` Solidity code in the trusted environment. 

See the example code below:
```solidity
contract WebProver is Prover {
    function main(Web calldata web) public pure returns (string memory) {
        require(web.url.equal("https://api.x.com"), "Invalid URL");

        return (web.content);
    }
}
```

Once the `WebProver` computation is complete, proof and the `web.content` is returned. This output can now be consumed and cryptographically verified by the `Verifier` on-chain smart contracts.

Note that `web.content` becomes public input for on-chain verification because it was returned from `main` function. All other variables remain secret.

## Deployment 
The `Prover` contract code must be deployed upon use. To do so, just use our CLI: 
```sh
vlayer deploy
```

The above command deploys the `Prover` contract code to the off-chain zkEVM and submits it's metadata on-chain. 

If successful, the above command returns the contract address and the prover is ready to generate proofs.

## Prover server
The vlayer node is an HTTP server that acts as a prover. You can start it with the following command:

```sh
vlayer serve
```

By default, it accepts JSON-RPC client requests on port `3000` in the format specified in the [JSON-RPC API appendix](/appendix/api.md).

## Proving Modes

The vlayer node provides two proving modes:

- `DEVELOPMENT` - For development and testing only. It executes code and verifies the correctness of the execution, but doesn't perform any actual proving. In this mode, the `Verifier` contract verifies the correctness of computations, but it can be cheated by a malicious `Prover`.
- `PRODUCTION` - Intended for production and final testing. It performs the actual proving.

> By default, the vlayer node operates in the `DEVELOPMENT` mode.
> Note that the `PRODUCTION` mode is much slower than the `DEVELOPMENT` mode. It is important to design protocols with proving execution time in mind, which takes at least a few minutes.
> The `DEVELOPMENT` mode only works on development and test chains to avoid accidental errors.
