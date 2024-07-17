// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

// import {IRiscZeroVerifier} from "risc0-ethereum/IRiscZeroVerifier.sol";
import {Verifier} from "vlayer/Verifier.sol";
// import {ExecutionCommitment} from "vlayer/ExecutionCommitment.sol";
// import {WebProver} from "./Prover.sol";

contract WebVerifier is Verifier {
    mapping(address => bool) authorizedMembers;
    mapping(string => bool) claimedUsersId;
    mapping(string => bool) trustedPubKeys;
    address owner;

    constructor() {
        owner = msg.sender;
    }

    modifier _ownerOnly() {
        require(msg.sender == owner);
        _;
    }

    function join(
        uint8[] calldata proof,
        string calldata userId,
        string calldata notaryPubKey
    ) public {
        require(!claimedUsersId[userId], "User Id claimed");
        require(trustedPubKeys[notaryPubKey], "Notary PubKey not trusted");

        claimedUsersId[userId] = true;
        authorizedMembers[msg.sender] = true;
    }

    function addTrustedPubKey(string calldata newPubKey) public _ownerOnly {
        trustedPubKeys[newPubKey] = true;
    }
}
