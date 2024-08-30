// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {IRiscZeroVerifier} from "risc0-ethereum/IRiscZeroVerifier.sol";

import {ImageID} from "../ImageID.sol";
import {Proof} from "../Proof.sol";
import {ProofMode, SealLib, Seal} from "../Seal.sol";

import {IProofVerifier} from "./IProofVerifier.sol";

abstract contract ProofVerifierBase is IProofVerifier {
    using SealLib for Seal;

    uint256 private constant AVAILABLE_HISTORICAL_BLOCKS = 256;

    ProofMode public immutable PROOF_MODE;
    IRiscZeroVerifier public immutable VERIFIER;

    function guest_id() external pure returns (bytes32) {
        return ImageID.RISC0_CALL_GUEST_ID;
    }

    function verify(Proof calldata proof, bytes32 journalHash, address expectedProver, bytes4 expectedSelector)
        external
        view
    {
        _verifyProofMode(proof);
        _verifyExecutionEnv(proof, expectedProver, expectedSelector);
        VERIFIER.verify(proof.seal.decode(), ImageID.RISC0_CALL_GUEST_ID, journalHash);
    }

    function _verifyProofMode(Proof memory proof) private view {
        require(proof.seal.proofMode() == PROOF_MODE, "Invalid proof mode");
    }

    function _verifyExecutionEnv(Proof memory proof, address prover, bytes4 selector) private view {
        require(proof.commitment.proverContractAddress == prover, "Invalid prover");
        require(proof.commitment.functionSelector == selector, "Invalid selector");

        require(proof.commitment.settleBlockNumber < block.number, "Invalid block number: block from future");
        require(
            proof.commitment.settleBlockNumber + AVAILABLE_HISTORICAL_BLOCKS >= block.number,
            "Invalid block number: block too old"
        );

        require(proof.commitment.settleBlockHash == blockhash(proof.commitment.settleBlockNumber), "Invalid block hash");
    }
}
