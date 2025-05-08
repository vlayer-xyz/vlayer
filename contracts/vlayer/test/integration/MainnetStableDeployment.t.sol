// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Test} from "forge-std-1.9.4/src/Test.sol";
import {Create2} from "@openzeppelin-contracts-5.0.1/utils/Create2.sol";

import {VLAYER_STABLE_SALT} from "../../script/MainnetVlayerDeployer.s.sol";

import {Repository} from "../../src/Repository.sol";
import {Groth16ProofVerifier} from "../../src/proof_verifier/Groth16ProofVerifier.sol";

import {MainnetStableDeployment} from "../../src/MainnetStableDeployment.sol";

contract StableMainnetDeployment_Tests is Test {
    address public constant INITIAL_ADMIN = address(0x0BE0eE404E921DE557b095872c4470a6a082180f);
    address public constant INITIAL_OWNER = address(0x07A733809BD4a30a3425104EC67e30d37E79b60A);
    address public constant CREATE2_DEPLOYER_CONTRACT = address(0x4e59b44847b379578588920cA78FbF26c0B4956C);

    function test_repositoryAddressIsStable() public pure {
        Repository repository = MainnetStableDeployment.repository();

        bytes memory bytecode =
            abi.encodePacked(type(Repository).creationCode, abi.encode(INITIAL_ADMIN, INITIAL_OWNER));
        bytes32 bytecodeHash = keccak256(bytecode);

        address computedAddress = Create2.computeAddress(VLAYER_STABLE_SALT, bytecodeHash, CREATE2_DEPLOYER_CONTRACT);
        assertEq(computedAddress, address(repository));
    }

    function test_groth16ProofVerifierAddressIsStable() public pure {
        Repository repository = MainnetStableDeployment.repository();
        (Groth16ProofVerifier groth16ProofVerifier) = MainnetStableDeployment.verifiers();

        bytes memory bytecode = abi.encodePacked(type(Groth16ProofVerifier).creationCode, abi.encode(repository));
        bytes32 bytecodeHash = keccak256(bytecode);

        address computedAddress = Create2.computeAddress(VLAYER_STABLE_SALT, bytecodeHash, CREATE2_DEPLOYER_CONTRACT);
        assertEq(computedAddress, address(groth16ProofVerifier));
    }
}
