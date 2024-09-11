// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Prover} from "vlayer/Prover.sol";
import {IERC20} from "openzeppelin-contracts/token/ERC20/IERC20.sol";

contract Balance is Prover {
    function balanceOf(address _owner, address _tokenAddr) public view returns (uint256) {
        return IERC20(_tokenAddr).balanceOf(_owner);
    }
}
