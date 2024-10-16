// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Proof} from "vlayer-0.1.0/Proof.sol";
import {Prover} from "vlayer-0.1.0/Prover.sol";
import {IERC20} from "openzeppelin-contracts/token/ERC20/IERC20.sol";

struct Erc20Token {
    address addr;
    uint256 chainId;
    uint256 blockNumber;
}

contract SimpleTeleportProver is Prover {
    Erc20Token[] public tokens;

    constructor() {
        tokens.push(Erc20Token(0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48, 1, 20683110)); // mainnet
        tokens.push(Erc20Token(0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913, 8453, 19367633)); // base
        tokens.push(Erc20Token(0x0b2C639c533813f4Aa9D7837CAf62653d097Ff85, 10, 124962954)); // optimism
    }

    function crossChainBalanceOf(address _owner) public returns (Proof memory, address, uint256) {
        uint256 balance = 0;

        for (uint256 i = 0; i < tokens.length; i++) {
            setChain(tokens[i].chainId, tokens[i].blockNumber);
            balance += IERC20(tokens[i].addr).balanceOf(_owner);
        }

        return (proof(), _owner, balance);
    }
}
