// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

enum ProofMode {
    GROTH16,
    FAKE
}

struct Seal {
    bytes32[8] seal;
    ProofMode mode;
}

library SealLib {
    uint256 public constant VERIFIER_SELECTOR_LENGTH = 4;
    uint256 public constant FAKE_SEAL_LENGTH = VERIFIER_SELECTOR_LENGTH + 32;
    uint256 public constant GROTH16_SEAL_LENGTH = 256;
    uint256 public constant SEAL_ENCODING_LENGTH = 32 + 256 + 32;

    function decode(Seal memory seal) internal pure returns (bytes memory) {
        if (seal.mode == ProofMode.FAKE) {
            bytes32 firstWord = seal.seal[0];
            bytes4 secondWord = bytes4(seal.seal[1]);
            return abi.encodePacked(firstWord, secondWord);
        } else {
            return abi.encode(seal.seal);
        }
    }

    function proofMode(Seal memory seal) internal pure returns (ProofMode) {
        return seal.mode;
    }
}