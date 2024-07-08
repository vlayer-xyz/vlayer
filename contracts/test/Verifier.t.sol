// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {IRiscZeroVerifier} from "risc0-ethereum/IRiscZeroVerifier.sol";

import {Steel} from "vlayer-engine/Steel.sol";
import {Seal} from "../src/Seal.sol";
import {Proof} from "../src/Proof.sol";

import {Verifier} from "../src/Verifier.sol";

contract ExampleVerifier is Verifier {
    event Increment(uint256 count);

    struct Counter {
        uint64 count;
        uint64 multiplier;
    }

    uint256 public counter;

    constructor(IRiscZeroVerifier verifier) Verifier(verifier) {
        counter = 0;
    }

    function withoutExtraParams(Proof calldata proof) external onlyVerified returns (bool) {
        emit Increment(++counter);
        return true;
    }

    function withOneExtraParam(Proof calldata proof, address) external onlyVerified returns (bool) {
        emit Increment(++counter);
        return true;
    }

    function withManyExtraParams(Proof calldata proof, address, Counter calldata, uint256)
        external
        onlyVerified
        returns (bool)
    {
        emit Increment(++counter);
        return true;
    }
}

contract Verifier_OnlyVerified_Modifier_Tests is Test {
    ExampleVerifier public vlayer;
    address public prover;

    function setUp() public {
        prover = address(1);
        vlayer = new ExampleVerifier(IRiscZeroVerifier(address(0)));
    }

    // Fixtures

    function seal_fixture() public view returns (Seal memory) {
        return Seal(bytes32(0), bytes32(0));
    }

    function commitment_fixture() public view returns (Steel.ExecutionCommitment memory) {
        return Steel.ExecutionCommitment(prover, bytes4(0x00), 1, bytes32(0x00));
    }

    function proof_fixture() public view returns (Proof memory) {
        return Proof(0, seal_fixture(), commitment_fixture());
    }

    // Happy paths

    function test_verifiesWithoutAdditonalParams() public {
        bool result = vlayer.withoutExtraParams(proof_fixture());
        require(result);
    }
}
