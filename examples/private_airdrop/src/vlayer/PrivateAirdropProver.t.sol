// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { VTest } from "vlayer/testing/VTest.sol";
import { PrivateAirdropProver } from "./PrivateAirdropProver.sol";
import { ExampleToken } from "./ExampleToken.sol";
import { Strings } from "openzeppelin/contracts/utils/Strings.sol";
import "forge-std/Test.sol";
import "forge-std/console.sol";

contract PrivateAirdropProverTest is VTest {
    function test_prove() public {
        (address alice, uint256 alicePk) = makeAddrAndKey("alice");
        address[] memory initialOwners = new address[](1);
        initialOwners[0] = alice;

        bytes32 hash = keccak256(
            abi.encodePacked("\x19Ethereum Signed Message:\n", Strings.toString(bytes("erc20 prover").length), "erc20 prover")
        );
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(alicePk, hash);

        bytes memory signature = abi.encodePacked(r, s, v); 

        ExampleToken token = new ExampleToken(initialOwners);
        PrivateAirdropProver prover = new PrivateAirdropProver(token);
        (uint256 balance, bytes32 nullifier) = prover.main(alice, signature);
        assert(balance > 0);
    }

    function test_revertProve() public {
        (address alice, uint256 alicePk) = makeAddrAndKey("alice");
        (address joe, uint256 joePk) = makeAddrAndKey("joe");
        address[] memory initialOwners = new address[](2);
        initialOwners[0] = alice;
        initialOwners[1] = joe;

        bytes32 hash = keccak256(
            abi.encodePacked("\x19Ethereum Signed Message:\n", Strings.toString(bytes("erc20 prover").length), "erc20 prover")
        );
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(joePk, hash);

        bytes memory signature = abi.encodePacked(r, s, v); 

        ExampleToken token = new ExampleToken(initialOwners);
        PrivateAirdropProver prover = new PrivateAirdropProver(token);
        vm.expectRevert(bytes("Invalid Signature"));
        prover.main(alice, signature);
    }    
}
