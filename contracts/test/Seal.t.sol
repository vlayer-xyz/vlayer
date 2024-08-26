// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {TestHelpers} from "./helpers/TestHelpers.sol";

import {Seal, ProofMode, SealLib} from "../src/Seal.sol";

contract SealLib_decode_Tests is Test {
    using SealLib for Seal;

    function test_empty() public pure {
        // SEAL_EMPTY = 000...000
        Seal memory seal = Seal(SealFixtures.EMPTY_WORDS(), ProofMode.GROTH16);
        assertEq(seal.decode(), SealFixtures.EMPTY());
    }

    function test_max() public pure {
        // SEAL_MAX = fff...fff
        Seal memory seal = Seal(SealFixtures.MAX_WORDS(), ProofMode.GROTH16);
        assertEq(seal.decode(), SealFixtures.MAX());
    }

    function test_groth16ProofIs256BytesLong() public pure {
        Seal memory seal = Seal(SealFixtures.MAX_WORDS(), ProofMode.GROTH16);
        assertEq(seal.decode().length, 256);
    }

    function test_fakeProofIs36BytesLong() public pure {
        Seal memory seal = Seal(SealFixtures.MAX_WORDS(), ProofMode.FAKE);
        assertEq(seal.decode().length, 36);
    }
}

contract SealLib_proofMode_Tests is Test {
    using SealLib for Seal;

    uint32 constant PROOF_MODE_ENC_LOCATION = 256 + 32 - 1; // 288th byte, array index 287

    function test_proofMode() public pure {
        Seal memory sealGroth16 = Seal(SealFixtures.MAX_WORDS(), ProofMode.GROTH16);
        Seal memory sealFake = Seal(SealFixtures.MAX_WORDS(), ProofMode.FAKE);

        assert(sealGroth16.proofMode() == ProofMode.GROTH16);
        assert(sealFake.proofMode() == ProofMode.FAKE);
    }

    function test_proofModeIsEncodedOn288thByte() public pure {
        Seal memory sealGroth16 = Seal(SealFixtures.MAX_WORDS(), ProofMode.GROTH16);
        Seal memory sealFake = Seal(SealFixtures.MAX_WORDS(), ProofMode.FAKE);

        bytes memory encodedGroth16 = abi.encode(sealGroth16);
        bytes memory encodedFake = abi.encode(sealFake);

        assertEq(encodedGroth16[PROOF_MODE_ENC_LOCATION], bytes1(uint8(ProofMode.GROTH16)));
        assertEq(encodedFake[PROOF_MODE_ENC_LOCATION], bytes1(uint8(ProofMode.FAKE)));
    }

    function test_invalidValueReverts() public {
        Seal memory seal = Seal(SealFixtures.MAX_WORDS(), ProofMode.GROTH16);
        bytes memory encoded = abi.encode(seal);
        encoded[PROOF_MODE_ENC_LOCATION] = bytes1(0x02);

        vm.expectRevert(); // should panic with 0x21
        abi.decode(encoded, (Seal));
    }
}

library SealFixtures {
    function MAX_WORDS() public pure returns (bytes32[8] memory) {
        return WORDS(0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff);
    }

    function EMPTY_WORDS() public pure returns (bytes32[8] memory) {
        return WORDS(0x0000000000000000000000000000000000000000000000000000000000000000);
    }

    function WORDS(bytes32 word) public pure returns (bytes32[8] memory) {
        bytes32[8] memory words;
        for (uint256 i = 0; i < 8; i++) {
            words[i] = word;
        }
        return words;
    }

    function EMPTY() public pure returns (bytes memory) {
        return abi.encode(
            [
                0x0000000000000000000000000000000000000000000000000000000000000000,
                0x0000000000000000000000000000000000000000000000000000000000000000,
                0x0000000000000000000000000000000000000000000000000000000000000000,
                0x0000000000000000000000000000000000000000000000000000000000000000,
                0x0000000000000000000000000000000000000000000000000000000000000000,
                0x0000000000000000000000000000000000000000000000000000000000000000,
                0x0000000000000000000000000000000000000000000000000000000000000000,
                0x0000000000000000000000000000000000000000000000000000000000000000
            ]
        );
    }

    function MAX() public pure returns (bytes memory) {
        return abi.encode(
            [
                0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff,
                0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff,
                0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff,
                0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff,
                0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff,
                0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff,
                0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff,
                0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
            ]
        );
    }
}
