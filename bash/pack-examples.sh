#!/usr/bin/env bash

set -uexo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

output_dir="$(pwd)/out"
ARCHIVE="${output_dir}/examples.tar"

# Create the output directory if it doesn't exist
mkdir -p $output_dir

if [[ -f $ARCHIVE ]]; then
    rm $ARCHIVE
fi

touch $ARCHIVE

cd ${VLAYER_HOME}/examples

for example in $(find . -type d -maxdepth 1 -mindepth 1) ; do

    scripts="${example}/vlayer"
    contracts="${example}/src/vlayer"

    tar --append --file=$ARCHIVE --strip 1  --exclude-from .gitignore "${contracts}"
    tar --append --file=$ARCHIVE --strip 1  --exclude-from .gitignore "${scripts}"

done

gzip -f $ARCHIVE
