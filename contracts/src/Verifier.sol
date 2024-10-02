// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Proof, ProofLib, MAX_NUMBER_OF_DYNAMIC_PARAMS} from "./Proof.sol";

import {IProofVerifier} from "./proof_verifier/IProofVerifier.sol";
import {ProofVerifierFactory} from "./proof_verifier/ProofVerifierFactory.sol";
import {CallAssumptionsLib} from "./CallAssumptions.sol";

abstract contract Verifier {
    uint256 private constant SELECTOR_LEN = 4;
    uint256 private constant PROOF_OFFSET = SELECTOR_LEN;
    uint256 private constant JOURNAL_OFFSET = PROOF_OFFSET + ProofLib.CALL_ASSUMPTIONS_OFFSET;

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
        Proof memory proof = abi.decode(msg.data[PROOF_OFFSET:], (Proof));

        uint256 journalEnd = JOURNAL_OFFSET + proof.length;
        bytes memory journal = msg.data[JOURNAL_OFFSET:journalEnd];

        for (uint256 i = 0; i < MAX_NUMBER_OF_DYNAMIC_PARAMS; i++) {
            if (proof.dynamicParamsOffsets[i] > 0) {
                journal = shiftOffset(journal, ProofLib.PROOF_ENCODING_LENGTH, proof.dynamicParamsOffsets[i]);
            }
        }
        bytes32 journalHash = sha256(journal);
        return (proof, journalHash);
    }

    function shiftOffset(bytes memory data, uint256 shiftBy, uint256 offsetPosition)
        public
        pure
        returns (bytes memory)
    {
        uint256 offsetPositionRelativeToJournal = CallAssumptionsLib.CALL_ASSUMPTIONS_ENCODING_LENGTH + offsetPosition;

        require(data.length >= offsetPositionRelativeToJournal, "Encoded data too short");

        uint256 dataOffset;
        assembly {
            dataOffset := mload(add(data, offsetPositionRelativeToJournal))
        }

        uint256 shiftedOffset = dataOffset - shiftBy;

        bytes memory dataCopy = data;

        assembly {
            mstore(add(dataCopy, offsetPositionRelativeToJournal), shiftedOffset)
        }

        return dataCopy;
    }
}
