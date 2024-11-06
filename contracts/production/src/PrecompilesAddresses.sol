// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

library Precompiles {
    address public constant VERIFY_AND_PARSE_PRECOMPILE = address(0x100);
    address public constant VERIFY_EMAIL_PRECOMPILE = address(0x101);
    address public constant JSON_GET_STRING_PRECOMPILE = address(0x102);
    address public constant JSON_GET_INT_PRECOMPILE = address(0x103);
    address public constant JSON_GET_BOOL_PRECOMPILE = address(0x104);
    address public constant JSON_GET_ARRAY_LENGTH = address(0x105);
    address public constant REGEX_MATCH_PRECOMPILE = address(0x110);
    address public constant REGEX_CAPTURE_PRECOMPILE = address(0x111);
}
