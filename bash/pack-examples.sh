#!/bin/bash
set -e
# Define the output file
archive="contracts.tar"

if [[ -f $archive ]]; then
    rm $archive
fi

touch $archive

for dir in examples/*/src/vlayer/
do
    if [[ -d $dir ]]; then
        tar --append -f $archive -C $dir .
    fi
done

gzip $archive

