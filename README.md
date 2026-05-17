# Проектная работа модуля 5. Поиск ошибок и оптимизация

## Отработка этапов отладки(debugging), профилирования и бенчмаркинга приложения broken_app

- Сделайте скрипты исполняемыми перед первым запуском:

```bash
    chmod +x scripts/*.sh scripts/debugging/*.sh scripts/profiling/*.sh
```

## Debugging

1) `off-by-one` в `sum_even()` – устранён выход за границу;
2) `Use-after-free` в `use_after_free()` – теперь значение копируется до освобождения `Box`, `UB` устранено;
3) Утечка в `leak_buffer()` – больше не аллоцирует временный буфер;
4) Неверная логика в `average_positive()` – фильтрация только положительных и корректное деление;
5) Нормализация `normalize()` – покрывает все пробельные символы, что соответствуют тестам;
6) Исправление гонки данных в модуле `concurrency.rs` - использование атомарного счётчика, добавлен интеграционный тест для проверки `test_race_increment_correct_total()`.

### Итог: 

Были устранены как логические ошибки, так и ошибки неправильного использования памяти в однопоточных и многопоточных функциях. Добавлены регрессионные юнит- и интеграционные тесты.

Подтверждение корректности результатов изменений в `artifacts/debugging/after/`:

1) `tests.log` - результат простого `cargo test`;
2) `miri.log` - результат Miri;
3) `valgrind.log` - результат Valgrind с флагом `--leak-check=full`;
4) `asan.log` - результат запуска с AddressSanitizer;
5) `tsan.log` - результат запуска сThreadSanitizer.

### Воспроизведение результатов тестов

Скрипт `run_debugging_tests.sh` последовательно запускает пять инструментов
(обычные тесты, Miri, Valgrind, AddressSanitizer, ThreadSanitizer) и сохраняет
логи в указанную папку (по умолчанию `artifacts/debugging/after`).

**Использование:**

- Запуск с папкой логов по умолчанию (`artifacts/debugging/after` в корне проекта):

```bash
./scripts/run_debugging_tests.sh
```

- Явное указание папки для логов:

```bash
./scripts/run_debugging_tests.sh путь/к/моей/папке
```

### Требования для работы отладочных скриптов:

- Убедитесь, что у вас установлен nightly toolchain (```rustup toolchain install nightly```)!
- Если ваша целевая платформа отличается от x86_64-unknown-linux-gnu, 
  замените параметр --target в скриптах asan_tests.sh и tsan_tests.sh на ваш host-триплет (можно узнать командой rustc -vV).


## Profiling

### Профилирование через `profile_workload.rs`

Так как `demo.rs` выполняется слишком быстро и perf не успевает собрать достаточно сэмплов для построения графа, был создан `profile_workload.rs`.
Он содержит многократные вызовы функций `broken_app`, что обеспечивает стабильную нагрузку для сбора профилей производительности и памяти.

**Запуск всех профилировщиков одной командой:**

```bash
./scripts/run_profiling.sh [папка_для_логов]
```

**Индивидуальный запуск:**

```bash
./scripts/profiling/profiling_perf.sh [папка_для_логов]
./scripts/profiling/profiling_flamegraph.sh [папка_для_логов]
./scripts/profiling/profiling_heaptrack.sh [папка_для_логов]
./scripts/profiling/profiling_massif.sh [папка_для_логов]
```

- Все скрипты используют `target/release/profile_workload`, который создаётся
автоматически командой ```cargo build --release -p broken-app```.

- Результаты, если не указывать папку для логов, сохраняются в `artifacts/profiling/`:

1) `perf_report.txt` – текстовый отчёт perf с сортировкой по затратам времени;
2) `flamegraph.svg` – визуализация стека вызовов (открывается в браузере);
3) `heaptrack_report.txt` – анализ аллокаций памяти;
4) `massif_report.txt` – график использования кучи.

### Требования для профилирования:

- Установлен perf (обычно в пакете `linux-tools` или `perf`).
- Для `flamegraph` рекомендуется установить `cargo-flamegraph` (```cargo install flamegraph```).
- Для `heaptrack`: ```sudo apt install heaptrack``` (или собрать из исходников).
- `Valgrind` (`massif`) должен быть установлен.

### Основные выводы профилирования

- Основное временя выполнения `profile_workload` приходится на функцию `slow_fib()`.
- Остальное время распределяется между `slow_dedup()` и `normalize()`.
- Основная аллокация связана с созданием вектора перед вызовом slow_dedup



## Benchmarking

1) 