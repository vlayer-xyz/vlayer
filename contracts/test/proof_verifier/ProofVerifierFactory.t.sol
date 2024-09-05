// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std-1.9.2/src/Test.sol";

import {IProofVerifier} from "../../src/proof_verifier/IProofVerifier.sol";
import {ProofVerifierFactory, InvalidChainId} from "../../src/proof_verifier/ProofVerifierFactory.sol";

contract VerifierFactory_Tests is Test {
    function test_producesAVerifierInDevMode() public {
        vm.chainId(31337);

        IProofVerifier verifier = ProofVerifierFactory.produce();
        assert(verifier != IProofVerifier(address(0)));
    }

    function test_returnsAConstantForMainnets() public {
        vm.chainId(1);

        IProofVerifier verifier = ProofVerifierFactory.produce();
        assert(verifier == IProofVerifier(address(0)));
    }

    function test_failsForOtherNetworks() public {
        vm.chainId(11155111 + 1);

        vm.expectRevert(InvalidChainId.selector);
        ProofVerifierFactory.produce();
    }
}
