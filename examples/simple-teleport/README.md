# Simple Teleport

## Overview

`simple-teleport` example demonstrates teleport from L1 to L2. During execution, it runs the following scripts:

## Scripts

### 1. `loadFixtures`

- Deploys the `L2State`, `MockERC20`, and cross-chain prover contracts.
- Mints and transfers `L2ERC20` tokens.
- Computes and commits the `outputRoot` to L1 in the `L2State` contract.

### 2. `prove`

- Generates a proof of cross-chain `ERC-20` balance for `TOKEN_HOLDER`.
- Submits the proof to `SimpleTeleportVerifier` for verification on L1.
- Confirms the proof is valid.

## Blockchain Transactions

### L2

Chain id: 31338

The L2 state is modified only in the `loadFixtures` script, and all L2 updates are completed before any updates on L1.

| Block # | Action                                      |
| ------- | ------------------------------------------- |
| 1       | Deploy `MockERC20` `ERC-20` token contract. |
| 2       | Mint 1000 `L2ERC20` tokens for opAccount.   |
| 3       | Transfer 100 `L2ERC20` to `TOKEN_HOLDER`.   |

### L1

Chain id: 31337

| Block # | Action                                                    |
| ------- | --------------------------------------------------------- |
| 1       | Deploy `L2State` with `outputRoot`.                       |
| 2       | Deploy `WhaleBadgeNFT` contract.                          |
| 3       | Deploy `SimpleTeleportProver` & `SimpleTeleportVerifier`. |
| 4       | Generate proof for `TOKEN_HOLDER`'s cross-chain balance.  |
| 5       | Verify proof on `SimpleTeleportVerifier`.                 |

## Addresses Used

| Contract                   | Address                                      | Chain |
| -------------------------- | -------------------------------------------- | ----- |
| **MockERC20**              | `0xda52b25ddB0e3B9CC393b0690Ac62245Ac772527` | L2    |
| **Token holder**           | `0xe2148eE53c0755215Df69b2616E552154EdC584f` | L2    |
| **L2State**                | `0x5FbDB2315678afecb367f032d93F642f64180aa3` | L1    |
| **WhaleBadgeNFT**          | `0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512` | L1    |
| **SimpleTeleportProver**   | `0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0` | L1    |
| **SimpleTeleportVerifier** | `0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9` | L1    |
