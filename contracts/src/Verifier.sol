// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {IRiscZeroVerifier} from "risc0-ethereum/IRiscZeroVerifier.sol";
import {ControlID, RiscZeroGroth16Verifier} from "risc0-ethereum/groth16/RiscZeroGroth16Verifier.sol";
import {Steel} from "vlayer-engine/Steel.sol";
import {Proof} from "./Proof.sol";

contract VerifierBase {
    bytes32 public GUEST_ID = bytes32(0xb7079f57c71b4e1d95b8b1254303e13f78914599a8c119534c4c947c996b4d7d);

    IRiscZeroVerifier public verifier;

    modifier onlyVerified(address prover, bytes4 selector) {
        _verify(prover, selector);
        _;
    }

    function _verify(address prover, bytes4) internal virtual {
        Proof memory proof = abi.decode(msg.data[4:], (Proof));

        require(prover == proof.commitment.startContractAddress, "Invalid prover");
    }
}

contract Verifier is VerifierBase {

    constructor() {
        verifier = new RiscZeroGroth16Verifier(ControlID.CONTROL_ROOT, ControlID.BN254_CONTROL_ID);
    }
}

contract VerifierUnderTest is VerifierBase {
    function setVerifier(IRiscZeroVerifier _verifier) public {
        verifier = _verifier;
    }
}
