// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Test} from "forge-std-1.9.4/src/Test.sol";
import {Create2} from "@openzeppelin-contracts-5.0.1/utils/Create2.sol";

import {VLAYER_STABLE_SALT} from "../../script/TestnetVlayerDeployer.s.sol";

import {Repository} from "../../src/Repository.sol";
import {FakeProofVerifier} from "../../src/proof_verifier/FakeProofVerifier.sol";
import {Groth16ProofVerifier} from "../../src/proof_verifier/Groth16ProofVerifier.sol";
import {ProofVerifierRouter} from "../../src/proof_verifier/ProofVerifierRouter.sol";

import {TestnetStableDeployment} from "../../src/TestnetStableDeployment.sol";

contract StableTestDeployment_Tests is Test {
    address public constant INITIAL_ADMIN = address(0xAeb4F991499dDC040d28653b42209e1eA6E8c151);
    address public constant INITIAL_OWNER = address(0xF52c0F73B4BceA4a13125197Eba625Dc65996441);
    address public constant CREATE2_DEPLOYER_CONTRACT = address(0x4e59b44847b379578588920cA78FbF26c0B4956C);

    function test_repositoryAddressIsStable() public pure {
        Repository repository = TestnetStableDeployment.repository();

        bytes memory bytecode =
            abi.encodePacked(type(Repository).creationCode, abi.encode(INITIAL_ADMIN, INITIAL_OWNER));
        bytes32 bytecodeHash = keccak256(bytecode);

        address computedAddress = Create2.computeAddress(VLAYER_STABLE_SALT, bytecodeHash, CREATE2_DEPLOYER_CONTRACT);
        assertEq(computedAddress, address(repository));
    }

    function test_FakeProofVerifierAddressIsStable() public pure {
        Repository repository = TestnetStableDeployment.repository();
        (FakeProofVerifier fakeProofVerifier,,) = TestnetStableDeployment.verifiers();

        bytes memory bytecode = abi.encodePacked(type(FakeProofVerifier).creationCode, abi.encode(repository));
        bytes32 bytecodeHash = keccak256(bytecode);

        address computedAddress = Create2.computeAddress(VLAYER_STABLE_SALT, bytecodeHash, CREATE2_DEPLOYER_CONTRACT);
        assertEq(computedAddress, address(fakeProofVerifier));
    }

    function test_groth16ProofVerifierAddressIsStable() public pure {
        Repository repository = TestnetStableDeployment.repository();
        (, Groth16ProofVerifier groth16ProofVerifier,) = TestnetStableDeployment.verifiers();

        bytes memory bytecode = abi.encodePacked(type(Groth16ProofVerifier).creationCode, abi.encode(repository));
        bytes32 bytecodeHash = keccak256(bytecode);

        address computedAddress = Create2.computeAddress(VLAYER_STABLE_SALT, bytecodeHash, CREATE2_DEPLOYER_CONTRACT);
        assertEq(computedAddress, address(groth16ProofVerifier));
    }

    function test_proofVerifierRouterIsStable() public pure {
        (FakeProofVerifier fakeProofVerifier, Groth16ProofVerifier groth16ProofVerifier, ProofVerifierRouter router) =
            TestnetStableDeployment.verifiers();

        bytes memory bytecode = abi.encodePacked(
            type(ProofVerifierRouter).creationCode, abi.encode(fakeProofVerifier, groth16ProofVerifier)
        );

        bytes32 bytecodeHash = keccak256(bytecode);

        address computedAddress = Create2.computeAddress(VLAYER_STABLE_SALT, bytecodeHash, CREATE2_DEPLOYER_CONTRACT);
        assertEq(computedAddress, address(router));
    }
}
