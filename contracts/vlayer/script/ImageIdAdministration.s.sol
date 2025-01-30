// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {console, Script} from "forge-std-1.9.4/src/Script.sol";

import {ImageID} from "../src/ImageID.sol";
import {IImageIdRepository} from "../src/Repository.sol";
import {ChainIdLibrary} from "../src/proof_verifier/ChainId.sol";

contract AddSupportForCurrentImageId is Script {
    function run() external {
        AddImageIdSupport addImageIdSupportScript = new AddImageIdSupport();
        address repository = vm.envAddress("REPOSITORY_CONTRACT_ADDRESS");

        addImageIdSupportScript.run(IImageIdRepository(repository), ImageID.RISC0_CALL_GUEST_ID);
    }
}

contract AddImageIdSupport is Script {
    function run(IImageIdRepository repository, bytes32 imageId) external {
        uint256 ownerPrivateKey = vm.envUint("REPOSITORY_CONTRACT_OWNER_PRIVATE_KEY");

        console.log("REPOSITORY_ADDRESS=%s", address(repository));
        console.log("IMAGE_ID=");
        console.logBytes32(imageId);

        if (repository.isImageSupported(imageId)) {
            console.log("Image ID is already supported");
            return;
        }

        vm.startBroadcast(ownerPrivateKey);
        repository.addImageIdSupport(imageId);
        vm.stopBroadcast();
    }
}

contract RevokeImageIdSupport is Script {
    function run(IImageIdRepository repository, bytes32 imageId) external {
        uint256 ownerPrivateKey = vm.envUint("REPOSITORY_CONTRACT_OWNER_PRIVATE_KEY");

        console.log("REPOSITORY_ADDRESS=%s", address(repository));
        console.log("IMAGE_ID=");
        console.logBytes32(imageId);

        vm.startBroadcast(ownerPrivateKey);
        repository.revokeImageIdSupport(imageId);
        vm.stopBroadcast();
    }
}
