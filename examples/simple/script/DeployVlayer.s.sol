// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";

import {IRiscZeroVerifier} from "risc0-ethereum/IRiscZeroVerifier.sol";

import {Simple} from "../src/Simple.sol";
import {SimpleVerification} from "../src/vlayer/Simple.v.sol";

contract SimpleScript is Script {
    function setUp() public {}

    function run() public {

        uint256 deployerPrivateKey = vm.envUint("DEPLOYER_PRIV");
        vm.startBroadcast(deployerPrivateKey);

        Simple simple = new Simple(IRiscZeroVerifier(address(0)));
        SimpleVerification simpleVerifier = new SimpleVerification(simple);

    }
}
