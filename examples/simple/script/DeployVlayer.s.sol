// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";

import {IRiscZeroVerifier} from "risc0-ethereum/IRiscZeroVerifier.sol";

import {Simple} from "../src/Simple.sol";
import {SimpleVerification} from "../src/vlayer/Simple.v.sol";

contract SimpleScript is Script {
    function setUp() public {}

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("DEPLOYER_PRIV");
        vm.startBroadcast(deployerPrivateKey);

        SimpleVerification simpleVerifier = new SimpleVerification();
        console2.log("SimpleVerification contract deployed to:", address(simpleVerifier));
    }
}
