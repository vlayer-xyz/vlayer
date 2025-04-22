// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

/// @dev Addresses follow the EIP-7201 scheme:
///      keccak256(keccak256("vlayer.precompiles") - 1), truncated to 19 bytes,
///      with a 1-byte suffix appended to identify each precompile.
///      The `PRECOMPILES` constant is the base address (ending with 0x00),
///      and others are defined relative to it using fixed offsets.
///      Address generation is implemented in Rust via `generate_precompile!`.
library Precompiles {
    /// @dev Base precompile address: first 19 bytes of keccak256(keccak256("vlayer.precompiles") - 1) + 0x00 suffix
    address public constant PRECOMPILES = 0xF4E4FdcA9d5D55e64525e314391996a15f7EC600;
    address public constant VERIFY_AND_PARSE = address(uint160(PRECOMPILES) + 0);
    address public constant VERIFY_EMAIL = address(uint160(PRECOMPILES) + 1);
    address public constant JSON_GET_STRING = address(uint160(PRECOMPILES) + 2);
    address public constant JSON_GET_INT = address(uint160(PRECOMPILES) + 3);
    address public constant JSON_GET_BOOL = address(uint160(PRECOMPILES) + 4);
    address public constant JSON_GET_FLOAT_AS_INT = address(uint160(PRECOMPILES) + 5);
    address public constant REGEX_MATCH = address(uint160(PRECOMPILES) + 0x10);
    address public constant REGEX_CAPTURE = address(uint160(PRECOMPILES) + 0x11);
    address public constant URL_PATTERN_TEST = address(uint160(PRECOMPILES) + 0x20);
    address public constant IS_VLAYER_TEST = address(uint160(PRECOMPILES) + 0x1E);
}
