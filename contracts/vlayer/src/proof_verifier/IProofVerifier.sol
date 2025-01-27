// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Proof} from "../Proof.sol";
import {IImageIdRepository} from "../Repository.sol";

interface IProofVerifier {
    function verify(Proof calldata proof, bytes32 journalHash, address expectedProver, bytes4 expectedSelector)
        external
        view;

    function imageIdRepository() external view returns (IImageIdRepository);
}
