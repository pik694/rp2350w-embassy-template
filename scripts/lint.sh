#!/usr/bin/env bash
set -ueo pipefail

# If called via cargo subcommand, drop the "lint" argument
if [[ "${1:-}" == "lint" ]]; then
    shift
fi

CHECK_MODE="${1:-}"

if [[ "$CHECK_MODE" == "--check" ]]; then
    echo "Checking Rust linters..."
    FMT_FLAGS=(--check)
    SORT_FLAGS=(--check --check-format)
    CLIPPY_FLAGS=(-- -D warnings)
    MACHETE_FLAGS=()
    SUCCESS_MSG="✅ All checks passed!"
else
    echo "Running Rust linters..."
    FMT_FLAGS=()
    SORT_FLAGS=()
    CLIPPY_FLAGS=(--fix --allow-dirty --allow-staged)
    MACHETE_FLAGS=(--fix)
    SUCCESS_MSG="✅ All linters completed successfully!"
fi

echo "1. Running rustfmt..."
cargo fmt "${FMT_FLAGS[@]}"

echo "2. Running cargo-sort..."
cargo sort --grouped "${SORT_FLAGS[@]}"

echo "3. Running clippy..."
cargo clippy "${CLIPPY_FLAGS[@]}"

echo "4. Running cargo-machete..."
cargo machete "${MACHETE_FLAGS[@]}"

echo "$SUCCESS_MSG"
