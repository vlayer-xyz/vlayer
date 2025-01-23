// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

enum ProofMode {
    GROTH16,
    FAKE
}

struct Seal {
    bytes4 verifierSelector;
    bytes32[8] seal;
    ProofMode mode;
}

library SealLib {
    uint256 public constant ETH_WORD_SIZE = 32;

    uint256 public constant VERIFIER_SELECTOR_LENGTH = 4;
    uint256 public constant FAKE_SEAL_LENGTH = VERIFIER_SELECTOR_LENGTH + 32;
    uint256 public constant GROTH16_SEAL_LENGTH = VERIFIER_SELECTOR_LENGTH + 256;

    uint256 public constant VERIFIER_SELECTOR_ENCODING_LENGTH = ETH_WORD_SIZE;
    uint256 public constant SEAL_BYTES_ENCODING_LENGTH = 256;
    uint256 public constant PROOF_MODE_ENCODING_LENGTH = ETH_WORD_SIZE;

    uint256 public constant SEAL_ENCODING_LENGTH =
        VERIFIER_SELECTOR_ENCODING_LENGTH + SEAL_BYTES_ENCODING_LENGTH + ETH_WORD_SIZE;

    function decode(Seal memory seal) internal pure returns (bytes memory) {
        if (seal.mode == ProofMode.FAKE) {
            bytes32 firstWord = seal.seal[0];
            return abi.encodePacked(seal.verifierSelector, firstWord);
        } else {
            return abi.encodePacked(seal.verifierSelector, seal.seal);
        }
    }

    function proofMode(Seal memory seal) internal pure returns (ProofMode) {
        return seal.mode;
    }

    function verifierSelector(Seal memory seal) internal pure returns (bytes4) {
        return seal.verifierSelector;
    }
}
