// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Prover} from "vlayer/Prover.sol";

struct SessionProof {
    uint8[] header;
    uint8[] signature;
    uint8[] session_info;
}

struct SubstringsProof {
    uint8[][] openings;
    uint8[] inclusion_proof;
}

contract WebProver is Prover {
    constructor() {}

    function main(
        SessionProof calldata session,
        SubstringsProof calldata substrings,
        string calldata notary_pub_key
    ) public pure returns (uint8[] memory) {
        return substrings.openings[0];
    }
}
