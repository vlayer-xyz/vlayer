// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Prover} from "vlayer/Prover.sol";
import { ExampleToken } from "./ExampleToken.sol";

contract SimpleTravelProver is Prover {
    ExampleToken [] public tokens;
    mapping(address => uint256) public chainIdToToken;

    constructor(address [] memory _tokens, uint256 [] memory _chainIds) {
        for (uint256 i = 0; i < _tokens.length; i++) {
            tokens.push(ExampleToken(_tokens[i]));
            chainIdToToken[_tokens[i]] = _chainIds[i];
        }
    } 

    function proveMultiChainOwnership(address _owner) public returns (address, uint256) {
        uint256 sum = 0;
        uint256 blockNo = 1;

        for (uint256 i = 0; i < tokens.length; i++) {
            setChain(chainIdToToken[address(tokens[i])], blockNo);
            sum += tokens[i].balanceOf(_owner);
        }

        return (_owner, sum);
    }
}
