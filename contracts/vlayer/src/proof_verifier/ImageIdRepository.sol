// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {AccessControlEnumerable} from "openzeppelin-contracts/access/extensions/AccessControlEnumerable.sol";

contract ImageIdRepository is AccessControlEnumerable {
    event AddedImageIDSupport(bytes32 imageID);
    event RevokedImageIDSupport(bytes32 imageID);

    bytes32 public constant OWNER_ROLE = keccak256("OWNER_ROLE");
    mapping(bytes32 => bool) private imageIds;

    constructor() {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(OWNER_ROLE, msg.sender);
    }

    function transferAdminRole(address newAdmin) public {
        grantRole(DEFAULT_ADMIN_ROLE, newAdmin);
        renounceRole(DEFAULT_ADMIN_ROLE, msg.sender);
    }

    function transferOwnership(address newOwner) public {
        address owner = getRoleMember(OWNER_ROLE, 0);
        revokeRole(OWNER_ROLE, owner);
        grantRole(OWNER_ROLE, newOwner);
    }

    function addSupport(bytes32 imageId) external onlyRole(OWNER_ROLE) {
        require(!isSupported(imageId), "ImageID is already supported");

        imageIds[imageId] = true;
        emit AddedImageIDSupport(imageId);
    }

    function revokeSupport(bytes32 imageId) external onlyRole(OWNER_ROLE) {
        require(isSupported(imageId), "Cannot revoke unsupported ImageID");

        imageIds[imageId] = false;
        emit RevokedImageIDSupport(imageId);
    }

    function isSupported(bytes32 imageId) public view returns (bool) {
        return imageIds[imageId];
    }
}
