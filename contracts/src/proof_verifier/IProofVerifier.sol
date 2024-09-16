// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Proof} from "../Proof.sol";

interface IProofVerifier {

    function CALL_GUEST_ID() external view returns (bytes32);

    function verify(Proof calldata proof, bytes32 journalHash, address expectedProver, bytes4 expectedSelector)
        external
        view;
}
