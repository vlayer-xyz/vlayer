// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

contract L2State {
    struct Anchor {
        bytes32 root;
        uint256 l2BlockNumber;
    }

    uint256 internal padding;
    mapping(uint32 => Anchor) public anchors;

    constructor(bytes32 root, uint256 l2BlockNumber) {
        padding = 1;
        anchors[0] = Anchor(root, l2BlockNumber);
    }
}
