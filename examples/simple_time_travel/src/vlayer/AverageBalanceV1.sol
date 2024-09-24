// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Prover} from "vlayer/Prover.sol";
import {IERC20} from "openzeppelin-contracts/token/ERC20/IERC20.sol";

contract AverageBalanceV1 is Prover {
    IERC20 immutable token;
    uint256 immutable startingBlock;
    uint256 immutable endingBlock;
    uint256 immutable step;

    constructor(IERC20 _token, uint256 _startBlockNo, uint256 _endingBlockNo, uint256 _step) {
        token = _token;
        startingBlock = _startBlockNo;
        endingBlock = _endingBlockNo;
        step = _step;
    }

    function averageBalanceOf(address _owner) public returns (address, uint256) {
        uint256 totalBalance = 0;
        uint256 iterations = 0;

        for (uint256 blockNo = startingBlock; blockNo <= endingBlock; blockNo += step) {
            setBlock(blockNo);
            totalBalance += token.balanceOf(_owner);
            iterations += 1;
        }
        return (_owner, totalBalance / iterations);
    }
}
