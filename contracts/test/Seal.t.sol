// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {TestHelpers} from "./helpers/TestHelpers.sol";

import {Seal, ProofMode, SealLib} from "../src/Seal.sol";

contract SealLib_decode_Tests is Test {
    using SealLib for Seal;

    function test_empty() public pure {
        Seal memory seal =
            Seal(0x000000000000000000000000000000000000, 0x00000000000000000000000000000000000000, ProofMode.GROTH16);
        assertEq(seal.decode(), SealFixtures.EMPTY());
    }

    function test_max() public pure {
        // SEAL_MAX = ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
        Seal memory seal =
            Seal(0xffffffffffffffffffffffffffffffffffff, 0xffffffffffffffffffffffffffffffffffffff, ProofMode.GROTH16);
        assertEq(seal.decode(), SealFixtures.MAX());
    }

    function test_sealIsDecodedFromLeftToRight() public pure {
        // SEAL_SEQ = 000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f2021222324
        Seal memory seal =
            Seal(0x0102030405060708090a0b0c0d0e0f101112, 0x131415161718191a1b1c1d1e1f202122232400, ProofMode.GROTH16);

        assertEq(seal.decode(), SealFixtures.SEQ());
    }

    function test_immutability() public pure {
        bytes18 lhv = 0x0102030405060708090a0b0c0d0e0f101112;
        bytes19 rhv = 0x131415161718191a1b1c1d1e1f202122232400;
        Seal memory seal = Seal(lhv, rhv, ProofMode.GROTH16);

        seal.decode();

        assertEq(seal.lhv, lhv);
        assertEq(seal.rhv, rhv);
    }
}

contract SealLib_proofMode_Tests is Test {
    using SealLib for Seal;

    uint8 constant PROOF_MODE_ENC_LOCATION = 3 * 32 - 1;

    function test_sealModeIsEncodedOn_64th_byte() public pure {
        Seal memory sealGroth16 =
            Seal(0x0102030405060708090a0b0c0d0e0f101112, 0x131415161718191a1b1c1d1e1f202122232400, ProofMode.GROTH16);
        Seal memory sealFake =
            Seal(0x0102030405060708090a0b0c0d0e0f101112, 0x131415161718191a1b1c1d1e1f202122232400, ProofMode.FAKE);

        bytes memory encodedGroth16 = abi.encode(sealGroth16);
        bytes memory encodedFake = abi.encode(sealFake);

        assertEq(encodedGroth16[PROOF_MODE_ENC_LOCATION], bytes1(uint8(ProofMode.GROTH16)));
        assertEq(encodedFake[PROOF_MODE_ENC_LOCATION], bytes1(uint8(ProofMode.FAKE)));
    }

    function test_proofMode() public pure {
        Seal memory sealGroth16 =
            Seal(0x0102030405060708090a0b0c0d0e0f101112, 0x131415161718191a1b1c1d1e1f202122232400, ProofMode.GROTH16);
        Seal memory sealFake =
            Seal(0x0102030405060708090a0b0c0d0e0f101112, 0x131415161718191a1b1c1d1e1f202122232400, ProofMode.FAKE);

        assert(sealGroth16.proofMode() == ProofMode.GROTH16);
        assert(sealFake.proofMode() == ProofMode.FAKE);
    }

    function test_invalidValueReverts() public {
        Seal memory seal =
            Seal(0x0102030405060708090a0b0c0d0e0f101112, 0x131415161718191a1b1c1d1e1f202122232400, ProofMode.GROTH16);
        bytes memory encoded = abi.encode(seal);
        encoded[PROOF_MODE_ENC_LOCATION] = bytes1(0x02);

        vm.expectRevert(); // should panic with 0x21
        abi.decode(encoded, (Seal));
    }

    function test_immutability() public pure {
        bytes18 lhv = 0x0102030405060708090a0b0c0d0e0f101112;
        bytes19 rhv = 0x131415161718191a1b1c1d1e1f202122232401;
        Seal memory seal = Seal(lhv, rhv, ProofMode.FAKE);

        seal.proofMode();

        assertEq(seal.lhv, lhv);
        assertEq(seal.rhv, rhv);
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
