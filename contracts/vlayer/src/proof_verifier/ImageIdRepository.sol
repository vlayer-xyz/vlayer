// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

contract ImageIdRepository {
    constructor() {}

    function isSupported(bytes32) public pure returns (bool) {
        return true;
    }
}
