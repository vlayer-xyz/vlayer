// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Test} from "forge-std-1.9.4/src/Test.sol";
import {Create2} from "@openzeppelin-contracts-5.0.1/utils/Create2.sol";

import {VLAYER_STABLE_SALT} from "../../script/VerifierDeployer.s.sol";

import {Repository} from "../../src/Repository.sol";
import {FakeProofVerifier} from "../../src/proof_verifier/FakeProofVerifier.sol";
import {Groth16ProofVerifier} from "../../src/proof_verifier/Groth16ProofVerifier.sol";
import {ProofVerifierRouter} from "../../src/proof_verifier/ProofVerifierRouter.sol";

import {ProofVerifierFactory} from "../../src/proof_verifier/ProofVerifierFactory.sol";

contract StableTestDeployment_Tests is Test {
    address public constant INITIAL_ADMIN = address(0xAeb4F991499dDC040d28653b42209e1eA6E8c151);
    address public constant CREATE2_DEPLOYER_CONTRACT = address(0x4e59b44847b379578588920cA78FbF26c0B4956C);

    function test_repositoryAddressIsStable() public {
        vm.skip(true);
        (Repository repository,,,) = ProofVerifierFactory.testnetStableDeployment();

        bytes memory bytecode =
            abi.encodePacked(type(Repository).creationCode, abi.encode(INITIAL_ADMIN, INITIAL_ADMIN));
        bytes32 bytecodeHash = keccak256(bytecode);

        address computedAddress = Create2.computeAddress(VLAYER_STABLE_SALT, bytecodeHash, CREATE2_DEPLOYER_CONTRACT);
        assertEq(computedAddress, address(repository));
    }

    function test_FakeProofVerifierAddressIsStable() public {
        vm.skip(true);
        (Repository repository, FakeProofVerifier fakeProofVerifier,,) = ProofVerifierFactory.testnetStableDeployment();

        bytes memory bytecode = abi.encodePacked(type(FakeProofVerifier).creationCode, abi.encode(repository));
        bytes32 bytecodeHash = keccak256(bytecode);

        address computedAddress = Create2.computeAddress(VLAYER_STABLE_SALT, bytecodeHash, CREATE2_DEPLOYER_CONTRACT);
        assertEq(computedAddress, address(fakeProofVerifier));
    }

    function test_groth16ProofVerifierAddressIsStable() public {
        vm.skip(true);
        (Repository repository,, Groth16ProofVerifier groth16ProofVerifier,) =
            ProofVerifierFactory.testnetStableDeployment();

        bytes memory bytecode = abi.encodePacked(type(Groth16ProofVerifier).creationCode, abi.encode(repository));
        bytes32 bytecodeHash = keccak256(bytecode);

        address computedAddress = Create2.computeAddress(VLAYER_STABLE_SALT, bytecodeHash, CREATE2_DEPLOYER_CONTRACT);
        assertEq(computedAddress, address(groth16ProofVerifier));
    }

    function test_proofVerifiierRouterIsStable() public {
        vm.skip(true);
        (, FakeProofVerifier fakeProofVerifier, Groth16ProofVerifier groth16ProofVerifier, ProofVerifierRouter router) =
            ProofVerifierFactory.testnetStableDeployment();

        bytes memory bytecode = abi.encodePacked(
            type(ProofVerifierRouter).creationCode, abi.encode(fakeProofVerifier, groth16ProofVerifier)
        );

        bytes32 bytecodeHash = keccak256(bytecode);

        address computedAddress = Create2.computeAddress(VLAYER_STABLE_SALT, bytecodeHash, CREATE2_DEPLOYER_CONTRACT);
        assertEq(computedAddress, address(router));
    }
}
