// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Prover} from "vlayer/Prover.sol";
import {Balance} from "./Balance.sol";

contract SimpleProver is Balance {
    address immutable tokenAddr;

    constructor(address _tokenAddr) {
        tokenAddr = _tokenAddr;
    }

    function balance(address _owner) public view returns (address, uint256) {
        return (_owner, balanceOf(_owner, tokenAddr));
    }
}
