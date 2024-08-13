#!/bin/bash

if [ -z "$EXPECTED_LLVM_VERSION" ]; then
  echo "Error: EXPECTED_LLVM_VERSION is not defined."
  exit 1
fi

if ! command -v clang &> /dev/null; then
    echo "Error: clang command not found"
    exit 1
fi

LLVM_VERSION=$(llvm-config --version)
if [[ "$LLVM_VERSION" != "$EXPECTED_LLVM_VERSION"* ]]; then
    echo "Error: LLVM version mismatches $EXPECTED_LLVM_VERSION. Found version: $LLVM_VERSION"
    exit 1
fi
echo "clang available and LLVM version starts with $EXPECTED_LLVM_VERSION"
