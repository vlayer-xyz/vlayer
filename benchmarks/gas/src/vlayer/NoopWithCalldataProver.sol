// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Proof} from "vlayer-0.1.0/Proof.sol";
import {Prover} from "vlayer-0.1.0/Prover.sol";

contract NoopWithCalldataProver is Prover {
    function noopWithCalldata(
        bytes calldata payload
    ) public pure returns (Proof memory) {
        return (proof());
    }
}
