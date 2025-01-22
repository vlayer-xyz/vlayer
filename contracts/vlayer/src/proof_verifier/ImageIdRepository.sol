// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

contract ImageIdRepository {
    mapping(bytes32 => bool) private imageIds;

    function addSupport(bytes32 imageId) external {
        imageIds[imageId] = true;
    }

    function isSupported(bytes32 imageId) public view returns (bool) {
        return imageIds[imageId];
    }
}
