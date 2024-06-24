// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import { Airdrop } from "../src/Airdrop.sol";

contract AirdropTest is Test {
    Airdrop public aidrop;

    function setUp() public {
        aidrop = new Airdrop();
        aidrop.setNumber(0);
    }

    function test_Increment() public {
        aidrop.increment();
        assertEq(aidrop.number(), 1);
    }

    function testFuzz_SetNumber(uint256 x) public {
        aidrop.setNumber(x);
        assertEq(aidrop.number(), x);
    }
}
