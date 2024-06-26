// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {Simple, SimpleVerification} from "../src/vlayer/Simple.v.sol";

contract SimpleScript is Script {
    function setUp() public {}

    function run() public {

        uint256 deployerPrivateKey = vm.envUint("STABLE_DEPLOYER_PRIV");
        vm.startBroadcast(deployerPrivateKey);
        SimpleVerification simpleVerifier = new SimpleVerification(Simple(address(0)));

    }
}
