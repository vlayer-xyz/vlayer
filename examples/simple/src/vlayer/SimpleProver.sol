// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Prover} from "vlayer/Prover.sol";
import {IERC20} from "openzeppelin-contracts/token/ERC20/IERC20.sol";

contract SimpleProver is Prover {
    IERC20 immutable token;
    uint256 immutable blockNo;

    constructor(IERC20 _token, uint256 _blockNo) {
        token = _token;
        blockNo = _blockNo;
    }

    function balance(address _owner) public returns (address, uint256) {
        setBlock(blockNo);
        uint256 currentBalance = token.balanceOf(_owner);
 
        return (_owner, currentBalance);
    }
}
