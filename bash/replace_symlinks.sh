#!/bin/bash

TARGET_DIR=$(pwd)

# Find all symbolic links in the current directory and subdirectories
find "$TARGET_DIR" -type l | while read -r symlink; do
  # Get the target the symlink points to
  target=$(readlink "$symlink")
  
  # If the target is a relative path, resolve it to an absolute path
  if [[ "$target" != /* ]]; then
    target=$(dirname "$symlink")/"$target"
  fi  
  # Remove the symlink
  rm "$symlink"
  
  if [[ -d "$target" ]]; then
    cp -r "$target" "$symlink"
  else
    cp "$target" "$symlink"
  fi

  echo "Replaced symlink $symlink with actual file $target"
done
