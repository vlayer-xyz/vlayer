// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Seal, SealLib} from "../../src/Seal.sol";

library TestHelpers {
    function encodeSeal(bytes calldata seal) public pure returns (Seal memory) {
        require(seal.length == SealLib.SEAL_LENGTH, "Invalid seal length");

        uint256 lhv = 0;
        uint256 rhv = 0;

        for (uint256 i = 0; i < SealLib.SEAL_MIDDLE; i++) {
            lhv <<= 8;
            lhv += uint8(seal[i]);
            rhv <<= 8;
            rhv += uint8(seal[i + SealLib.SEAL_MIDDLE]);
        }

        // shift value to most significant bytes
        lhv <<= 8 * (32 - SealLib.SEAL_MIDDLE);
        rhv <<= 8 * (32 - SealLib.SEAL_MIDDLE);

        return Seal(bytes18(bytes32(lhv)), bytes19(bytes32(rhv)));
    }

    function concat(bytes calldata a, bytes calldata b) public pure returns (bytes memory) {
        bytes memory c = new bytes(a.length + b.length);

        for (uint256 i = 0; i < a.length; i++) {
            c[i] = a[i];
        }

        uint256 offset = a.length;
        for (uint256 i = 0; i < b.length; i++) {
            c[offset + i] = b[i];
        }

        return c;
    }
}
