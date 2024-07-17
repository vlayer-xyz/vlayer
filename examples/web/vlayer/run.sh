#!/bin/bash

bun install
forge build

anvil &
anvil_pid=$!

vlayer serve &
vlayer_pid=$!

sleep 2

(cd ..; ../../bash/vlayer-deploy.sh web)

bun run index.ts

kill $anvil_pid
kill $vlayer_pid
