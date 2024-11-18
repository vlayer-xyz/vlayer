// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {IRiscZeroVerifier} from "risc0-ethereum-1.0.0/src/IRiscZeroVerifier.sol";

import {ImageID} from "../ImageID.sol";
import {Proof} from "../Proof.sol";
import {ProofMode, SealLib, Seal} from "../Seal.sol";

import {IProofVerifier} from "./IProofVerifier.sol";

abstract contract ProofVerifierBase is IProofVerifier {
    using SealLib for Seal;

    uint256 private constant AVAILABLE_HISTORICAL_BLOCKS = 256;

    ProofMode public immutable PROOF_MODE;
    IRiscZeroVerifier public immutable VERIFIER;
    bytes32 public immutable CALL_GUEST_ID;

    constructor() {
        CALL_GUEST_ID = ImageID.RISC0_CALL_GUEST_ID;
    }

    function verify(Proof calldata proof, bytes32 journalHash, address expectedProver, bytes4 expectedSelector)
        external
        view
    {
        _verifyProofMode(proof);
        _verifyExecutionEnv(proof, expectedProver, expectedSelector);
        VERIFIER.verify(proof.seal.decode(), CALL_GUEST_ID, journalHash);
    }

    function call_guest_id() external view returns (bytes32) {
        return CALL_GUEST_ID;
    }

    function _verifyProofMode(Proof memory proof) private view {
        require(proof.seal.proofMode() == PROOF_MODE, "Invalid proof mode");
    }

    function _verifyExecutionEnv(Proof memory proof, address prover, bytes4 selector) private view {
        require(proof.callAssumptions.proverContractAddress == prover, "Invalid prover");
        require(proof.callAssumptions.functionSelector == selector, "Invalid selector");

        require(proof.callAssumptions.settleBlockNumber < block.number, "Invalid block number: block from future");
        require(
            proof.callAssumptions.settleBlockNumber + AVAILABLE_HISTORICAL_BLOCKS >= block.number,
            "Invalid block number: block too old"
        );

        require(
            proof.callAssumptions.settleBlockHash == blockhash(proof.callAssumptions.settleBlockNumber),
            "Invalid block hash"
        );

        // CALL_GUEST_ID is not a part of the verified arguments
        // and the following require is just to enable better error handling.
        require(proof.callGuestId == CALL_GUEST_ID, "CallGuestId mismatched");
    }
}
