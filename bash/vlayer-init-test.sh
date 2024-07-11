#!/usr/bin/env bash

cd $(mktemp -d)
forge init myproject --no-commit
cd myproject

vlayer init
test -d src/vlayer
test -f src/vlayer/Simple.v.sol

vlayer init | grep -q "ERROR"
if [ $? -ne 0 ]; then
exit 1
fi