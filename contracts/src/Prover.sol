// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

interface ITraveler {
    // These functions need to return something because otherwise Solidity compiler won't generate CALL opcode when they're called.
    function setBlock(uint256 blockNo) external returns (bool);
    function setChain(uint256 chainId, uint256 blockNo) external returns (bool);
}

contract Prover {
    // Address generated from first 20-bytes of "vlayer.traveler"'s keccak256.
    ITraveler constant TRAVELER = ITraveler(address(uint160(uint256(keccak256("vlayer.traveler"))))); // 0x76dc9aa45aa006a0f63942d8f9f21bd4537972a3

    function setBlock(uint256 blockNo) public {
        (bool success, ) = address(TRAVELER).call(abi.encodeWithSelector(ITraveler.setBlock.selector, blockNo));
        require(success, "Call to traveler should get intercepted and succeed");
    }

    function setChain(uint256 chainId, uint256 blockNo) public {
        (bool success, ) = address(TRAVELER).call(abi.encodeWithSelector(ITraveler.setChain.selector, chainId, blockNo));
        require(success, "Call to traveler should get intercepted and succeed");
    }
}
