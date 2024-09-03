// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Prover} from "vlayer/Prover.sol";
import {ExampleToken} from "./ExampleToken.sol";
import {IERC20} from "openzeppelin-contracts/contracts/token/ERC20/IERC20.sol";

contract SimpleTravelProver is Prover {
    struct Erc20Token {
        address contractAddr;
        uint256 chainId;
        uint256 blockNumber;
    }

    mapping(address => Erc20Token) public permittedTokens;

    constructor(address[] memory _tokens, uint256[] memory _chainIds, uint256[] memory _blockNumbers) {
        require(_tokens.length == _chainIds.length && _tokens.length == _blockNumbers.length, "Invalid input length");
        for (uint256 i = 0; i < _tokens.length; i++) {
            permittedTokens[_tokens[i]] = Erc20Token(_tokens[i], _chainIds[i], _blockNumbers[i]);
        }
    }

    function crossChainBalanceOf(address _owner, Erc20Token[] memory _tokens) public returns (address, uint256) {
        uint256 crossChainBalance = 0;

        for (uint256 i = 0; i < _tokens.length; i++) {
            Erc20Token memory token = permittedTokens[_tokens[i].contractAddr];
            require(token.contractAddr != address(0), "token not permitted");
            require(token.blockNumber == _tokens[i].blockNumber, "wrong block no");

            setChain(token.chainId, token.blockNumber);
            crossChainBalance += IERC20(token.contractAddr).balanceOf(_owner);
        }

        return (_owner, crossChainBalance);
    }
}
