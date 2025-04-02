# Upgrading Risc0

There are 4 parts to this process:

* upgrading r0vm and rust toolchain,
* upgrading Rust cargo dependencies,
* upgrading Risc0 Ethereum contracts, and
* generating new Guest ID using Docker.

Deciding which parts need updating will mainly depend if Risc0 has issued a patch, minor or major version bump. Quite naturally, a major version
bump will require updating all 4 parts, however minor or patch will most likely require updating toolchain and Rust cargo dependencies, and
generating a new Guest ID using Docker.

In what follows, all paths are relative to the project's root.

Please note the upgrade process should be done in the order specified below.

## Upgrade r0vm and rust toolchain

1. Risc0 action - `.github/actions/risc0/action.yml`

```yaml
inputs:
  version:
    description: 'Version of r0vm to install'
    default: 2.0.0 # bump r0vm version which usually corresponds to overall risc0 version
    required: false
  rust_version:
    description: 'Version of Rust toolchain for building guest images'
    default: 1.81.0 # bump Rust toolchain version which usually changes very rarely
    required: false
```

2. Ansible config for deployed provers - `ansible/group_vars/all.yml`

```yaml
vlayer_risc0_version: '2.0.0'
```

3. Post-release test scripts - `bash/test-release-local-prover.sh` and `bash/test-release-remote-prover.sh`

```bash
echo "::group::risczero installation"
curl -L https://risczero.com/install | bash
export PATH="$PATH:~/.risc0/bin"
export PATH="$PATH:~/.cargo/bin"
rzup install r0vm 2.0.0
rzup install rust 1.81.0
rzup show
echo "::endgroup::"
```

4. Nightly Dockerfiles - `docker/call_server/Dockerfile.nightly` and `docker/chain_worker/Dockerfile.nightly`

```docker
RUN curl -L https://github.com/risc0/risc0/releases/download/v2.0.0/cargo-risczero-x86_64-unknown-linux-gnu.tgz -o cargo-risczero-x86_64-unknown-linux-gnu.tgz
```

5. Nix flake - `nix/risc0.nix`

This is a two-step process. First, change/add new Risc0 release like so:

```nix
let
releases = {
  "2.0.0" = {
    "aarch64-darwin" = {
      arch = "aarch64-apple-darwin";
      hash = pkgs.lib.fakeHash;
    };
  };
  # rest of the file
};
in
rec {
  default = risc0."2.0.0";
  inherit (package "2.0.0") "2.0.0";
  # rest of the file
}
```

Next, run from the project root and get the actual hash value for the new package:

```sh
$ nix develop
error: hash mismatch in fixed-output derivation '/nix/store/z82gnv3n7z1js3s33zrnq25kafdl82pk-cargo-risczero-aarch64-apple-darwin.tgz.drv':
         specified: sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=
            got:    sha256-d/gQVXK+WAPTGRnx8Fw3XTWe6oKPw8FsLK7VNxC9aF8=
error: 1 dependencies of derivation '/nix/store/z45x6ixixxi6wrvsg29p9r8g6p2546qy-cargo-risczero-v2.0.0.drv' failed to build
error: 1 dependencies of derivation '/nix/store/nl0wkkhzm2zh7f0n6cmd2s7ismc5bm33-vlayer-env.drv' failed to build
```

And copy the actual hash value (`got` in the snippet above) into `nix/risc0.nix` file:

```nix
let
releases = {
  "2.0.0" = {
    "aarch64-darwin" = {
      arch = "aarch64-apple-darwin";
      hash = "sha256-d/gQVXK+WAPTGRnx8Fw3XTWe6oKPw8FsLK7VNxC9aF8=";
    };
  };
  # rest of the file
```

6. Update Risc0 requirements in the book in [/appendix/contributing/rust.md](/appendix/contributing/rust.html)

## Upgrade Rust cargo dependencies

In this step of the upgrade process, as with any version bump of a Rust dependency, it may so happen that just a bump is not enough
and API usage needs to be updated too. Bear that in mind when updating and experiencing compilation errors.

1. Main project manifest - `Cargo.toml` 

We leave out `risc0-build-ethereum` unless there was a major Risc0 version bump that forced Ethereum contracts update, in which case
we will come back to it in the next step of the upgrade process.

```toml
# ...
risc0-build = "=2.0.1";
risc0-build-ethereum = { git = "https://github.com/vlayer-xyz/risc0-ethereum.git", rev = "12f4a76616bdc31108fc585c4a0ce7ebd97059fd" }
risc0-zkp = { version = "=2.0.0", default-features = false }
risc0-zkvm = "=2.0.0"
bonsai-sdk = "=1.4.0"
```

2. Guests manifests - `rust/guest_wrapper/risc0_call_guest/Cargo.toml` and `rust/guest_wrapper/risc0_chain_guest/Cargo.toml`

```toml
# ...
risc0-zkvm = { version = "=2.0.0", default-features = false, features = ["std", "unstable"] }
risc0-zkvm-platform = { version = "=2.0.1", features = [ "rust-runtime", "export-getrandom", "sys-getenv" ] }
```

3. Risc0 VM benchmarks - `rust/zkvm-benchmarks/runner/risc0_guest/Cargo.toml`

```toml
# ...
risc0-zkvm = { version = "=2.0.0", default-features = false, features = ["std", "unstable"] }
```

## (Optional) Upgrade Risc0 Ethereum contracts

If there was a major version bump, the project is likely failing with a compilation error when trying to build `risc0-build-ethereum`
crate. In this case, you will need to upgrade our `risc0-ethereum` fork to the desired Risc0 release version.

1. Upgrade `risc0-ethereum` repo

Clone [`risc0-ethereum`](https://github.com/vlayer-xyz/risc0-ethereum) repo. You will notice that each significant Risc0 version bump
has a matching branch with our changes, for instance `release-1.2-soldeer` or `release-2.0-soldeer`. We will do the same for this release.

1.1. Create new release branch

```sh
git checkout main
git checkout -b release-2.0-soldeer
```

1.2. Sync with Risc0 release

```sh
git remote add upstream git@github.com:risc0/risc0-ethereum
git fetch upstream
git checkout v2.0.0 # or whatever Risc0 has tagged the release as
```

1.3. Configure soldeer and remove submodules

See commit [a5613d1c](https://github.com/vlayer-xyz/risc0-ethereum/commit/a5613d1c4583b70ef7c468561a18b5c2051d7cb5) for an illustrative example.

1.3.1. Update `.gitignore`

Add the following line anywhere in `.gitignore`

```
# ...
contracts/dependencies/

```

1.3.2. Update Foundry config - `contracts/foundry.toml`

Point `libs` at `./dependencies` and add `[dependencies]` header as the last header in the manifest

```toml
# ...
libs = ["./dependencies"]

# ...

[dependencies]

# See more config options https://book.getfoundry.sh/static/config.default.toml
```

1.3.3. Remove git submodules

```sh
$ git rm -f lib/forge-std lib/openzeppelin-contracts
```

1.4. Add soldeer dependencies

See commit [ab6aaa37](https://github.com/vlayer-xyz/risc0-ethereum/commit/ab6aaa37a2e962343c31bc800450d2d42ba9b058) for an illustrative example.

1.4.1. Add dependencies to manifest - `contracts/foundry.toml`

```toml
[dependencies]
forge-std = "1.9.4"
"@openzeppelin-contracts" = "5.1.0"
```

1.4.2. Tweak the remappings - `contracts/remappings.txt`

```
forge-std/=dependencies/forge-std-1.9.4/src
openzeppelin-contracts=dependencies/@openzeppelin-contracts-5.1.0/
```

1.4.3. Pull the dependencies from soldeer registry and commit the lock file

```sh
$ cd contracts
$ forge soldeer install
$ git add soldeer.lock
$ git commit
```

1.5. Refactor remappings in contracts

We need to tweak every contract that imports `openzeppelin-contracts` to import it as `openzeppelin-contracts/...`
rather than `openzeppelin/contracts/...`.

For example, take `contracts/src/RiscZeroSetVerifier.sol`

```
// ...
import {MerkleProof} from "openzeppelin-contracts/utils/cryptography/MerkleProof.sol";
// ...  
```

See commit [cf3d1381](https://github.com/vlayer-xyz/risc0-ethereum/commit/cf3d1381f076f1094455a0fc6ebd5bd172424d09) for an illustrative example.

1.6. Pack contracts using soldeer

Packing contracts is equivalent to publishing the contracts with `--dry-run` flag on in soldeer lingo

```
$ forge soldeer push risc0-ethereum~2.0.0 contracts --dry-run
```

This command will generate `contracts/contracts.zip` that we will need in the next step.

1.7. Create GitHub release

Finally, create GitHub release by first tagging the branch and then uploading `contracts/contracts.zip`.

```
$ git tag v2.0.0-soldeer
$ git push origin tag v2.0.0-soldeer
```

1.8. Update Risc0 Ethereum contracts in the main repo

Now, let's circle back to the main repo (`vlayer`). We will need to propagate the new version of Solidity contracts in a few places.

1.8.1. Update main Cargo manifest - `Cargo.toml`

```
risc0-build-ethereum = { git = "https://github.com/vlayer-xyz/risc0-ethereum.git", rev = "cf3d1381f076f1094455a0fc6ebd5bd172424d09" }
```

Here the revision hash corresponds to the commit hash which we tagged in the `risc0-ethereum` repo.

1.8.2. Update foundry manifest - `contracts/vlayer/foundry.toml`

```
risc0-ethereum = { version = "2.0.0", url = "https://github.com/vlayer-xyz/risc0-ethereum/releases/download/v2.0.0-soldeer/contracts.zip" }
```

Note that the url should point to the zipped contracts we just created a GitHub release for in `risc0-ethereum` repo.

1.8.3. Update remappings - `contracts/vlayer/remappings.txt` and `contracts/fixtures/remappings.txt`

```
risc0-ethereum-2.0.0/=../vlayer/dependencies/risc0-ethereum-2.0.0/
```

1.8.4. Update vlayer-init config - `bash/lib/e2e.sh` and `rust/cli/config.toml`

```bash
[sol-dependencies.risc0-ethereum]
version = '2.0.0'
url = "https://github.com/vlayer-xyz/risc0-ethereum/releases/download/v2.0.0-soldeer/contracts.zip"
remappings = [["risc0-ethereum-2.0.0/", "dependencies/risc0-ethereum-2.0.0/"]]
```

1.8.5. Update imports in contracts

Change any `risc0-ethereum` import to the correct version.

1.8.6. Update Groth16 verifier selector - `contracts/vlayer/test/helpers/Groth16VerifierSelector.sol` and `rust/services/call/seal/src/lib.rs`

The new selector value can be extracted from [risc0-ethereum/contracts/src/selector.rs](https://github.com/vlayer-xyz/risc0-ethereum/blob/release-2.0-soldeer/contracts/src/selector.rs)
and in particular, from the `enum Selector`. For example, v2.0 selector is tagged as `Selector::Groth16V2_0 = 0x9f39696c`.

Having noted this value down, we can now update the contracts and the seal:

```solidity
library Groth16VerifierSelector {
    // value ensures that versions of risc0-ethereum and risc0-zkvm deps are compatible
    // must be kept in-sync with GROTH16_VERIFIER_SELECTOR value in rust/services/call/seal/src/lib.rs
    bytes4 public constant STABLE_VERIFIER_SELECTOR = bytes4(0x9f39696c);
}
```

```rust
    // stable, expected selector by solidity groth16 verifiers
    // must be kept in sync with value from `contracts/vlayer/test/helpers/Groth16VerifierSelector.sol`
    const GROTH16_VERIFIER_SELECTOR: VerifierSelector = VerifierSelector([0x9f, 0x39, 0x69, 0x6c]);
```

1.8.7. Fix Solidity integration tests - `contracts/vlayer/test/integration/ProofVerifier.t.sol`

Values that will require updating are:

* `FIXED_CALL_GUEST_ID`
* `FIXED_GROTH16_SETTLE_BLOCK_HASH`
* `FIXED_FAKE_SETTLE_BLOCK_HASH`
* `sealBytes` in `fakeProofFixture()`
* `sealBytes` in `groth16ProofFixture`

Obtaining those new values requires running `e2e-test.sh` on `simple` example and copying the values from the generated proof.

* Fake values

Run the following e2e test:

```sh
$ EXAMPLE=simple ./bash/e2e-test.sh
```

The values we are looking for can be matched in the output using the following blueprint:

```sh
Proof result: [
  {
    seal: {
      verifierSelector: "0xdeafbeef",
      seal: [ "sealBytes[0]",
        "0x0000000000000000000000000000000000000000000000000000000000000000", "0x0000000000000000000000000000000000000000000000000000000000000000",
        "0x0000000000000000000000000000000000000000000000000000000000000000", "0x0000000000000000000000000000000000000000000000000000000000000000",
        "0x0000000000000000000000000000000000000000000000000000000000000000", "0x0000000000000000000000000000000000000000000000000000000000000000",
        "0x0000000000000000000000000000000000000000000000000000000000000000"
      ],
      mode: 1,
    },
    callGuestId: "FIXED_CALL_GUEST_ID",
    length: 768,
    callAssumptions: {
      proverContractAddress: "0x9fe46736679d2d9a65f0992f2272de9f3c7fa6e0",
      functionSelector: "0xe3d670d7",
      settleChainId: "0x7a69",
      settleBlockNumber: "0x6",
      settleBlockHash: "FIXED_FAKE_SETTLE_BLOCK_HASH",
    },
  },
  "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266", 10000000n
]
```

* Groth16 values

Tweak `bash/run_services/lib.sh` so that `startup_vlayer()` always runs using remote Groth16 (Bonsai) prover:

```bash
function startup_vlayer() {
  local proof_arg="groth16";
  # ...
}
```

Next, execute e2e test with Bonsai credentials:

```sh
$ EXAMPLE=simple BONSAI_API_URL=https://api.bonsai.xyz BONSAI_API_KEY=... ./bash/e2e-test.sh
```

The values we are looking for can be matched as follows from the output:

```sh
Proof result: [
  {
    seal: {
      verifierSelector: "0x9f39696c",
      seal: [ "sealBytes[0]",
        "sealBytes[1]", "sealBytes[2]",
        "sealBytes[3]", "sealBytes[4]",
        "sealBytes[4]", "sealBytes[5]",
        "sealBytes[6]"
      ],
      mode: 0,
    },
    callGuestId: "FIXED_CALL_GUEST_ID",
    length: 768,
    callAssumptions: {
      proverContractAddress: "0x9fe46736679d2d9a65f0992f2272de9f3c7fa6e0",
      functionSelector: "0xe3d670d7",
      settleChainId: "0x7a69",
      settleBlockNumber: "0x6",
      settleBlockHash: "FIXED_GROTH16_SETTLE_BLOCK_HASH",
    },
  },
  "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266", 10000000n
]
```

## Generate new Guest ID using Docker

Make sure all previous steps have been performed successfully.

1. Generate new GuestID using Docker

```sh
$ UPDATE_GUEST_ELF_ID=1 RISC0_USE_DOCKER=1 cargo build
```

Go and get yourself a cup of coffee or something - this *will* take a while.

2. Update the CHANGELOG with the name of the change - `rust/guest_wrapper/artifacts/chain_guest/CHANGELOG.md`

```
# ...
* `<a_new_guest_id_value> - TODO`
```

replace `TODO` with an actual reason for an upgrade.
