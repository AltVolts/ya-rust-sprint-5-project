#!/bin/bash
# Запускает все инструменты отладки последовательно.
# Использование: ./run_debugging_tests.sh [папка_для_логов]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEBUG_DIR="$SCRIPT_DIR/debugging"

LOGDIR="${1:-$SCRIPT_DIR/../artifacts/debugging/after}"
mkdir -p "$LOGDIR"

echo "=============================================="
echo "  1/5  Обычные тесты (cargo test)"
echo "=============================================="
if [ -x "$DEBUG_DIR/cargo_tests.sh" ]; then
    "$DEBUG_DIR/cargo_tests.sh" "$LOGDIR"
else
    echo "ОШИБКА: скрипт cargo_tests.sh не найден или неисполняемый"
fi

echo ""
echo "=============================================="
echo "  2/5  Miri (проверка небезопасного кода)"
echo "=============================================="
if [ -x "$DEBUG_DIR/miri_tests.sh" ]; then
    "$DEBUG_DIR/miri_tests.sh" "$LOGDIR"
else
    echo "ОШИБКА: скрипт miri_tests.sh не найден или неисполняемый"
fi

echo ""
echo "=============================================="
echo "  3/5  Valgrind (утечки памяти)"
echo "=============================================="
if [ -x "$DEBUG_DIR/valgrind_tests.sh" ]; then
    "$DEBUG_DIR/valgrind_tests.sh" "$LOGDIR"
else
    echo "ОШИБКА: скрипт valgrind_tests.sh не найден или неисполняемый"
fi

echo ""
echo "=============================================="
echo "  4/5  AddressSanitizer"
echo "=============================================="
if [ -x "$DEBUG_DIR/asan_tests.sh" ]; then
    "$DEBUG_DIR/asan_tests.sh" "$LOGDIR"
else
    echo "ОШИБКА: скрипт asan_tests.sh не найден или неисполняемый"
fi

echo ""
echo "=============================================="
echo "  5/5  ThreadSanitizer"
echo "=============================================="
if [ -x "$DEBUG_DIR/tsan_tests.sh" ]; then
    "$DEBUG_DIR/tsan_tests.sh" "$LOGDIR"
else
    echo "ОШИБКА: скрипт tsan_tests.sh не найден или неисполняемый"
fi

echo ""
echo "Все отладочные проверки завершены. Логи в $LOGDIR"