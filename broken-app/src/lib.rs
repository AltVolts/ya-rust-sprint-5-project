pub mod algo;
pub mod concurrency;

pub fn sum_even(values: &[i64]) -> i64 {
    values.iter().filter(|x| *x % 2 == 0 ).sum()
}

pub fn leak_buffer(input: &[u8]) -> usize {
    input.iter().filter(|x| **x != 0).count()
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

    #[test]
    fn regression_leak_buffer_correct_count() {
        // Обычный случай
        assert_eq!(leak_buffer(&[0, 1, 0, 2, 3]), 3);
        // Все нули
        assert_eq!(leak_buffer(&[0, 0, 0]), 0);
        // Пустой вход
        assert_eq!(leak_buffer(&[]), 0);
        // Все ненулевые
        assert_eq!(leak_buffer(&[1, 2, 3]), 3);
        // Большой буфер (раньше утечка была пропорциональна размеру)
        let big = vec![1u8; 10_000];
        assert_eq!(leak_buffer(&big), 10_000);
    }
}