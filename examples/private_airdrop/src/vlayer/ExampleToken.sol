// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { IERC20 } from "openzeppelin-contracts/contracts/token/ERC20/IERC20.sol";
import { ERC20 } from "openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";

contract ExampleToken is ERC20 {
    address[] public initialOwners;

    constructor(address [] memory extraOwners) ERC20("ExampleToken", "ET") {
        for (uint i = 0; i < extraOwners.length; i++){
            initialOwners.push(extraOwners[i]);
        }

        for (uint i = 0; i < initialOwners.length; i++){
            _mint(initialOwners[i], i+1 * 1 ether);
        }
    }
}

