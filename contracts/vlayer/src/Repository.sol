// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {AccessControlEnumerable} from "@openzeppelin-contracts-5.0.1/access/extensions/AccessControlEnumerable.sol";

interface IImageIdRepository {
    event ImageIDAdded(bytes32 imageID);
    event ImageIDRevoked(bytes32 imageID);

    function addImageIdSupport(bytes32 imageId) external;
    function revokeImageIdSupport(bytes32 imageId) external;
    function isImageSupported(bytes32 imageId) external view returns (bool);
}

interface IVDnsKeyRepository {
    event DnsKeyAdded(address indexed who, bytes key);
    event DnsKeyRevoked(address indexed who, bytes key);

    function isDnsKeyValid(bytes memory key) external view returns (bool);
}

interface INotaryKeyRepository {
    event NotaryKeyAdded(address indexed who, string key);
    event NotaryKeyRevoked(address indexed who, string key);

    function isNotaryKeyValid(string memory key) external view returns (bool);
}

contract Repository is AccessControlEnumerable, IImageIdRepository, IVDnsKeyRepository, INotaryKeyRepository {
    bytes32 public constant OWNER_ROLE = keccak256("OWNER_ROLE");

    mapping(bytes32 => bool) internal dnsKeys;
    mapping(bytes32 => bool) internal imageIds;
    mapping(bytes32 => bool) internal notaryKeys;

    constructor(address admin, address owner) {
        _grantRole(DEFAULT_ADMIN_ROLE, admin);
        _grantRole(OWNER_ROLE, owner);
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

    ///===============*** IMAGE ID ***===============///

    function addImageIdSupport(bytes32 imageId) external onlyRole(OWNER_ROLE) {
        require(!isImageSupported(imageId), "ImageID is already supported");

        imageIds[imageId] = true;
        emit ImageIDAdded(imageId);
    }

    function revokeImageIdSupport(bytes32 imageId) external onlyRole(OWNER_ROLE) {
        require(isImageSupported(imageId), "Cannot revoke unsupported ImageID");

        imageIds[imageId] = false;
        emit ImageIDRevoked(imageId);
    }

    function isImageSupported(bytes32 imageId) public view returns (bool) {
        return imageIds[imageId];
    }

    ///===============*** DNS KEYS ***===============///

    function addDnsKey(bytes memory key) external onlyRole(OWNER_ROLE) {
        bytes32 hash = keccak256(key);
        require(!dnsKeys[hash], "Key is already valid");

        dnsKeys[hash] = true;
        emit DnsKeyAdded(msg.sender, key);
    }

    function revokeDnsKey(bytes memory key) external onlyRole(OWNER_ROLE) {
        bytes32 hash = keccak256(key);
        require(dnsKeys[hash], "Cannot revoke invalid key");

        dnsKeys[hash] = false;
        emit DnsKeyRevoked(msg.sender, key);
    }

    function isDnsKeyValid(bytes memory key) external view override returns (bool) {
        return dnsKeys[keccak256(key)];
    }

    ///===============*** NOTARY KEYS ***===============///

    function addNotaryKey(string memory key) external onlyRole(OWNER_ROLE) {
        bytes32 hash = keccak256(bytes(key));
        require(!notaryKeys[hash], "Key is already valid");

        notaryKeys[hash] = true;
        emit NotaryKeyAdded(msg.sender, key);
    }

    function revokeNotaryKey(string memory key) external onlyRole(OWNER_ROLE) {
        bytes32 hash = keccak256(bytes(key));
        require(notaryKeys[hash], "Cannot revoke invalid key");

        notaryKeys[hash] = false;
        emit NotaryKeyRevoked(msg.sender, key);
    }

    function isNotaryKeyValid(string memory key) external view override returns (bool) {
        return notaryKeys[keccak256(bytes(key))];
    }
}
