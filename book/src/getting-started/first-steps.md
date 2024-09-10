# First Steps

## Initialization

To initialize a fresh new vlayer project just run following command:
```bash
$ vlayer init project-name
```

The above command will create new `project-name` folder in your current location, add all necessary dependencies with sample vlayer contracts.

### Adding to existing project
To initialize vlayer within your already created Foundry project use `--existing` flag: 
```bash
cd ./your-project && vlayer init --existing
```

## Testing

To run tests, one must first launch a local Ethereum node:
```bash
$ anvil 
```
and in a new terminal session start [Prover server](/advanced/prover.html#prover-server):

```bash
$ vlayer serve
``` 
