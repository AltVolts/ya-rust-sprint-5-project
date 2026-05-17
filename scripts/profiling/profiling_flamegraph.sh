#!/bin/bash
# Профилирование бинарника demo с помощью flamegraph

set -e

export PATH="$HOME/.cargo/bin:$PATH"

if ! command -v cargo-flamegraph &>/dev/null; then
    echo "Ошибка: cargo-flamegraph не установлен. Установите: cargo install flamegraph" >&2
    exit 1
fi

LOGDIR="${1:-$(dirname "$0")/../../artifacts/profiling}"
mkdir -p "$LOGDIR"

cargo flamegraph --profile profiling --bin profile_workload -p broken-app -o "$LOGDIR/flamegraph.svg"

rm -f perf.data

echo "Flamegraph сохранён в $LOGDIR/flamegraph.svg"