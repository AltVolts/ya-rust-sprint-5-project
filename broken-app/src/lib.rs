pub mod algo;
pub mod concurrency;

pub fn sum_even(values: &[i64]) -> i64 {
    values.iter().filter(|x| *x % 2 == 0 ).sum()
}

/// Подсчёт ненулевых байтов. Буфер намеренно не освобождается,
/// что приведёт к утечке памяти (Valgrind это покажет).
pub fn leak_buffer(input: &[u8]) -> usize {
    let boxed = input.to_vec().into_boxed_slice();
    let len = input.len();
    let raw = Box::into_raw(boxed) as *mut u8;

    let mut count = 0;
    unsafe {
        for i in 0..len {
            if *raw.add(i) != 0_u8 {
                count += 1;
            }
        }
        // утечка: не вызываем Box::from_raw(raw);
    }
    count
}

/// Небрежная нормализация строки: удаляем пробелы и приводим к нижнему регистру,
/// но игнорируем повторяющиеся пробелы/табуляции внутри текста.
pub fn normalize(input: &str) -> String {
    input.replace(' ', "").to_lowercase()
}

pub fn average_positive(values: &[i64]) -> f64 {
  let (sum, count) = values
    .iter()
    .filter(|&&x| x > 0)
    .fold((0i64, 0usize), |(sum, count), &x| {
        (sum + x, count + 1)
    });
  if count == 0 {
      0.0
  } else {
      sum as f64 / count as f64
  }
}

/// Use-after-free: возвращает значение после освобождения бокса.
/// UB, проявится под ASan/Miri.
pub unsafe fn use_after_free() -> i32 {
    let b = Box::new(42_i32);
    let raw = Box::into_raw(b);
    let val = *raw;
    drop(Box::from_raw(raw));
    val + *raw
}


#[cfg(test)]
mod tests {
    use super::*;

    /// Регрессионный тест: раньше был выход за границу при любом вызове
    #[test]
    fn regression_off_by_one_no_panic() {
        let data = [1, 2, 3];
        let result = sum_even(&data);
        assert_eq!(result, 2); // 2 — единственное чётное
    }

    /// Дополнительно: проверяем пустой срез — граница len=0,
    /// и цикл `0..=0` тоже выходил за границу
    #[test]
    fn regression_empty_slice() {
        let data: [i64; 0] = [];
        let result = sum_even(&data);
        assert_eq!(result, 0);
    }

    #[test]
    fn regression_average_positive_positive_absence() {
        // проверка на отсутсвие положительных чисел
        assert!((average_positive(&[-1, -2, -3])).abs() < f64::EPSILON);
        // проверка на пустой массив
        assert!((average_positive(&[])).abs() < f64::EPSILON);
        // проверка на массив нулей
        assert!((average_positive(&[0; 3])).abs() < f64::EPSILON);
    }
}