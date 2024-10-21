# Prover
<div class="feature-card feature-in-dev">
  <div class="title">
    <svg width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
    <path d="M8.57499 3.21665L1.51665 15C1.37113 15.252 1.29413 15.5377 1.29331 15.8288C1.2925 16.1198 1.3679 16.4059 1.51201 16.6588C1.65612 16.9116 1.86392 17.1223 2.11474 17.2699C2.36556 17.4174 2.65065 17.4968 2.94165 17.5H17.0583C17.3493 17.4968 17.6344 17.4174 17.8852 17.2699C18.136 17.1223 18.3439 16.9116 18.488 16.6588C18.6321 16.4059 18.7075 16.1198 18.7067 15.8288C18.7058 15.5377 18.6288 15.252 18.4833 15L11.425 3.21665C11.2764 2.97174 11.0673 2.76925 10.8176 2.62872C10.568 2.48819 10.2864 2.41437 9.99999 2.41437C9.71354 2.41437 9.43193 2.48819 9.18232 2.62872C8.93272 2.76925 8.72355 2.97174 8.57499 3.21665V3.21665Z" stroke="#FCA004" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
    <path d="M10 7.5V10.8333" stroke="#FCA004" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
    <path d="M10 14.1667H10.0083" stroke="#FCA004" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
    </svg>
    Actively in Development
  </div>
  <p>Our team is currently working on this feature. If you experience any bugs, please let us know <a href="https://discord.gg/JS6whdessP" target="_blank">on our Discord</a>. We appreciate your patience. </p>
</div>

vlayer `Prover` contracts are almost the same as regular Solidity smart contracts, with two main differences:

- **Access to Off-Chain Data:** `Prover` contracts accept data from multiple sources through features such as [time travel](/features/time-travel.html), [teleport](/features/teleport.html), [email proofs](/features/email.html), and [web proofs](/features/web.html). This allows claims to be verified on-chain without exposing all input the data.

- **Execution Environment:** The `Prover` code executes on the [vlayer zkEVM](/appendix/architecture/prover.html), where the proofs of computation are subsequently verified by the on-chain `Verifier` contract. Unlike the on-chain contract, the `Prover` does not have access to the current block. It can only access previously mined blocks. Under the hood, vlayer generates zero-knowledge proofs of the `Prover`'s execution.

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
