# First Steps

## Off-chain smart contracts

By convention off-chain smart contracts have the `.v.sol` extension.

## Initialisation

To initialise a vlayer project run:
```bash
$ vlayer init
```
which should be run inside a foundry-based project.

This will add vlayer dependencies and sample vlayer contracts.

## Testing

To run tests, one must first run:
```bash
$ anvil 
```
and in a new terminal session:

```bash
$ vlayer serve
``` 
