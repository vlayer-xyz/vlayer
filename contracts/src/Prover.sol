// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract Prover {

    address constant TRAVELER = 0x5b2063246fAa1061741463a4A6f74e3CB39A7C5B;

    function setBlock(uint blockNo) public {
        (bool success, ) = TRAVELER.call(abi.encodeWithSignature("setBlock(uint)", blockNo));
        require(success, "setBlock call failed");
    }

    function setChain(uint chainId, uint blockNo) public {
        (bool success, ) = TRAVELER.call(abi.encodeWithSignature("setChain(uint, uint)", chainId, blockNo));
        require(success, "setChain call failed");
    }
}
