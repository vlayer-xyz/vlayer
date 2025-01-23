// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Proof} from "vlayer-0.1.0/Proof.sol";
import {Prover} from "vlayer-0.1.0/Prover.sol";
import {IERC20} from "openzeppelin-contracts/token/ERC20/IERC20.sol";

contract AverageBalance is Prover {
    IERC20 public immutable TOKEN;
    uint256 public immutable STARTING_BLOCK;
    uint256 public immutable ENDING_BLOCK;
    uint256 public immutable STEP;

    constructor(IERC20 _token, uint256 _startBlockNo, uint256 _endingBlockNo, uint256 _step) {
        TOKEN = _token;
        STARTING_BLOCK = _startBlockNo;
        ENDING_BLOCK = _endingBlockNo;
        STEP = _step;
    }

    function averageBalanceOf(address _owner) public returns (Proof memory, address, uint256) {
        uint256 totalBalance = 0;
        uint256 iterations = 0;

        for (uint256 blockNo = STARTING_BLOCK; blockNo <= ENDING_BLOCK; blockNo += STEP) {
            setBlock(blockNo);
            totalBalance += TOKEN.balanceOf(_owner);
            iterations += 1;
        }
        return (proof(), _owner, totalBalance / iterations);
    }
}
