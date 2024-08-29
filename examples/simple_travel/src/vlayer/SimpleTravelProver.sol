// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Prover} from "vlayer/Prover.sol";
import {ExampleToken} from "./ExampleToken.sol";

contract SimpleTravelProver is Prover {
    function proveMultiChainOwnership(
        address [] memory _tokens, 
        address _owner, 
        uint256 [] memory _chainIds,
        uint256 [] memory _blockNumbers
    ) public returns (address, uint256) {
        uint256 sum = 0;

        for (uint256 i = 0; i < _tokens.length; i++) {
            setChain(_chainIds[i], _blockNumbers[i]);
            sum += ExampleToken(_tokens[i]).balanceOf(_owner);
        }

        return (_owner, sum);
    }
}
