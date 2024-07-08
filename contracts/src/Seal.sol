// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

struct Seal {
    bytes18 lhv;
    bytes18 rhv;
}

library SealLib {
    function encodeSeal(bytes calldata seal) public pure returns (Seal memory) {
        require(seal.length == 36, "Invalid seal length");
        uint256 lhv = 0;
        uint256 rhv = 0;
        for (uint256 i = 0; i < 18; i++) {
            lhv <<= 8;
            lhv += uint8(seal[i]);
            rhv <<= 8;
            rhv += uint8(seal[i + 18]);
        }
        return Seal(bytes18(bytes32(lhv)), bytes18(bytes32(rhv)));
    }
}
