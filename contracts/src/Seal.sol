// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

struct Seal {
    bytes18 lhv;
    bytes18 rhv;
}

library SealLib {
    uint256 constant SEAL_LENGTH = 36;
    uint256 constant SEAL_MIDDLE = SEAL_LENGTH / 2;

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
}
