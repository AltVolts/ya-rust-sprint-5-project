#!/bin/bash
# Профилирование бинарника demo с помощью massif

set -e

export PATH="$HOME/.cargo/bin:$PATH"

if ! command -v valgrind &>/dev/null; then
    echo "Ошибка: valgrind не установлен. Установите: sudo apt install valgrind" >&2
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

valgrind --tool=massif \
    --massif-out-file="$LOGDIR/massif.out.%p" \
    --time-unit=B \
    --detailed-freq=1 \
    "$BIN"

ms_print "$LOGDIR"/massif.out.* > "$LOGDIR/massif_report.txt"
echo "Massif-отчёт сохранён в $LOGDIR/massif_report.txt"
