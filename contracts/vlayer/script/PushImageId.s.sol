// SPDX-License-Identifier: UNLICENSED
/* solhint-disable no-console */
pragma solidity ^0.8.21;

import {ImageID} from "../src/ImageID.sol";
import {Script} from "forge-std-1.9.4/src/Script.sol";
import {IImageIdRepository} from "../src/Repository.sol";

contract PushImageId is Script {
    function run() external {
        bytes32 imageId = ImageID.RISC0_CALL_GUEST_ID;
        address repositoryAddress = vm.envAddress("REPOSITORY_CONTRACT_ADDRESS");
        require(repositoryAddress != address(0), "IMAGE_ID_REPOSITORY not set");
        IImageIdRepository repository = IImageIdRepository(repositoryAddress);

        uint256 ownerPrivateKey = vm.envUint("REPOSITORY_OWNER_PRIVATE_KEY");
        vm.startBroadcast(ownerPrivateKey);
        repository.addImageIdSupport(imageId);
        vm.stopBroadcast();
    }
}
