// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

library Precompiles {
    // Hashed from: vlayer.precompiles.web_proof
    address public constant VERIFY_AND_PARSE = 0x95c8504add469381E88aEa9Db3BAB9a9BDFf857b;

    // Hashed from: vlayer.precompiles.email_proof
    address public constant VERIFY_EMAIL = 0x73d0Db8dE43551380021cbbAddeE85B385A6B0dD;

    // Hashed from: vlayer.precompiles.json_get_string
    address public constant JSON_GET_STRING = 0x6d07A0585656d3CC8106cF095F54aEf4E4F6Ca3A;

    // Hashed from: vlayer.precompiles.json_get_int
    address public constant JSON_GET_INT = 0xd8AAA782188aCcA9A773ff08FA7bb996059DCa41;

    // Hashed from: vlayer.precompiles.json_get_bool
    address public constant JSON_GET_BOOL = 0x527ea1cfF79264A4a1d7522C57550DEd36Ea6679;

    // Hashed from: vlayer.precompiles.json_get_array_length
    address public constant JSON_GET_ARRAY_LENGTH = 0x858AFe9948f30E463CaE2a3FFf28C687f326D81A;

    // Hashed from: vlayer.precompiles.regex_is_match
    address public constant REGEX_MATCH = 0xb33b6A2c6974Caf72FC5E2412011c77AE6A1aB56;

    // Hashed from: vlayer.precompiles.regex_capture
    address public constant REGEX_CAPTURE = 0x1B6700095D2191b5DFAEBeE63B45e992d6bdAd10;

    // Hashed from: vlayer.precompiles.url_pattern_test
    address public constant URL_PATTERN_TEST = 0x0dBfb79Dc9520274891c1ff7EdbC6Aa76D998348;

    // Hashed from: vlayer.precompiles.is_vlayer_test
    address public constant IS_VLAYER_TEST = 0x0C64EcBf8e8444ED3dC6d09F4dA812DB20c182A1;
}
