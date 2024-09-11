// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Prover} from "vlayer/Prover.sol";
import {CrossChainBalance, Erc20Token} from "./CrossChainBalance.sol";

contract SimpleTravelProver is CrossChainBalance {
    function getTokens() public pure returns (Erc20Token[] memory) {
        Erc20Token[] memory tokens = new Erc20Token[](3);

        tokens[0] = Erc20Token(0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48, 1, 20683110); // mainnet
        tokens[1] = Erc20Token(0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913, 8453, 19367633); // base
        tokens[2] = Erc20Token(0x0b2C639c533813f4Aa9D7837CAf62653d097Ff85, 10, 124962954); // arbitrum

        return tokens;
    }

    function crossChainBalanceOf(address _owner) public returns (address, uint256) {
        uint256 balance = balanceOf(_owner, getTokens());

        return (_owner, balance);
    }
}
