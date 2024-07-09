// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

struct Seal {
    bytes18 lhv;
    bytes18 rhv;
}

library SealLib {
    uint256 constant SEAL_LENGTH = 36;
    uint256 constant SEAL_MIDDLE = SEAL_LENGTH / 2;

    function encodeSeal(bytes calldata seal) public pure returns (Seal memory) {
        require(seal.length == SEAL_LENGTH, "Invalid seal length");
        uint256 lhv = 0;
        uint256 rhv = 0;
        for (uint256 i = 0; i < SEAL_MIDDLE; i++) {
            lhv <<= 8;
            lhv += uint8(seal[i]);
            rhv <<= 8;
            rhv += uint8(seal[i + SEAL_MIDDLE]);
        }
        lhv <<= 8 * (32-SEAL_MIDDLE); // shift value to most significant bytes
        rhv <<= 8 * (32-SEAL_MIDDLE);

        return Seal(bytes18(bytes32(lhv)), bytes18(bytes32(rhv)));
    }

    function decode(Seal memory seal) public pure returns (bytes memory) {
        
        bytes memory sealBytes = new bytes(SEAL_LENGTH);

        for (uint256 i = 0; i < SEAL_MIDDLE; i++) {
            sealBytes[i] = bytes1(seal.lhv); 
            sealBytes[SEAL_MIDDLE+i] = bytes1(seal.rhv); 

            seal.lhv <<= 8;
            seal.rhv <<= 8;
        }

        return sealBytes;
    }
}
