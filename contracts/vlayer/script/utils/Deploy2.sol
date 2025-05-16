// SPDX-License-Identifier: UNLICENSED
/* solhint-disable no-console */
pragma solidity ^0.8.21;

import {console} from "forge-std-1.9.4/src/Script.sol";
import {Create2} from "@openzeppelin-contracts-5.0.1/utils/Create2.sol";

library Deploy2 {
    // The CREATE2 deterministic deployer contract: https://book.getfoundry.sh/guides/deterministic-deployments-using-create2#getting-started
    address public constant CREATE2_DEPLOYER_CONTRACT = 0x4e59b44847b379578588920cA78FbF26c0B4956C;

    function getOrDeploy(bytes memory creationCode, bytes32 salt) internal returns (address) {
        address computed = Deploy2.compute(salt, creationCode);
        if (computed.code.length == 0) {
            return Deploy2.deploy(salt, creationCode);
        } else {
            console.log("[SKIPPING - ALREADY DEPLOYED]");
            return computed;
        }
    }

    function compute(bytes32 salt, bytes memory creationCode) internal pure returns (address) {
        return Create2.computeAddress(salt, keccak256(creationCode), CREATE2_DEPLOYER_CONTRACT);
    }

    function deploy(bytes32 salt, bytes memory creationCode) internal returns (address) {
        return Create2.deploy(0, salt, creationCode);
    }
}
