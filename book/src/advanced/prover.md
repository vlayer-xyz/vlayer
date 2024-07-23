# Prover
`Prover` contracts accept data from multiple sources (via features such as [time travel](/features/time-travel.html), [teleport](/features/teleport.html), [email proofs](/features/email.html), and [web proofs](/features/web.html)) and allow claims to be proven without exposing all the data on chain.

Under the hood, vlayer generates zero-knowledge proofs of the `Prover` execution. 

## Proving
vlayer `Prover` contracts are almost the same as regular Solidity smart contracts. The main difference is the environment they run on. The `Prover` code runs on the [vlayer zkEVM](/appendix/architecture/prover.html) and proofs of computation are later verified by the on-chain `Verifier` contract.

Extra features like [teleport](/features/teleport.html) or [time travel](/features/time-travel.html) are only available if the contract inherits from the `Prover` class. 

Arbitrary arguments can be passed to `Prover` functions. All arguments are private, while each value returned from the functions becomes public.

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

Once the `WebProver` computation is complete, proof and the `web.content` is returned. This output can now be consumed and cryptographically verified by the `Verifier` on-chain smart contracts.

Note that `web.content` becomes public input for on-chain verification because it was returned from `main` function. All other variables remain secret.

> List of returned arguments need to match with arguments used by `Verifer` (see [Verifier page](/advanced/verifier.html) for details)

## Deployment 
The `Prover` contract code must be deployed before use. To do so, just use regular [Foundry](https://book.getfoundry.sh/tutorials/solidity-scripting) workflow. 

Run local Ethereum test node in the separate terminal: 
```sh
anvil
```

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

Now we can call the node and get proof of computation. 

Here is example JS call to deployed `Prover` contract:
```javascript
import Vlayer from 'vlayer-sdk';

// deployed Prover contract address 
const to = "0x3f275A83aCdA95e48010132a69Fc604a4e1c1797";     

// caller address (optional)
const caller = "0x902A5583aF6723fdeb66C3Db98F888F14353d01a"; 

// teleport to desired chainId
const chainId = 11155420;

// block number you want time travel to
const blockNo = 123067261;  

const main = async () => {
    const client = new Vlayer({ url: `http://localhost:3000` });
    const { proof, params } = await client.call({
        to,
        caller,
        chainId,
        blockNo
    });

    console.log("Generated proof:" proof); 
    console.log("Public inputs:", params);
}

main();
```
See [JSON-RPC API appendix](/appendix/api.md) for more detailed call specification.

## Proving Modes

The vlayer node provides two proving modes:

- `DEVELOPMENT` - For development and testing only. It executes code and verifies the correctness of the execution, but doesn't perform any actual proving. In this mode, the `Verifier` contract verifies the correctness of computations, but it can be cheated by a malicious `Prover`.
- `PRODUCTION` - Intended for production and final testing. It performs the actual proving.

> By default, the vlayer node operates in the `DEVELOPMENT` mode.
> Note that the `PRODUCTION` mode is much slower than the `DEVELOPMENT` mode. It is important to design protocols with proving execution time in mind, which takes at least a few minutes.
> The `DEVELOPMENT` mode only works on development and test chains to avoid accidental errors.
