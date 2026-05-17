#!/bin/bash
# Запускает все инструменты профилирования последовательно.
# Использование: ./run_profiling.sh [папка_для_логов]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROFILING_DIR="$SCRIPT_DIR/profiling"

LOGDIR="${1:-$SCRIPT_DIR/../artifacts/profiling}"
mkdir -p "$LOGDIR"

echo "=============================================="
echo "  1/4  Perf (profile_workload)"
echo "=============================================="
if [ -x "$PROFILING_DIR/profiling_perf.sh" ]; then
    "$PROFILING_DIR/profiling_perf.sh" "$LOGDIR"
else
    echo "ОШИБКА: скрипт profiling_perf.sh не найден или неисполняемый"
fi

echo ""
echo "=============================================="
echo "  2/4  Flamegraph (profile_workload)"
echo "=============================================="
if [ -x "$PROFILING_DIR/profiling_flamegraph.sh" ]; then
    "$PROFILING_DIR/profiling_flamegraph.sh" "$LOGDIR"
else
    echo "ОШИБКА: скрипт profiling_flamegraph.sh не найден или неисполняемый"
fi

echo ""
echo "=============================================="
echo "  3/4  Heaptrack (profile_workload)"
echo "=============================================="
if [ -x "$PROFILING_DIR/profiling_heaptrack.sh" ]; then
    "$PROFILING_DIR/profiling_heaptrack.sh" "$LOGDIR"
else
    echo "ОШИБКА: скрипт profiling_heaptrack.sh не найден или неисполняемый"
fi

echo ""
echo "=============================================="
echo "  4/4  Massif (profile_workload)"
echo "=============================================="
if [ -x "$PROFILING_DIR/profiling_massif.sh" ]; then
    "$PROFILING_DIR/profiling_massif.sh" "$LOGDIR"
else
    echo "ОШИБКА: скрипт profiling_massif.sh не найден или неисполняемый"
fi

echo ""
echo "Все профилировочные отчёты сохранены в $LOGDIR"