// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {IERC20} from "@openzeppelin-contracts-5.0.1/token/ERC20/IERC20.sol";
import {Prover} from "vlayer-0.1.0/src/Prover.sol";

contract ERC20Prover is Prover {
    IERC20 public immutable TOKEN;

    constructor(IERC20 token) {
        TOKEN = token;
    }

    function prove(address[] calldata accounts) public view returns (uint256) {
        uint256 sum = 0;

        for (uint256 i = 0; i < accounts.length; i++) {
            uint256 balance = TOKEN.balanceOf(accounts[i]);
            require(balance > 0, "Insufficient balance");
            sum += balance;
        }

        return sum;
    }
}
