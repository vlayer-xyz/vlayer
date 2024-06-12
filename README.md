# vlayer


Vlayer allows you to run EVM smart contracts off-chain and use results of their execution in on-chain smart contracts. Off-chain smart contracts have extra capabilities, like access to historical state of many chains, user emails and web data.

## Off-chain smart contracts

By convention off-chain smart contracts have the `.v.sol` extension.

## Examples 
You can find and run examples from `examples` directory.

To run an example - go to specific example directory (e.g. `example/simple`) and run:
```sh
../../bash/vlayer-build.sh
```


## Architecture
On the high level, vlayer runs zkEVM that produces proof of proper execution. Under the hood, vlayer is written in Rust that is complied to zero knowledge proofs. Currently, Rust is complied with RISC-0, but we aim to build vendor-lock free solutions working on multiple zk stack, like [sp-1](https://github.com/succinctlabs/sp1) or [Jolt](https://github.com/a16z/jolt). Inside rust [revm](https://github.com/bluealloy/revm) is executed. 

Architecture is inspired by RISC-0 steel, with 3 main components, that can be found in `rust/template/` subdirectories:
- host - (in `host`) - Collects all data required by guest and runs guest execution and proving
- guest - (in `guest_wrapper/guest`) - Contains the code to be run inside zkvm
- guest-wrapper - (in `guest_wrapper`) - Compiles guest to [risc-0](https://doc.rust-lang.org/rustc/platform-support/riscv32im-risc0-zkvm-elf.html) target and makes it available to be run inside host

Host passes arguments to guest via standard input/output like functionality.

In ZK terms - all inputs are **private**, all outputs are **public**. If you need public inputs - copy them to the output.

