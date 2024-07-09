// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {TestHelpers} from "./helpers/TestHelpers.sol";

import {Seal, SealLib} from "../src/Seal.sol";

contract SealLib_decodeTests is Test {
    using SealLib for Seal;

    function test_empty() public pure {
        Seal memory seal = Seal(0x000000000000000000000000000000000000, 0x000000000000000000000000000000000000);
        assertEq(seal.decode(), SealFixtures.EMPTY());
    }

    function test_max() public pure {
        // SEAL_MAX = ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
        Seal memory seal = Seal(0xffffffffffffffffffffffffffffffffffff, 0xffffffffffffffffffffffffffffffffffff);
        assertEq(seal.decode(), SealFixtures.MAX());
    }

    function test_sealIsDecodedFromLeftToRight() public pure {
        // SEAL_SEQ = 000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f2021222324
        Seal memory seal = Seal(0x0102030405060708090a0b0c0d0e0f101112, 0x131415161718191a1b1c1d1e1f2021222324);

        assertEq(seal.decode(), SealFixtures.SEQ());
    }
}

library SealFixtures {
    function EMPTY() public pure returns (bytes memory) {
        return to_bytes(
            [
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00
            ]
        );
    }

    function MAX() public pure returns (bytes memory) {
        return to_bytes(
            [
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff
            ]
        );
    }

    function SEQ() public pure returns (bytes memory) {
        return to_bytes(
            [
                0x01,
                0x02,
                0x03,
                0x04,
                0x05,
                0x06,
                0x07,
                0x08,
                0x09,
                0x0a,
                0x0b,
                0x0c,
                0x0d,
                0x0e,
                0x0f,
                0x10,
                0x11,
                0x12,
                0x13,
                0x14,
                0x15,
                0x16,
                0x17,
                0x18,
                0x19,
                0x1a,
                0x1b,
                0x1c,
                0x1d,
                0x1e,
                0x1f,
                0x20,
                0x21,
                0x22,
                0x23,
                0x24
            ]
        );
    }

    function to_bytes(uint8[36] memory arr) internal pure returns (bytes memory) {
        bytes memory r = new bytes(arr.length);

        for (uint32 i = 0; i < arr.length; i++) {
            r[i] = bytes1(arr[i]);
        }

        return r;
    }
}
