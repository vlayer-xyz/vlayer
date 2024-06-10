#!/usr/bin/env bash
set -uexo pipefail

TMP=$(mktemp -d)
OUTPATH=$(pwd)/out/_vlayer

mkdir -p ${OUTPATH}

git clone https://github.com/vlayer-xyz/vlayer.git ${TMP}
cp -r ${TMP}/rust/template ${OUTPATH}/rust
