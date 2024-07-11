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

    function decode(Seal memory seal) public pure returns (bytes memory) {
        bytes memory sealBytes = new bytes(SEAL_LENGTH);

        for (uint256 i = 0; i < SEAL_MIDDLE; i++) {
            sealBytes[i] = bytes1(seal.lhv);
            sealBytes[SEAL_MIDDLE + i] = bytes1(seal.rhv);

            seal.lhv <<= 8;
            seal.rhv <<= 8;
        }

        return sealBytes;
    }

    function proofMode(Seal memory seal) public pure returns (ProofMode) {
        bytes1 proofModeByte = bytes1(seal.rhv << 8 * (PROOF_MODE_POSITION - SEAL_MIDDLE - 1));
        return ProofMode(uint8(proofModeByte));
    }
}
