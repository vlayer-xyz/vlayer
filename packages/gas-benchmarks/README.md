# gas-benchmarks

This app is used to gather benchmark data comparing gas to cycles used
for each benchmarked scenario. It uses our client SDK to run the proofs
and then gather the metrics.

## Building benchmarks

There are a few things that need to happen before we can build this package.
Firstly, we need to build the contracts

```
$ pushd contracts/fixtures
$ forge soldeer install
$ forge build
$ popd
```

Next, we need generate typescript bindings for the said contracts

```
$ ./bash/build-ts-types.sh
```

Finally, we need to build the SDK

```
$ bun install
$ pushd packages/sdk
$ bun run build
$ popd
```

and this package

```
$ cd packages/gas-benchmarks
$ bun install
$ bun run build
```

## Running benchmarks

Since we are actually communicating with the Vlayer server, you need to have
`call_server` and `anvil` up and running when benchmarking in fake mode. Then,
just issue

```
$ bun run bench:dev
```

Internally, the app uses `debug` package for logging, so if you are interested
in JSON output of the benchmarks, you can enable `DEBUG=gas-benchmarks` to get
those printed to the console (in addition to standard pretty-printed results table)

```
$ DEBUG="gas-benchmarks" bun run bench:dev
  gas-benchmarks {"No-op":{"gas":26994,"cycles":7340032},"No-op-with-1byte-calldata":{"gas":27742,"cycles":7340032},"No-op-with-2byte-calldata":{"gas":27754,"cycles":7340032},"No-op-with-3byte-calldata":{"gas":27766,"cycles":7340032},"No-op-with-4byte-calldata":{"gas":27778,"cycles":7340032},"No-op-with-10byte-calldata":{"gas":27850,"cycles":7340032},"No-op-with-20byte-calldata":{"gas":27970,"cycles":7340032},"No-op-with-100byte-calldata":{"gas":29314,"cycles":7340032},"No-op-with-1000byte-calldata":{"gas":43710,"cycles":7340032}} +0ms
|                              | Gas                          | Cycles                       |
______________________________________________________________________________________________
| No-op                        | 26994                        | 7340032                      |
| No-op-with-1byte-calldata    | 27742                        | 7340032                      |
| No-op-with-2byte-calldata    | 27754                        | 7340032                      |
| No-op-with-3byte-calldata    | 27766                        | 7340032                      |
| No-op-with-4byte-calldata    | 27778                        | 7340032                      |
| No-op-with-10byte-calldata   | 27850                        | 7340032                      |
| No-op-with-20byte-calldata   | 27970                        | 7340032                      |
| No-op-with-100byte-calldata  | 29314                        | 7340032                      |
| No-op-with-1000byte-calldata | 43710                        | 7340032                      |
______________________________________________________________________________________________  
```

If you would like to log what is happening in the SDK, you can widen the log scope by passing
`DEBUG="*"` instead.

## Adding new benchmark

All benchmarked contracts live in `contracts/fixtures`. Adding a new
contract for benchmarking is as simple as adding a new prover contract
to `contracts/fixtures`, and then adding a matching description to
`./src/benches/new_bench.ts` which will import
the prover spec and encode it as `Benchmark` type

```ts
import { Benchmark } from "../types";
import proverSpec from "../../../../contracts/fixtures/out/NewProver.sol/NewProver";

export const benchmark: Benchmark = {
  name: "NewProver",
  spec: proverSpec,
  args: [someArgs],
  functionName: "proveMe",
};
```

which can then be imported in `./src/bench.ts` and added to the `benchmarks` list

```ts
import { benchmark as newProverBenchmark } from "./benches/new_bench";

const benchmarks = [noopBenchmark, ...noopWithCalldataBenchmarks, newProverBenchmark];
```

When adding a new benchmark, remember to generate matching typescript definition for the
prover spec by running `bash/build-ts-types.sh`.
