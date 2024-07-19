#!/bin/bash
set -e

archive="contracts.tar"

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

gzip $archive