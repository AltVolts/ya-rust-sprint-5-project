#!/bin/bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LOGDIR="${1:-$SCRIPT_DIR/../../artifacts/benchmarking/after}"
mkdir -p "$LOGDIR"

echo "==> Запуск бенчмарков (ПОСЛЕ оптимизаций) ..."
cargo bench -p broken-app --bench criterion -- --output-format bencher | tee "$LOGDIR/bench_results_after.txt"

if [ -d "target/criterion" ]; then
    echo "==> Копирование HTML-отчёта Criterion в $LOGDIR/html ..."
    rm -rf "$LOGDIR/html"
    cp -r target/criterion "$LOGDIR/html"
    echo "HTML-отчёт сохранён."
else
    echo "Предупреждение: target/criterion не найден."
fi