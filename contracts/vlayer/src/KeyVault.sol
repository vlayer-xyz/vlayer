// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {AccessControl} from "@openzeppelin-contracts-5.0.1/access/AccessControl.sol";

import {IVDnsKeyVerifier} from "./interface/IVDnsKeyVerifier.sol";

contract KeyVault is AccessControl, IVDnsKeyVerifier {
    bytes32 public constant KEY_MANAGER_ROLE = keccak256("KEY_MANAGER_ROLE");

    mapping(bytes => bool) internal dnsKeys;

    constructor() {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
    }

    function addDnsKey(bytes memory key) external onlyRole(KEY_MANAGER_ROLE) {
        require(!dnsKeys[key], "Key is already valid");
        dnsKeys[key] = true;

        emit KeyAdded(msg.sender, key);
    }

    function revokeDnsKey(bytes memory key) external onlyRole(KEY_MANAGER_ROLE) {
        require(dnsKeys[key], "Key is invalid");
        dnsKeys[key] = false;

        emit KeyRevoked(msg.sender, key);
    }

    function isDnsKeyValid(bytes memory key) external view override returns (bool) {
        return dnsKeys[key];
    }
}
