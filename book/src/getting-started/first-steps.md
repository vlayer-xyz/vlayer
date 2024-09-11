# First Steps

## Initialization

To initialize a fresh new vlayer project, run the following command:
```bash
$ vlayer init project-name
```

The above command will create a new folder called `project-name` in your current location, and then add all the necessary dependencies with sample vlayer contracts.

### Adding to existing project
Use the `--existing` flag to initialize vlayer within your existing [Foundry](https://getfoundry.sh/) project:
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
