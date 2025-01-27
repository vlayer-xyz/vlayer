// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Proof} from "vlayer/Proof.sol";
import {Prover} from "vlayer/Prover.sol";

contract NoopWithCalldataProver is Prover {
    function noopWithCalldata(bytes calldata) public pure returns (Proof memory) {
        return (proof());
    }
}
