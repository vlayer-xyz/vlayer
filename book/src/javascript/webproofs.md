# Web proofs from javascript

## Proving

To augment proving request with web proof, add webProof argument to `prove` function call.

```ts
const hash = vlayerClient.prove({
    to: '0x70997970c51812dc3a010c7d01b50e0d17dc79c8',
    data,    
    chain: mainnet,
    webProof
});
```
