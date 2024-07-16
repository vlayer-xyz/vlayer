// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {IRiscZeroVerifier} from "risc0-ethereum/IRiscZeroVerifier.sol";
import {Verifier} from "vlayer/Verifier.sol";
import {ExecutionCommitment} from "vlayer/ExecutionCommitment.sol";
import {WebProver} from "./Prover.sol";

contract WebVerifier is Verifier {
    mapping(address => bool) authorizedMembers;
    mapping(string => bool) claimedUsersId;
    mapping(string => bool) trustedPubKeys;
    address owner;

    constructor() public {
        owner = msg.sender;
    }

    modifier _ownerOnly() {
        require(msg.sender == owner);
    }

    function join(
        Proof proof,
        uint8[] calldata userId,
        string calldata notaryPubKey
    ) public onlyVerified(PROVER_ADDR, PROVER_FUNC_SELECTOR) {
        require(!claimedUsersId[userId], "User Id claimed");
        require(trustedPubKeys[notaryPubKey], "Notary PubKey not trusted");

        claimedUsersId[userId] = true;
        authorizedMembers[msg.sender] = true;
    }

    function addTrustedPubKey(String newPubKey) _ownerOnly {
        trustedPubKeys[newPubKey] = true;
    }
}
