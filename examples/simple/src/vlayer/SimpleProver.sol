// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Proof} from "vlayer-0.1.0/Proof.sol";
import {Prover} from "vlayer-0.1.0/Prover.sol";
import {IERC20} from "openzeppelin-contracts/token/ERC20/IERC20.sol";

contract SimpleProver is Prover {
    IERC20 public immutable TOKEN;

    constructor(IERC20 _token) {
        TOKEN = _token;
    }

    function balance(address _owner) public view returns (Proof memory, address, uint256) {
        uint256 ownerBalance = TOKEN.balanceOf(_owner);

        return (proof(), _owner, ownerBalance);
    }
}
