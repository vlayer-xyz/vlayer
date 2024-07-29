// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";

import {SimpleTravelProver} from "../src/vlayer/SimpleTravelProver.sol";
import {Counter} from "../src/vlayer/Counter.sol";

contract SimpleTravelScript is Script {
    function setUp() public {}

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("DEPLOYER_PRIV");
        vm.startBroadcast(deployerPrivateKey);

        Counter counter = new Counter();
        SimpleTravelProver simpleTravelProver = new SimpleTravelProver(address(counter));
        console2.log("SimpleTravelProver contract deployed to:", address(simpleTravelProver));
        counter.increment();
    }
}
