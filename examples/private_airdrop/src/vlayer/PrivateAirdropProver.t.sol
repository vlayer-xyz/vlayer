// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { VTest } from "vlayer/testing/VTest.sol";
import { PrivateAirdropProver } from "./PrivateAirdropProver.sol";
import { VToyken } from "./VToyken.sol";
import { Strings } from "openzeppelin/contracts/utils/Strings.sol";
import "forge-std/Test.sol";
import "forge-std/console.sol";

contract PrivateAirdropProverTest is VTest {
    function test_prove() public {
        (address alice, uint256 alicePk) = makeAddrAndKey("alice");
        bytes32 hash = keccak256(
            abi.encodePacked("\x19Ethereum Signed Message:\n", Strings.toString(bytes("erc20 prover").length), "erc20 prover")
        );
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(alicePk, hash);

        bytes memory signature = abi.encodePacked(r, s, v); 

        VToyken token = new VToyken();
        PrivateAirdropProver prover = new PrivateAirdropProver(token);
        (uint256 balance, bytes32 nullifier) = prover.prove(alice, signature);
        assert(balance == 9000000000000000000);
    }

    function test_revertProve() public {
        (address alice, uint256 alicePk) = makeAddrAndKey("alice");
        (address joe, uint256 joePk) = makeAddrAndKey("joe");
        bytes32 hash = keccak256(
            abi.encodePacked("\x19Ethereum Signed Message:\n", Strings.toString(bytes("erc20 prover").length), "erc20 prover")
        );
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(joePk, hash);

        bytes memory signature = abi.encodePacked(r, s, v); 

        VToyken token = new VToyken();
        PrivateAirdropProver prover = new PrivateAirdropProver(token);
        vm.expectRevert(bytes("Invalid Signature"));
        prover.prove(alice, signature);
    }    
}
