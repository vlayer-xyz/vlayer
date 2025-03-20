#!/usr/bin/env bash
set -euo pipefail

cargo +nightly fmt --all --check || {
    echo "❌ Formatting issues detected!"
    exit 1
}
echo "✅ Formatting check passed!"

cargo sort --check --grouped --workspace || {
    echo "❌ Dependency sorting issues detected!"
    exit 1
}
echo "✅ Dependency sorting check passed!"

cargo clippy --all-targets --all-features --locked -- -D warnings || {
    echo "❌ Clippy found issues!"
    exit 1
}
echo "✅ Clippy check passed!"

echo "All checks passed successfully"
