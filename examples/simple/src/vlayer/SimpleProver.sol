// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Proof} from "vlayer-0.1.0/Proof.sol";
import {Prover} from "vlayer-0.1.0/Prover.sol";
import {IERC20} from "openzeppelin-contracts/token/ERC20/IERC20.sol";

contract SimpleProver is Prover {
    IERC20 immutable token;
    uint256 immutable blockNo;

    constructor(IERC20 _token, uint256 _blockNo) {
        token = _token;
        blockNo = _blockNo;
    }

    function balance(address _owner) public returns (Proof memory, address, uint256) {
        setBlock(blockNo);
        uint256 ownerBalance = token.balanceOf(_owner);

        return (proof(), _owner, ownerBalance);
    }
}
