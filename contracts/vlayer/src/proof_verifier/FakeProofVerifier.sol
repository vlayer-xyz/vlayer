// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {RiscZeroMockVerifier} from "risc0-ethereum-2.0.0/src/test/RiscZeroMockVerifier.sol";

import {ProofMode, SealLib, Seal} from "../Seal.sol";

import {ChainIdLibrary, InvalidChainId} from "./ChainId.sol";
import {ProofVerifierBase, IImageIdRepository} from "./ProofVerifierBase.sol";

bytes4 constant FAKE_VERIFIER_SELECTOR = bytes4(0xdeafbeef);

contract FakeProofVerifier is ProofVerifierBase {
    using SealLib for Seal;

    constructor(IImageIdRepository _repository) ProofVerifierBase(_repository) {
        if (ChainIdLibrary.isMainnet()) {
            revert InvalidChainId();
        }

        VERIFIER = new RiscZeroMockVerifier(FAKE_VERIFIER_SELECTOR);
        PROOF_MODE = ProofMode.FAKE;
    }
}
