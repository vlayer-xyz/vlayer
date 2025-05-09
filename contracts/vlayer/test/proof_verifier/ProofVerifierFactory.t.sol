// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Test, console, console2} from "forge-std-1.9.4/src/Test.sol";

import {ImageID} from "../../src/ImageID.sol";
import {IProofVerifier} from "../../src/proof_verifier/IProofVerifier.sol";
import {ProofVerifierFactory, InvalidChainId} from "../../src/proof_verifier/ProofVerifierFactory.sol";

import {Repository} from "../../src/Repository.sol";
import {FakeProofVerifier} from "../../src/proof_verifier/FakeProofVerifier.sol";
import {Groth16ProofVerifier} from "../../src/proof_verifier/Groth16ProofVerifier.sol";
import {ProofVerifierRouter} from "../../src/proof_verifier/ProofVerifierRouter.sol";
import {TestnetStableDeployment} from "../../src/TestnetStableDeployment.sol";
import {TestHelpers} from "../helpers/TestHelpers.sol";

contract VerifierFactory_Tests is Test {
    TestHelpers helpers = new TestHelpers();

    struct DeployedContract {
        address contractAddress;
        string contractName;
    }

    struct Deployment {
        DeployedContract[] contracts;
    }

    function test_producesAVerifierInDevMode() public {
        vm.chainId(31337);

        IProofVerifier verifier = ProofVerifierFactory.produce();
        assert(verifier != IProofVerifier(address(0)));
    }

    function test_returnsAConstantForMainnets() public {
        vm.chainId(1);

        IProofVerifier verifier = ProofVerifierFactory.produce();
        assert(verifier == IProofVerifier(address(0x5553CF6Ce25E3f80fad2866f6230346159eCD89c)));
    }

    function test_devnetSupportsLatestImageID() public {
        vm.chainId(31337);

        IProofVerifier verifier = ProofVerifierFactory.produce();
        assertTrue(verifier.imageIdRepository().isImageSupported(ImageID.RISC0_CALL_GUEST_ID));
    }

    function test_returnsStableDeploymentForTestnets() public {
        vm.chainId(11155111);
        (,, ProofVerifierRouter router) = TestnetStableDeployment.verifiers();
        IProofVerifier verifier = ProofVerifierFactory.produce();
        assertEq(address(verifier), address(router));
    }

    function test_stableDeploymentForTestnetsEqualsToTheExpectedOne() public view {
        string memory root = vm.projectRoot();
        string memory path = string.concat(root, "/deployed_contracts.json");
        string memory json = vm.readFile(path);
        bytes memory data = vm.parseJson(json);

        Deployment memory deployment = abi.decode(data, (Deployment));

        Repository repository = TestnetStableDeployment.repository();
        (FakeProofVerifier fakeProofVerifier, Groth16ProofVerifier groth16ProofVerifier, ProofVerifierRouter router) =
            TestnetStableDeployment.verifiers();

        assertEq(address(repository), findAddress(deployment, "Repository"));
        assertEq(address(fakeProofVerifier), findAddress(deployment, "FakeProofVerifier"));
        assertEq(address(groth16ProofVerifier), findAddress(deployment, "Groth16ProofVerifier"));
        assertEq(address(router), findAddress(deployment, "ProofVerifierRouter"));
    }

    function test_failsForOtherNetworks() public {
        vm.chainId(11155111 + 1);

        vm.expectRevert(InvalidChainId.selector);
        helpers.produceProofVerifier();
    }

    function findAddress(Deployment memory deployment, string memory contractName) private pure returns (address) {
        for (uint256 i = 0; i < deployment.contracts.length; i++) {
            if (keccak256(bytes(deployment.contracts[i].contractName)) == keccak256(bytes(contractName))) {
                return deployment.contracts[i].contractAddress;
            }
        }
        console.log("Missing contract: %s", contractName);
        revert("Did not find contract with expected name");
    }
}
