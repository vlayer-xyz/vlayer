#!/bin/bash
set -ueo pipefail

output_dir="out"
archive="${output_dir}/contracts.tar"

# Create the output directory if it doesn't exist
mkdir -p $output_dir

if [[ -f $archive ]]; then
    rm $archive
fi

touch $archive

for dir in examples/*/src/vlayer/
do
    if [[ -d $dir ]]; then
        tar --append --file=$archive --transform 's|examples/||;s|/src||' $dir
    fi
done

gzip -f $archive
