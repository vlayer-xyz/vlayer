// SPDX-License-Identifier: UNLICENSED
/* solhint-disable no-console */
pragma solidity ^0.8.21;

import {console, Script} from "forge-std-1.9.4/src/Script.sol";

import {Repository} from "../src/Repository.sol";
import {FakeProofVerifier} from "../src/proof_verifier/FakeProofVerifier.sol";
import {Groth16ProofVerifier} from "../src/proof_verifier/Groth16ProofVerifier.sol";
import {ProofVerifierRouter} from "../src/proof_verifier/ProofVerifierRouter.sol";

bytes32 constant VLAYER_STABLE_SALT = keccak256("sepolia.vlayer.xyz");

contract VlayerDeployer is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        address admin = vm.envAddress("REPOSITORY_CONTRACT_ADMIN_ADDRESS");

        console.log("REPOSITORY_CONTRACT_ADMIN_ADDRESS=%s", admin);

        vm.startBroadcast(deployerPrivateKey);

        Repository repository = deployKeyRegistry(admin);

        FakeProofVerifier fakeProofVerifier = deployFakeProofVerifier(repository);
        Groth16ProofVerifier groth16ProofVerifier = deployGroth16ProofVerifier(repository);
        ProofVerifierRouter proofVerifierRouter = deployProofVerifierRouter(fakeProofVerifier, groth16ProofVerifier);

        vm.stopBroadcast();

        console.log("REPOSITORY_ADDRESS=%s", address(repository));
        console.log("FAKE_PROOF_VERIFIER_ADDRESS=%s", address(fakeProofVerifier));
        console.log("GROTH16_PROOF_VERIFIER_ADDRESS=%s", address(groth16ProofVerifier));
        console.log("PROOF_VERIFIER_ROUTER_ADDRESS=%s", address(proofVerifierRouter));
    }

    function deployKeyRegistry(address admin) internal returns (Repository) {
        return new Repository{salt: VLAYER_STABLE_SALT}(admin, admin);
    }

    function deployFakeProofVerifier(Repository repository) internal returns (FakeProofVerifier) {
        return new FakeProofVerifier{salt: VLAYER_STABLE_SALT}(repository);
    }

    function deployGroth16ProofVerifier(Repository repository) internal returns (Groth16ProofVerifier) {
        return new Groth16ProofVerifier{salt: VLAYER_STABLE_SALT}(repository);
    }

    function deployProofVerifierRouter(FakeProofVerifier fakeProofVerifier, Groth16ProofVerifier groth16ProofVerifier)
        internal
        returns (ProofVerifierRouter)
    {
        return new ProofVerifierRouter{salt: VLAYER_STABLE_SALT}(fakeProofVerifier, groth16ProofVerifier);
    }
}
