#!/bin/bash
# Профилирование бинарника demo с помощью heaptrack

set -e

export PATH="$HOME/.cargo/bin:$PATH"

if ! command -v heaptrack &>/dev/null; then
    echo "Ошибка: heaptrack не установлен" >&2
    exit 1
fi

LOGDIR="${1:-$(dirname "$0")/../artifacts/profiling}"
mkdir -p "$LOGDIR"

cargo build --release -p broken-app

BIN="target/release/profile_workload"
if [ ! -x "$BIN" ]; then
    echo "Бинарник $BIN не найден" >&2
    exit 1
fi

heaptrack -o "$LOGDIR/heaptrack" "$BIN"

if command -v heaptrack_print &>/dev/null; then
    heaptrack_print "$LOGDIR/heaptrack.gz" > "$LOGDIR/heaptrack_report.txt" 2>&1
    echo "Отчёт heaptrack: $LOGDIR/heaptrack_report.txt"
else
    echo "Для просмотра результатов выполните: heaptrack_gui $LOGDIR/heaptrack.gz"
fi