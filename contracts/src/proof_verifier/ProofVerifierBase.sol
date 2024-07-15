// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {IRiscZeroVerifier} from "risc0-ethereum/IRiscZeroVerifier.sol";

import {Proof} from "../Proof.sol";
import {SealLib, Seal} from "../Seal.sol";

import {IProofVerifier} from "./IProofVerifier.sol";

abstract contract ProofVerifierBase is IProofVerifier {
    using SealLib for Seal;

    bytes32 public GUEST_ID = bytes32(0xb7079f57c71b4e1d95b8b1254303e13f78914599a8c119534c4c947c996b4d7d);
    uint256 constant AVAILABLE_HISTORICAL_BLOCKS = 256;

    IRiscZeroVerifier public verifier;

    function verify(Proof calldata proof, bytes32 journalHash, address expectedProver, bytes4 expectedSelector)
        external
        view
    {
        _verifyExecutionEnv(proof, expectedProver, expectedSelector);
        verifier.verify(proof.seal.decode(), GUEST_ID, journalHash);
    }

    function _verifyExecutionEnv(Proof memory proof, address prover, bytes4 selector) private view {
        require(proof.commitment.startContractAddress == prover, "Invalid prover");
        require(proof.commitment.functionSelector == selector, "Invalid selector");

        require(proof.commitment.settleBlockNumber < block.number, "Invalid block number: block from future");
        require(
            proof.commitment.settleBlockNumber + AVAILABLE_HISTORICAL_BLOCKS >= block.number,
            "Invalid block number: block too old"
        );

        require(proof.commitment.settleBlockHash == blockhash(proof.commitment.settleBlockNumber), "Invalid block hash");
    }
}
