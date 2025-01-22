// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Test, console} from "forge-std-1.9.4/src/Test.sol";

import {ImageID} from "../../src/ImageID.sol";
import {ImageIdRepository} from "../../src/proof_verifier/ImageIdRepository.sol";

contract ImageIdRepository_addSupport_Tests is Test {
    ImageIdRepository public repository;

    function setUp() public {
        repository = new ImageIdRepository();
    }

    function test_byDefaultCurrentImageIdIsNotSupported() public view {
        assertTrue(!repository.isSupported(bytes32(0x1111111111111111111111111111111111111111111111111111111111111111)));
        assertTrue(!repository.isSupported(ImageID.RISC0_CALL_GUEST_ID));
    }

    function test_onceAddedSupportImageIsSupported() public {
        assertTrue(!repository.isSupported(bytes32(0x1111111111111111111111111111111111111111111111111111111111111111)));

        repository.addSupport(bytes32(0x1111111111111111111111111111111111111111111111111111111111111111));

        assertTrue(repository.isSupported(bytes32(0x1111111111111111111111111111111111111111111111111111111111111111)));
    }
}
