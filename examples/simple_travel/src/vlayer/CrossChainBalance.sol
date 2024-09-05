// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Prover} from "vlayer/Prover.sol";
import {IERC20} from "openzeppelin-contracts/contracts/token/ERC20/IERC20.sol";

struct Erc20Token {
    address addr;
    uint256 chainId;
    uint256 blockNumber;
}

contract CrossChainBalance is Prover {
    function balanceOf(address _owner, Erc20Token[] memory tokens) public returns (uint256) {
        uint256 balance = 0;

        for (uint256 i = 0; i < tokens.length; i++) {
            setChain(tokens[i].chainId, tokens[i].blockNumber);
            balance += IERC20(tokens[i].addr).balanceOf(_owner);
        }

        return balance;
    }
}
