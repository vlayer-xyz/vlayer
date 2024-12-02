// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script} from "forge-std/src/Script.sol";
import {ERC20} from "openzeppelin-contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor(string memory name, string memory symbol) ERC20(name, symbol) {}

    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}

contract LoadFixtures is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("EXAMPLES_TEST_PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);

        MockERC20 usdc = new MockERC20("USDC", "USDC");
        usdc.mint(msg.sender, 1000);
        // advance to block 10
        vm.roll(10);

        // send 100 USDC to second address
        usdc.transfer(address(2), 100);
        vm.roll(20);

        // send 100 USDC to third address
        usdc.transfer(address(3), 100);
        vm.roll(40);

        vm.stopBroadcast();
    }
}
