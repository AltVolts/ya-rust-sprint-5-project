#!/bin/bash

set -e

export PATH="$HOME/.cargo/bin:$PATH"

LOGDIR="${1:-$(dirname "$0")/../../artifacts/debugging}"
mkdir -p "$LOGDIR"

RUSTFLAGS="-Zsanitizer=address" \
cargo +nightly test -p broken-app --tests \
    -Zbuild-std --target x86_64-unknown-linux-gnu \
    > "$LOGDIR/asan.log" 2>&1