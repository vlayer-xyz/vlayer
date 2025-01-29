// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Proof} from "vlayer/Proof.sol";
import {Prover} from "vlayer/Prover.sol";

contract ArithOpProver is Prover {
    function add(uint256 lhs, uint256 rhs) public pure returns (Proof memory, uint256) {
        uint256 res = lhs + rhs;
        return (proof(), res);
    }

    function mul(uint256 lhs, uint256 rhs) public pure returns (Proof memory, uint256) {
        uint256 res = lhs * rhs;
        return (proof(), res);
    }

    function sub(int256 lhs, int256 rhs) public pure returns (Proof memory, int256) {
        int256 res = lhs - rhs;
        return (proof(), res);
    }

    function div(uint256 lhs, uint256 rhs) public pure returns (Proof memory, uint256) {
        uint256 res = lhs / rhs;
        return (proof(), res);
    }

    function sdiv(int256 lhs, int256 rhs) public pure returns (Proof memory, int256) {
        int256 res = lhs / rhs;
        return (proof(), res);
    }
}
