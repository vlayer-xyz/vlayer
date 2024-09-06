// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script} from "forge-std-1.9.2/src/Script.sol";
// solhint-disable-next-line no-console
import {console2} from "forge-std-1.9.2/src/console2.sol";

import {WebProofProver} from "../src/vlayer/WebProofProver.sol";

contract WebProofScript is Script {
    function setUp() public {}

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("DEPLOYER_PRIV");
        vm.startBroadcast(deployerPrivateKey);

        WebProofProver webProofProver = new WebProofProver();
        // solhint-disable-next-line no-console
        console2.log(
            "WebProofProver contract deployed to:",
            address(webProofProver)
        );
    }
}
