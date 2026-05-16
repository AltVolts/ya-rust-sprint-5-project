#!/bin/bash
export PATH="$HOME/.cargo/bin:$PATH"

LOGDIR="${1:-$(dirname "$0")/../artifacts}"
mkdir -p "$LOGDIR"

cargo +nightly miri test -p broken-app > "$LOGDIR/miri.log" 2>&1