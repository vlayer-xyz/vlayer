#!/bin/bash
set -e

bun install

anvil &
anvil_pid=$!

vlayer serve &
vlayer_pid=$!

sleep 5

bun run index.ts

kill $anvil_pid
kill $vlayer_pid
