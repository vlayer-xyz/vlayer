// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

interface IVDnsKeyVerifier {
    event KeyAdded(address indexed who, bytes key);
    event KeyRevoked(address indexed who, bytes key);

    function isDnsKeyValid(bytes memory key) external view returns (bool);
}
