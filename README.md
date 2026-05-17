# Проектная работа модуля 5. Поиск ошибок и оптимизация

## Отработка этапов отладки (`Debugging`), профилирования (`Profiling`) и бенчмаркинга (`Benchmarking`) приложения `broken_app`

- Перед первым запуском скриптов из директории `./scripts`, возможно, потребуется выполнить:

```bash
    chmod +x scripts/*.sh scripts/debugging/*.sh scripts/profiling/*.sh scripts/benchmarking/*.sh
```

**Предупреждение!**

Воспроизведение некоторых тестов и профилировщиков может быть затруднено 
на операционных системах, отличных от Linux (например, Windows без WSL). 
На macOS большинство инструментов доступны, но могут требовать дополнительной настройки 
(например, `perf` отсутствует, вместо него используются Instruments или DTrace). 
Убедитесь, что необходимые утилиты установлены и работают в вашем окружении.

## Debugging

Первоначальный запуск тестов с применением инструментов отладки (`gdb`, `miri`, `Valgrind`) выявил несколько проблем:

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
3) `valgrind.log` - результат Valgrind с флагом `--leak-check=full` (Утечка `possibly lost: 48 bytes in 1 blocks` является ложным срабатыванием `Valgrind` при работе с `Rust`);
4) `asan.log` - результат запуска с AddressSanitizer;
5) `tsan.log` - результат запуска с ThreadSanitizer.

### Воспроизведение результатов тестов

Скрипт `./scipts/run_debugging_tests.sh` последовательно запускает пять инструментов
(обычные тесты, `Miri`, `Valgrind`, `AddressSanitizer`, `ThreadSanitizer`) и сохраняет
логи в указанную папку (по умолчанию `artifacts/debugging/after`).

**Использование:**

- Запуск с папкой логов по умолчанию (`artifacts/debugging/after` в корне проекта):

```bash
./scripts/run_debugging_tests.sh
```

- С явным указанием папки для логов:

```bash
./scripts/run_debugging_tests.sh путь/к/моей/папке
```

### Требования для работы отладочных скриптов:

- Убедитесь, что у вас установлен `nightly toolchain` (```rustup toolchain install nightly```)!
- Если ваша целевая платформа отличается от `x86_64-unknown-linux-gnu`,
  замените параметр `--target` в скриптах `asan_tests.sh` и `tsan_tests.sh` под ваши условия (можно узнать командой ```rustc -vV``` в переменной `host`).


## Profiling

### Профилирование через `profile_workload.rs` c использованием профиля `profiling` (наследует `release`, но с `debug=true` и `strip=false`)

Так как `demo.rs` выполняется слишком быстро и `perf` не успевает собрать достаточно сэмплов для построения графа, был создан `profile_workload.rs`.
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

- Все скрипты используют `target/profiling/profile_workload`, который создаётся
автоматически командой ```cargo build --profile profiling -p broken-app```.

- Результаты, если не указывать папку для логов, сохраняются в `artifacts/profiling/`:

1) `perf_report.txt` – текстовый отчёт `perf` с сортировкой по затратам времени;
2) `flamegraph.svg` – визуализация стека вызовов (открывается в браузере);
3) `heaptrack_report.txt` – анализ аллокаций памяти;
4) `massif_report.txt` – график использования кучи.

### Требования для профилирования:

- Установлен `perf` (обычно в пакете `linux-tools` или `perf`).
- Для `flamegraph` рекомендуется установить `cargo-flamegraph` (`cargo install flamegraph`).
- Для `heaptrack`: ```sudo apt install heaptrack``` (или собрать из исходников).
- `Valgrind` (`massif`) должен быть установлен.

### Основные выводы профилирования

- Основное время выполнения `profile_workload` приходится на функциb `slow_dedup()` и `slow_fib()`.
- Основная аллокация связана с созданием вектора перед вызовом `slow_dedup()`.

## Benchmarking

Для первичного выявления «горячих» функций используется бинарник `broken_app/benches/baseline.rs`
(запускается через ```cargo bench -p broken-app --bench baseline```). Он производит по три замера 
для каждой публичной функции на больших входных данных и выводит время в консоль.

**Вывод `baseline.rs`:**

```text
    Function                  Avg time (µs)
    ----------------------------------------
    slow_dedup                   7684.66
    slow_fib                     4332.38
    sum_even                       11.67
    normalize                       5.68
    average_positive                3.24
    leak_buffer                     2.87
    ----------------------------------------
    Share of total time:
      slow_dedup                 63.8%
      slow_fib                   36.0%
      sum_even                    0.1%
      normalize                   0.0%
      average_positive            0.0%
      leak_buffer                 0.0%
```

- Видно, что основные цели для оптимизации — функции `slow_dedup()` и `slow_fib()`

Бенчмарки реализованы в `broken_app/benches/criterion.rs` и покрывают все функции `broken_app` (на будущее) , включая «горячие» участки (`slow_fib`, `slow_dedup`).

### Запуск бенчмарков «до» оптимизации

```bash
./scripts/benchmarks/run_bench_before.sh  [папка_для_логов]
```

Результаты в `artifacts/benchmarking/before/`:

- `bench_results_before.txt` – текстовый вывод `Criterion`:

```text
test sum_even_broken ... bench:        7888 ns/iter (+/- 31)

test leak_buffer_broken ... bench:        2781 ns/iter (+/- 14)

test normalize_broken ... bench:        2478 ns/iter (+/- 27)

test average_positive_broken ... bench:        3222 ns/iter (+/- 44)

test slow_fib_broken ... bench:     4023556 ns/iter (+/- 13399)

test slow_dedup_broken ... bench:    10195102 ns/iter (+/- 78415)
```

- `html/` – полный HTML-отчёт `Criterion`.

### Запуск бенчмарков «после» оптимизации

```bash
./scripts/benchmarks/run_bench_after.sh [папка_для_логов]
```

Результаты в `artifacts/benchmarking/after/`:

- `bench_results_after.txt` – текстовый вывод `Criterion`:

```text
test sum_even_broken ... bench:        8569 ns/iter (+/- 43)

test leak_buffer_broken ... bench:        3030 ns/iter (+/- 14)

test normalize_broken ... bench:        2715 ns/iter (+/- 18)

test average_positive_broken ... bench:        3281 ns/iter (+/- 7)

test slow_fib_broken ... bench:          14 ns/iter (+/- 0)

test slow_dedup_broken ... bench:       15275 ns/iter (+/- 3172)
```

- `html/` – полный HTML-отчёт `Criterion`.

### Итоги оптимизаций

На основании быстрого замера (`baseline`) и данных профилировщиков основное время
выполнения приходилось на две функции: `slow_dedup` (~64%) и `slow_fib` (~36%).
Остальные функции (`sum_even`, `leak_buffer`, `normalize`, `average_positive`)
занимали пренебрежимо малую долю и не требовали глубокой переработки.

**Проведённые оптимизации**

1. **`slow_fib()`** – алгоритмическая оптимизация: рекурсивный расчёт чисел Фибоначчи
   заменён на итеративный с линейной сложностью (O(n) вместо экспоненциальной).
2. **`slow_dedup()`** –  алгоритмическая оптимизация: квадратичный алгоритм удаления
  дубликатов заменён на сортировку с последующим применением стандартного метода
  `dedup()` (сложность O(n log n)). Это устранило многократные сортировки
3. Микрооптимизации в остальных функциях (использование итераторов, уменьшение
   клонирований) были сделаны на этапе отладки, поэтому не были учтены.

**Результаты бенчмарков (Criterion, release-сборка)**

| Функция              | Время до оптимизации (ns/iter) | Время после (ns/iter) | Ускорение |
|----------------------|--------------------------------|-----------------------|-----------|
| `slow_fib_broken`    | 4 023 556                      | 14                    | **~310 000×** |
| `slow_dedup_broken`  | 10 195 102                     | 15 275                | **~637×**  |

- `slow_fib()` стал практически мгновенным (14 наносекунд), ускорение достигнуто за счёт смены алгоритма.
- `slow_dedup()` ускорен в ~637 раз, что кардинально снижает общее время работы
  на больших массивах.

**Подтверждение корректности после оптимизаций**

После внесения изменений повторно запущены все проверки:

- `cargo test` – все регрессионные и интеграционные тесты проходят.
- `cargo +nightly miri test` – не обнаружено неопределённого поведения.
- `Valgrind` (`--leak-check=full`) – утечки памяти отсутствуют
- `AddressSanitizer` и `ThreadSanitizer` (сборка с `-Zsanitizer=address` / `thread`)
  ошибок не выявили.

Таким образом, ключевые узкие места устранены, корректность сохранена, а общее
время выполнения горячих участков сократилось на несколько порядков.