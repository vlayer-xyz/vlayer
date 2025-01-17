// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

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

    constructor(Erc20Token[] memory _tokens) {
        for (uint256 i = 0; i < _tokens.length; i++) {
            tokens.push(_tokens[i]);
        }
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
