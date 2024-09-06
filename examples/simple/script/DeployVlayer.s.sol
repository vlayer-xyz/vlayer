// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script} from "forge-std-1.9.2/src/Script.sol";
// solhint-disable-next-line no-console
import {console2} from "forge-std-1.9.2/src/console2.sol";

import {SimpleProver} from "../src/vlayer/SimpleProver.sol";

contract SimpleScript is Script {
    function setUp() public {}

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("DEPLOYER_PRIV");
        vm.startBroadcast(deployerPrivateKey);

        SimpleProver simpleProver = new SimpleProver();
        // solhint-disable-next-line no-console
        console2.log(
            "SimpleProver contract deployed to:",
            address(simpleProver)
        );
    }
}
