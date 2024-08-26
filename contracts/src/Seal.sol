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
<<<<<<< HEAD
    uint256 constant FAKE_SEAL_LENGTH = 36;
    uint256 constant GROTH16_SEAL_LENGTH = 256;
    uint256 constant SEAL_LENGTH = 256;
=======
    uint256 private constant SEAL_LENGTH = 36;
    uint256 private constant SEAL_MIDDLE = SEAL_LENGTH / 2;
    uint256 private constant PROOF_MODE_POSITION = SEAL_LENGTH + 1;
>>>>>>> 28d27156 (lint solidity in /contracts/src)

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
