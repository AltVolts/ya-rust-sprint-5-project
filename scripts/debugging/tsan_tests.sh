#!/bin/bash

set -e

export PATH="$HOME/.cargo/bin:$PATH"

LOGDIR="${1:-$(dirname "$0")/../artifacts}"
mkdir -p "$LOGDIR"

RUSTFLAGS="-Zsanitizer=thread" \
cargo +nightly test -p broken-app --tests \
    -Zbuild-std --target x86_64-unknown-linux-gnu \
    > "$LOGDIR/tsan.log" 2>&1