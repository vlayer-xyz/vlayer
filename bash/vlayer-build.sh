#!/usr/bin/env bash
set -uexo pipefail

OUTPATH=$(pwd)/out/_vlayer

mkdir -p ${OUTPATH}

git clone --no-checkout https://github.com/vlayer-xyz/vlayer.git ${OUTPATH}
cd ${OUTPATH}
git sparse-checkout init --cone
git sparse-checkout set rust/template
git checkout main
