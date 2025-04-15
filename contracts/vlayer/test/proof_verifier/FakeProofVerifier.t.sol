// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Test, console} from "forge-std-1.9.4/src/Test.sol";

import {RiscZeroMockVerifier} from "risc0-ethereum-2.0.0/src/test/RiscZeroMockVerifier.sol";

import {InvalidChainId} from "../../src/proof_verifier/ChainId.sol";

import {FakeProofVerifier, FAKE_VERIFIER_SELECTOR} from "../../src/proof_verifier/FakeProofVerifier.sol";
import {Repository} from "../../src/Repository.sol";
import {ImageID} from "../../src/ImageID.sol";
import {ProofMode} from "../../src/Seal.sol";

import {TestDeployer} from "../helpers/TestDeployer.sol";

contract FakeProofVerifier_Tests is Test {
    TestDeployer testDeployer = new TestDeployer();
    FakeProofVerifier immutable verifier;

    constructor() {
        verifier = testDeployer.fakeProofVerifier();
    }

    function test_usesFakeProofMode() public view {
        assert(verifier.PROOF_MODE() == ProofMode.FAKE);
    }

    function test_usesMockRiscZeroVerifier() public {
        RiscZeroMockVerifier mockVerifier = new RiscZeroMockVerifier(FAKE_VERIFIER_SELECTOR);

        assertEq(address(verifier.VERIFIER()).codehash, address(mockVerifier).codehash);
    }

    function test_cannotBeCreatedOnMainnet() public {
        vm.chainId(1);
        Repository repository = testDeployer.repository();

        vm.expectRevert(InvalidChainId.selector);
        new FakeProofVerifier(repository);
    }
}
