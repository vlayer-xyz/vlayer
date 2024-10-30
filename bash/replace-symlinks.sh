#!/bin/bash

TARGET_DIR=$(pwd)

echo "Replacing symlinks in $TARGET_DIR"
# Find all symbolic links in the current directory and subdirectories
find "$TARGET_DIR" -type l | while read -r symlink; do
  # Get the target the symlink points to
  target=$(readlink "$symlink")

  if [[ "$target" != /* ]]; then
    target=$(dirname "$symlink")/"$target"
  fi

  #touch only web-proof-commons
  if [[ "$symlink" == *"web-proof-commons"* || "$target" == *"web-proof-commons"* ]]; then
    rm "$symlink"

    if [[ -d "$target" ]]; then
      cp -r "$target" "$symlink"
    else
      cp "$target" "$symlink"
    fi

    echo "Replaced symlink $symlink with actual file $target"
  fi
done
