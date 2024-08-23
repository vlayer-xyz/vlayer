// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script} from "forge-std/Script.sol";
// solhint-disable-next-line
import { console2 } from "forge-std/console2.sol";

import {SimpleTravelProver} from "../src/vlayer/SimpleTravelProver.sol";

contract SimpleTravelScript is Script {
    function setUp() public {}

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("DEPLOYER_PRIV");
        vm.startBroadcast(deployerPrivateKey);

        SimpleTravelProver simpleTravelProver = new SimpleTravelProver();
        // solhint-disable-next-line
        console2.log("SimpleTravelProver contract deployed to:", address(simpleTravelProver));
    }
}
