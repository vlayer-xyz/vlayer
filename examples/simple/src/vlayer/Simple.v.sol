// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract Simple {

    uint256 public latestSum;

    function sum(uint256 lhs, uint256 rhs) public pure returns (uint256) {
        return lhs + rhs;
    }


    function updateSum(bytes calldata _proof, uint256 _sum, bytes32 _journalHash) public {

        assert(keccak256(_proof) == _journalHash);
        
        latestSum = _sum;
    }

}
