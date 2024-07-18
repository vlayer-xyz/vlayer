#!/bin/bash
set -ueo pipefail

archive="contracts.tar.gz"

temp_dir="temp_dir"

mkdir -p $temp_dir/vlayer

if [[ -f $archive ]]; then
    rm $archive
fi

for dir in examples/*/src/vlayer/
do
    if [[ -d $dir ]]; then
        cp -r $dir* $temp_dir/vlayer/
    fi
done

tar -czf $archive -C $temp_dir .

# Remove the temporary directory
rm -rf $temp_dir