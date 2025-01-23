// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

contract ImageIdRepository {
    event AddedImageIDSupport(bytes32 imageID);
    event RevokedImageIDSupport(bytes32 imageID);

    mapping(bytes32 => bool) private imageIds;

    function addSupport(bytes32 imageId) external {
        require(!isSupported(imageId), "ImageID is already supported");

        imageIds[imageId] = true;
        emit AddedImageIDSupport(imageId);
    }

    function revokeSupport(bytes32 imageId) external {
        require(isSupported(imageId), "Cannot revoke unsupported ImageID");

        imageIds[imageId] = false;
        emit RevokedImageIDSupport(imageId);
    }

    function isSupported(bytes32 imageId) public view returns (bool) {
        return imageIds[imageId];
    }
}
