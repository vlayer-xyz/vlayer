// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {IRiscZeroVerifier} from "risc0-ethereum-1.2.0/src/IRiscZeroVerifier.sol";

import {Proof} from "../Proof.sol";
import {ProofMode, SealLib, Seal} from "../Seal.sol";

import {IProofVerifier} from "./IProofVerifier.sol";
import {IImageIdRepository} from "../Repository.sol";

abstract contract ProofVerifierBase is IProofVerifier {
    using SealLib for Seal;

    error InvalidProofMode(ProofMode expected, ProofMode actual);
    error InvalidProverAddress(address expected, address actual);
    error InvalidFunctionSelector(bytes4 expected, bytes4 actual);
    error SettlementBlockFromTheFuture(uint256 currentBlockNumber, uint256 settlementBlockNumber);
    error SettlementBlockFromTooOld(uint256 latestPossbileBlock, uint256 settlementBlockNumber);
    error InvalidBlockHash(uint256 blockNumber, bytes32 expectedBlockHash, bytes32 actualBlockHash);
    error UnsupportedCallGuestId(bytes32 callGuestId);

    uint256 private constant AVAILABLE_HISTORICAL_BLOCKS = 256;

    ProofMode public immutable PROOF_MODE;
    IRiscZeroVerifier public immutable VERIFIER;
    IImageIdRepository public immutable IMAGE_ID_REPOSITORY;

    constructor(IImageIdRepository _repository) {
        IMAGE_ID_REPOSITORY = _repository;
    }

    function imageIdRepository() external view returns (IImageIdRepository) {
        return IMAGE_ID_REPOSITORY;
    }

    function verify(Proof calldata proof, bytes32 journalHash, address expectedProver, bytes4 expectedSelector)
        external
        view
    {
        _verifyProofMode(proof);
        _verifyExecutionEnv(proof, expectedProver, expectedSelector);
        VERIFIER.verify(proof.seal.decode(), proof.callGuestId, journalHash);
    }

    function _verifyProofMode(Proof memory proof) private view {
        if (proof.seal.proofMode() != PROOF_MODE) {
            revert InvalidProofMode(PROOF_MODE, proof.seal.proofMode());
        }
    }

    function _verifyExecutionEnv(Proof memory proof, address prover, bytes4 selector) private view {
        if (proof.callAssumptions.proverContractAddress != prover) {
            revert InvalidProverAddress(prover, proof.callAssumptions.proverContractAddress);
        }
        if (proof.callAssumptions.functionSelector != selector) {
            revert InvalidFunctionSelector(selector, proof.callAssumptions.functionSelector);
        }

        if (proof.callAssumptions.settleBlockNumber >= block.number) {
            revert SettlementBlockFromTheFuture(block.number, proof.callAssumptions.settleBlockNumber);
        }
        if (proof.callAssumptions.settleBlockNumber + AVAILABLE_HISTORICAL_BLOCKS < block.number) {
            revert SettlementBlockFromTooOld(
                block.number >= AVAILABLE_HISTORICAL_BLOCKS ? block.number - AVAILABLE_HISTORICAL_BLOCKS : 0,
                proof.callAssumptions.settleBlockNumber
            );
        }

        if (proof.callAssumptions.settleBlockHash != blockhash(proof.callAssumptions.settleBlockNumber)) {
            revert InvalidBlockHash(
                proof.callAssumptions.settleBlockNumber,
                blockhash(proof.callAssumptions.settleBlockNumber),
                proof.callAssumptions.settleBlockHash
            );
        }

        if (!IMAGE_ID_REPOSITORY.isImageSupported(proof.callGuestId)) {
            revert UnsupportedCallGuestId(proof.callGuestId);
        }
    }
}
