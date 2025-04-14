// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

library Precompiles {
    address public constant VERIFY_AND_PARSE = address(0x100);
    address public constant VERIFY_EMAIL = address(0x101);
    address public constant JSON_GET_STRING = address(0x102);
    address public constant JSON_GET_INT = address(0x103);
    address public constant JSON_GET_BOOL = address(0x104);
    address public constant JSON_GET_ARRAY_LENGTH = address(0x105);
    address public constant REGEX_MATCH = address(0x110);
    address public constant REGEX_CAPTURE = address(0x111);
    address public constant URL_PATTERN_TEST = address(0x120);
    address public constant IS_VLAYER_TEST = address(0x130);
}
