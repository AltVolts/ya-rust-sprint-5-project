#!/bin/bash
export PATH="$HOME/.cargo/bin:$PATH"

LOGDIR="${1:-$(dirname "$0")/../artifacts}"
mkdir -p "$LOGDIR"

# Собираем тесты, но не запускаем
cargo test -p broken-app --tests --no-run

# Ищем самый свежий бинарник, исключая файлы .d
BIN=$(ls -t target/debug/deps/broken_app-* 2>/dev/null | grep -v '\.d$' | head -1)

# Проверяем, что нашли и файл исполняемый
if [ -z "$BIN" ] || [ ! -x "$BIN" ]; then
    echo "Error: test binary not found or not executable" > "$LOGDIR/valgrind.log"
    exit 1
fi

valgrind --leak-check=full --error-exitcode=1 "$BIN" > "$LOGDIR/valgrind.log" 2>&1