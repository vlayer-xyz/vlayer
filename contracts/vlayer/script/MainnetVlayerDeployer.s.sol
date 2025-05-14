// SPDX-License-Identifier: UNLICENSED
/* solhint-disable no-console */
pragma solidity ^0.8.21;

import {console, Script} from "forge-std-1.9.4/src/Script.sol";
import {Repository} from "../src/Repository.sol";
import {Groth16ProofVerifier} from "../src/proof_verifier/Groth16ProofVerifier.sol";
import {Deploy2} from "./utils/Deploy2.sol";

bytes32 constant VLAYER_STABLE_SALT = keccak256("mainnet.vlayer.xyz");

contract MainnetVlayerDeployer is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        address admin = vm.envAddress("REPOSITORY_CONTRACT_ADMIN_ADDRESS");
        address owner = vm.envAddress("REPOSITORY_CONTRACT_OWNER_ADDRESS");

        console.log("REPOSITORY_CONTRACT_ADMIN_ADDRESS=%s", admin);
        console.log("REPOSITORY_CONTRACT_OWNER_ADDRESS=%s", owner);

        vm.startBroadcast(deployerPrivateKey);

        Repository repository = getOrDeployKeyRegistry(admin, owner);
        console.log("REPOSITORY_ADDRESS=%s", address(repository));

        Groth16ProofVerifier groth16ProofVerifier = getOrDeployGroth16ProofVerifier(repository);
        console.log("GROTH16_PROOF_VERIFIER_ADDRESS=%s", address(groth16ProofVerifier));

        vm.stopBroadcast();
    }

    function getOrDeployKeyRegistry(address admin, address owner) internal returns (Repository) {
        bytes memory constructorArgs = abi.encode(admin, owner);
        bytes memory creationCode = abi.encodePacked(type(Repository).creationCode, constructorArgs);

        address addr = Deploy2.getOrDeploy(creationCode, VLAYER_STABLE_SALT);

        return Repository(addr);
    }

    function getOrDeployGroth16ProofVerifier(Repository repository) internal returns (Groth16ProofVerifier) {
        bytes memory constructorArgs = abi.encode(repository);
        bytes memory creationCode = abi.encodePacked(type(Groth16ProofVerifier).creationCode, constructorArgs);

        address addr = Deploy2.getOrDeploy(creationCode, VLAYER_STABLE_SALT);

        return Groth16ProofVerifier(addr);
    }
}
