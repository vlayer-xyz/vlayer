// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {IRiscZeroVerifier} from "risc0-ethereum/IRiscZeroVerifier.sol";
import {Verifier} from "vlayer/Verifier.sol";
import {ExecutionCommitment} from "vlayer/ExecutionCommitment.sol";

import {WebProver} from "./Prover.sol";

contract WebVerifier is Verifier {
    mapping(address => bool) public authorizedMembers;
    mapping(string => bool) public claimedUsersId;

    function join(
        Proof proof,
        uint8[] calldata userId,
        string calldata notaryPubKey
    ) public onlyVerified(PROVER_ADDR, PROVER_FUNC_SELECTOR) {
        require(!claimedUsersId[userId], "Claimed user Id");

        claimedUsersId[userId] = true;
        authorizedMembers[msg.sender] = true;
    }
}
