// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {ChainIdLibrary, InvalidChainId} from "./ChainId.sol";
import {Repository} from "../Repository.sol";
import {ImageID} from "../ImageID.sol";
import {IProofVerifier} from "./IProofVerifier.sol";
import {FakeProofVerifier} from "./FakeProofVerifier.sol";
import {Groth16ProofVerifier} from "./Groth16ProofVerifier.sol";
import {ProofVerifierRouter} from "./ProofVerifierRouter.sol";

library ProofVerifierFactory {
    function produce() internal returns (IProofVerifier) {
        if (ChainIdLibrary.isMainnet()) {
            return IProofVerifier(address(0));
        } else if (ChainIdLibrary.isDevnet() || ChainIdLibrary.isTestnet()) {
            Repository repository = new Repository(address(this), address(this));
            repository.addImageIdSupport(ImageID.RISC0_CALL_GUEST_ID);
            return new ProofVerifierRouter(new FakeProofVerifier(repository), new Groth16ProofVerifier(repository));
        }

        revert InvalidChainId();
    }

    function testnetStableDeployment()
        internal
        pure
        returns (Repository, FakeProofVerifier, Groth16ProofVerifier, ProofVerifierRouter)
    {
        Repository repository = Repository(address(0xc9708B07ae9906b92FF19281Fd660FB19206a8fA));
        FakeProofVerifier fakeProofVerifier = FakeProofVerifier(address(0x67bAcaAf3B7E9B3e86B05479A9EE5b6ad611e34A));
        Groth16ProofVerifier groth16ProofVerifier =
            Groth16ProofVerifier(address(0x39599aC412c14F9635f5b5Bf8f4D4C1aeeCF6307));
        ProofVerifierRouter proofVerifierRouter =
            ProofVerifierRouter(address(0x782860fFa50B0cd234453612A56cf699f77a0687));

        return (repository, fakeProofVerifier, groth16ProofVerifier, proofVerifierRouter);
    }
}
