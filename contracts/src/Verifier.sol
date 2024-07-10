// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {IRiscZeroVerifier} from "risc0-ethereum/IRiscZeroVerifier.sol";
import {ControlID, RiscZeroGroth16Verifier} from "risc0-ethereum/groth16/RiscZeroGroth16Verifier.sol";

import {Proof} from "./Proof.sol";
import {SealLib, Seal} from "./Seal.sol";

contract VerifierBase {
    using SealLib for Seal;

    bytes32 public GUEST_ID = bytes32(0xb7079f57c71b4e1d95b8b1254303e13f78914599a8c119534c4c947c996b4d7d);
    uint256 constant AVAILABLE_HISTORICAL_BLOCKS = 256;
    uint256 constant JOURNAL_OFFSET = 100;

    IRiscZeroVerifier public verifier;

    modifier onlyVerified(address prover, bytes4 selector) {
        _verify(prover, selector);
        _;
    }

    function _verify(address prover, bytes4 selector) internal virtual {
        Proof memory proof = abi.decode(msg.data[4:], (Proof));

        uint256 journalEnd = JOURNAL_OFFSET + proof.length;
        bytes memory journal = msg.data[JOURNAL_OFFSET:journalEnd];
        bytes32 journalHash = sha256(journal);

        require(proof.commitment.startContractAddress == prover, "Invalid prover");
        require(proof.commitment.functionSelector == selector, "Invalid selector");

        require(proof.commitment.settleBlockNumber < block.number, "Invalid block number: block from future");
        require(
            proof.commitment.settleBlockNumber + AVAILABLE_HISTORICAL_BLOCKS >= block.number,
            "Invalid block number: block too old"
        );
        require(proof.commitment.settleBlockHash == blockhash(proof.commitment.settleBlockNumber), "Invalid block hash");

        verifier.verify(proof.seal.decode(), GUEST_ID, journalHash);
    }
}

contract Verifier is VerifierBase {
    constructor() {
        verifier = new RiscZeroGroth16Verifier(ControlID.CONTROL_ROOT, ControlID.BN254_CONTROL_ID);
    }
}
