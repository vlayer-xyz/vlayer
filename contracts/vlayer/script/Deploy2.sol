// SPDX-License-Identifier: UNLICENSED
/* solhint-disable no-console */
pragma solidity ^0.8.21;

import {Create2} from "@openzeppelin-contracts-5.0.1/utils/Create2.sol";
import {console} from "forge-std-1.9.4/src/Script.sol";

library Deploy2 {
    // The CREATE2 deterministic deployer contract: https://book.getfoundry.sh/guides/deterministic-deployments-using-create2#getting-started
    address public constant CREATE2_DEPLOYER_CONTRACT = 0x4e59b44847b379578588920cA78FbF26c0B4956C;

    function getOrDeploy(bytes memory creationCode, bytes32 salt) public returns (address) {
        address computedAddress = Create2.computeAddress(salt, keccak256(creationCode), CREATE2_DEPLOYER_CONTRACT);
        if (computedAddress.code.length == 0) {
            return Create2.deploy(0, salt, creationCode);
        } else {
            console.log("[ALREADY DEPLOYED]");
            return computedAddress;
        }
    }
}
