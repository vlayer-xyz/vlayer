// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Prover} from "vlayer/Prover.sol";
import {IERC20} from "openzeppelin-contracts/token/ERC20/IERC20.sol";

contract SimpleTimeTravelProver is Prover {
    IERC20 immutable token;
    uint256 immutable chainId;

    constructor(uint256 _chainId, IERC20 _token) {
        chainId = _chainId;
        token = _token;
    }

    function averageBalanceOf(address _owner) public returns (address, uint256) {
        uint256 startingBlock = 6639262;
        uint256 endingBlock = 6709262;
        uint256 step = 7000;
        uint256 totalBalance = 0;
        uint256 iterations = 0;

        for (uint256 blockNo = startingBlock; blockNo <= endingBlock; blockNo += step) {
            setChain(chainId, blockNo);
            totalBalance += token.balanceOf(_owner);
            iterations += 1;
        }
        return (_owner, totalBalance / iterations);
    }
}
