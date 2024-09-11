// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Prover} from "vlayer/Prover.sol";
import {Balance} from "./Balance.sol";

contract SimpleProver is Balance {
    address usdc;

    constructor(address _tokenAddr) {
        usdc = _tokenAddr;
    }

    function balance(address _owner) public view returns (uint256) {
        return balanceOf(_owner, usdc);
    }
}
