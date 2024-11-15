// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std-1.9.2/src/Test.sol";

import {RiscZeroMockVerifier} from "risc0-ethereum-1.0.0/src/test/RiscZeroMockVerifier.sol";

import {FakeProofVerifier, FAKE_VERIFIER_SELECTOR} from "../../src/proof_verifier/FakeProofVerifier.sol";
import {ImageID} from "../../src/ImageID.sol";
import {ProofMode} from "../../src/Seal.sol";

contract FakeProofVerifier_Tests is Test {
    FakeProofVerifier verifier = new FakeProofVerifier();

    function test_usesFakeProofMode() public view {
        assert(verifier.PROOF_MODE() == ProofMode.FAKE);
    }

    function test_usesProperImageId() public view {
        assert(verifier.CALL_GUEST_ID() == ImageID.RISC0_CALL_GUEST_ID);
    }

    function test_usesMockRiscZeroVerifier() public {
        RiscZeroMockVerifier mockVerifier = new RiscZeroMockVerifier(FAKE_VERIFIER_SELECTOR);

        assertEq(address(verifier.VERIFIER()).codehash, address(mockVerifier).codehash);
    }

    function test_cannotBeCreatedOnMainnet() public {
        vm.chainId(1);

        vm.expectRevert();
        new FakeProofVerifier();
    }
}
