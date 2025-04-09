// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Test} from "forge-std-1.9.4/src/Test.sol";
import {Create2} from "@openzeppelin-contracts-5.0.1/utils/Create2.sol";

import {VLAYER_STABLE_SALT} from "../../script/VlayerDeployer.s.sol";

import {Repository} from "../../src/Repository.sol";
import {FakeProofVerifier} from "../../src/proof_verifier/FakeProofVerifier.sol";
import {Groth16ProofVerifier} from "../../src/proof_verifier/Groth16ProofVerifier.sol";
import {ProofVerifierRouter} from "../../src/proof_verifier/ProofVerifierRouter.sol";

// import {TestnetStableDeployment} from "../../src/TestnetStableDeployment.sol";
library FutureTestnetStableDeployment {
    function repository() internal pure returns (Repository) {
        return Repository(address(0xCaba0E311bCB03B86dBb3ec5579f18C4dD411019));
    }

    function verifiers() internal pure returns (FakeProofVerifier, Groth16ProofVerifier, ProofVerifierRouter) {
        FakeProofVerifier fakeProofVerifier = FakeProofVerifier(address(0xc5bd546d48a64D54361a5D7cCd6FD0BB71117740));
        Groth16ProofVerifier groth16ProofVerifier =
            Groth16ProofVerifier(address(0xb577882cfDF3dD63aCDc378021941f4662db6400));
        ProofVerifierRouter proofVerifierRouter =
            ProofVerifierRouter(address(0x754Dd8fb59e4C963Af7649a4eBc8E0e4005aC95f));

        return (fakeProofVerifier, groth16ProofVerifier, proofVerifierRouter);
    }
}


contract StableTestDeployment_Tests is Test {
    address public constant INITIAL_ADMIN = address(0xAeb4F991499dDC040d28653b42209e1eA6E8c151);
    address public constant CREATE2_DEPLOYER_CONTRACT = address(0x4e59b44847b379578588920cA78FbF26c0B4956C);

    function test_repositoryAddressIsStable() public {
        Repository repository = FutureTestnetStableDeployment.repository();

        bytes memory bytecode =
            abi.encodePacked(type(Repository).creationCode, abi.encode(INITIAL_ADMIN, INITIAL_ADMIN));
        bytes32 bytecodeHash = keccak256(bytecode);

        address computedAddress = Create2.computeAddress(VLAYER_STABLE_SALT, bytecodeHash, CREATE2_DEPLOYER_CONTRACT);
        assertEq(computedAddress, address(repository));
    }

    function test_FakeProofVerifierAddressIsStable() public {
        Repository repository = FutureTestnetStableDeployment.repository();
        (FakeProofVerifier fakeProofVerifier,,) = FutureTestnetStableDeployment.verifiers();

        bytes memory bytecode = abi.encodePacked(type(FakeProofVerifier).creationCode, abi.encode(repository));
        bytes32 bytecodeHash = keccak256(bytecode);

        address computedAddress = Create2.computeAddress(VLAYER_STABLE_SALT, bytecodeHash, CREATE2_DEPLOYER_CONTRACT);
        assertEq(computedAddress, address(fakeProofVerifier));
    }

    function test_groth16ProofVerifierAddressIsStable() public {
        Repository repository = FutureTestnetStableDeployment.repository();
        (, Groth16ProofVerifier groth16ProofVerifier,) = FutureTestnetStableDeployment.verifiers();

        bytes memory bytecode = abi.encodePacked(type(Groth16ProofVerifier).creationCode, abi.encode(repository));
        bytes32 bytecodeHash = keccak256(bytecode);

        address computedAddress = Create2.computeAddress(VLAYER_STABLE_SALT, bytecodeHash, CREATE2_DEPLOYER_CONTRACT);
        assertEq(computedAddress, address(groth16ProofVerifier));
    }

    function test_proofVerifierRouterIsStable() public {
        (FakeProofVerifier fakeProofVerifier, Groth16ProofVerifier groth16ProofVerifier, ProofVerifierRouter router) =
            FutureTestnetStableDeployment.verifiers();

        bytes memory bytecode = abi.encodePacked(
            type(ProofVerifierRouter).creationCode, abi.encode(fakeProofVerifier, groth16ProofVerifier)
        );

        bytes32 bytecodeHash = keccak256(bytecode);

        address computedAddress = Create2.computeAddress(VLAYER_STABLE_SALT, bytecodeHash, CREATE2_DEPLOYER_CONTRACT);
        assertEq(computedAddress, address(router));
    }
}
