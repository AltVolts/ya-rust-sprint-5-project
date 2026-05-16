#!/bin/bash
export PATH="$HOME/.cargo/bin:$PATH"

# Папка для логов: первый аргумент, или "artifacts" рядом со скриптом
LOGDIR="${1:-$(dirname "$0")/../artifacts}"
mkdir -p "$LOGDIR"

cargo test -p broken-app > "$LOGDIR/tests.log" 2>&1