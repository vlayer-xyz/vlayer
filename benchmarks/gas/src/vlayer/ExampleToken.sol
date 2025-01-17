// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {ERC20} from "openzeppelin-contracts/token/ERC20/ERC20.sol";

contract ExampleToken is ERC20 {
    constructor(address initialOwner, uint256 initialSupply) ERC20("ExampleToken", "ET") {
        _mint(initialOwner, initialSupply);
    }
}
