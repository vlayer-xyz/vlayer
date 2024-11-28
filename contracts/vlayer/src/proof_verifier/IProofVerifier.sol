// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Proof} from "../Proof.sol";

interface IProofVerifier {
    function call_guest_id() external view returns (bytes32);

    function verify(Proof calldata proof, bytes32 journalHash, address expectedProver, bytes4 expectedSelector)
        external
        view;
}
