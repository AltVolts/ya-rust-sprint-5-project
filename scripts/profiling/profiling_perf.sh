#!/bin/bash
# Профилирование бинарника demo с помощью perf

set -e

export PATH="$HOME/.cargo/bin:$PATH"

LOGDIR="${1:-$(dirname "$0")/../../artifacts/profiling}"
mkdir -p "$LOGDIR"

cargo build --profile profiling -p broken-app

BIN="target/profiling/profile_workload"
if [ ! -x "$BIN" ]; then
    echo "Ошибка: $BIN не найден" >&2
    exit 1
fi

perf record -o "$LOGDIR/perf.data" -B -F 9999  --call-graph dwarf "$BIN" < /dev/null
perf report -i "$LOGDIR/perf.data" --stdio > "$LOGDIR/perf_report.txt" 2>&1
echo "Готово: $LOGDIR/perf_report.txt"