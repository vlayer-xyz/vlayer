// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

struct Seal {
    bytes18 lhv;
    bytes19 rhv;
}

enum ProofMode {
    GROTH16,
    FAKE
}

library SealLib {
    uint256 constant SEAL_LENGTH = 36;
    uint256 constant SEAL_MIDDLE = SEAL_LENGTH / 2;
    uint256 constant PROOF_MODE_POSITION = SEAL_LENGTH + 1;

    function decode(Seal memory seal) internal pure returns (bytes memory) {
        bytes memory sealBytes = new bytes(SEAL_LENGTH);

        // we don't want to change the original seal
        bytes18 lhv = seal.lhv;
        bytes19 rhv = seal.rhv;

        for (uint256 i = 0; i < SEAL_MIDDLE; i++) {
            sealBytes[i] = bytes1(lhv);
            sealBytes[SEAL_MIDDLE + i] = bytes1(rhv);

            lhv <<= 8;
            rhv <<= 8;
        }

        return sealBytes;
    }

    function proofMode(Seal memory seal) internal pure returns (ProofMode) {
        bytes1 proofModeByte = bytes1(seal.rhv << 8 * (PROOF_MODE_POSITION - SEAL_MIDDLE - 1));
        return ProofMode(uint8(proofModeByte));
    }
}
