// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {IRiscZeroVerifier} from "risc0-ethereum/IRiscZeroVerifier.sol";
import "forge-std/console.sol";
import {Proof, ProofLib} from "./Proof.sol";
import {Seal, SealLib} from "./Seal.sol";

import {IProofVerifier} from "./proof_verifier/IProofVerifier.sol";
import {ProofVerifierFactory} from "./proof_verifier/ProofVerifierFactory.sol";

abstract contract Verifier {
    uint256 constant SELECTOR_LEN = 4;
    uint256 constant PROOF_OFFSET = SELECTOR_LEN;
    uint256 constant JOURNAL_OFFSET = PROOF_OFFSET + ProofLib.COMMITMENT_OFFSET;

    IProofVerifier public verifier;

    constructor() {
        verifier = ProofVerifierFactory.produce();
    }

    modifier onlyVerified(address prover, bytes4 selector) {
        _verify(prover, selector);
        _;
    }

    function _verify(address prover, bytes4 selector) internal view {
        (Proof memory proof, bytes32 journalHash) = _decodeCalldata();
        verifier.verify(proof, journalHash, prover, selector);
    }

    function _decodeCalldata() private pure returns (Proof memory, bytes32) {
        Proof memory proof = abi.decode(msg.data[4:], (Proof));

        uint256 journalEnd = JOURNAL_OFFSET + proof.length;
        bytes memory journal = msg.data[JOURNAL_OFFSET:journalEnd];
        console.logBytes(journal);
        bytes32 journalHash = sha256(journal);

        return (proof, journalHash);
    }
}
