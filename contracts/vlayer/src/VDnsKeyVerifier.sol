// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {AccessControl} from "@openzeppelin-contracts-5.0.1/access/AccessControl.sol";

import {IVDnsKeyVerifier} from "./interface/IVDnsKeyVerifier.sol";

contract VDnsKeyVerifier is AccessControl, IVDnsKeyVerifier {
    bytes32 public constant KEY_MANAGER_ROLE = keccak256("KEY_MANAGER_ROLE");

    mapping(bytes => bool) internal keys;

    constructor() {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
    }

    function addKey(bytes memory key) external onlyRole(KEY_MANAGER_ROLE) {
        require(!keys[key], "Key is already valid");
        keys[key] = true;

        emit KeyAdded(msg.sender, key);
    }

    function revokeKey(bytes memory key) external onlyRole(KEY_MANAGER_ROLE) {
        require(keys[key], "Key is invalid");
        keys[key] = false;

        emit KeyRevoked(msg.sender, key);
    }

    function isKeyValid(bytes memory key) external view override returns (bool) {
        return keys[key];
    }
}