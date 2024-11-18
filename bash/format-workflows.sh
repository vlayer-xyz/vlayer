#!/usr/bin/env bash

# Exit on errors, undefined variables, or pipeline failures
set -ueo pipefail

usage() {
  echo "Usage: $0 [--write]"
  echo "  --write   Fix files instead of just checking them"
  exit 1
}

# Default Prettier command
PRETTIER_CMD="prettier --check .github/workflows/**/*.yaml"

# Parse arguments
if [ "$#" -gt 0 ]; then
  if [ "$1" == "--write" ]; then
    PRETTIER_CMD="prettier --write .github/workflows/**/*.yaml"
  else
    usage
  fi
fi

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
  echo "Error: Node.js is not installed. Please install Node.js and try again."
  exit 1
fi

# Check if Prettier is installed
if ! command -v prettier &> /dev/null; then
  echo "Prettier is not installed."

  # Prompt the user for installation
  read -p "Do you want to install Prettier globally? (y/N): " install_prettier
  install_prettier=${install_prettier:-n}

  if [[ "$install_prettier" =~ ^[Yy]$ ]]; then
    echo "Installing Prettier globally..."
    npm install --global prettier || {
      echo "Error: Failed to install Prettier. Please check your npm setup and try again."
      exit 1
    }
  else
    echo "Prettier is required to run this script. Exiting."
    exit 1
  fi
fi

echo "Running Prettier..."
$PRETTIER_CMD

echo "Prettier command completed successfully."
