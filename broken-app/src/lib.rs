pub mod algo;
pub mod concurrency;

pub fn sum_even(values: &[i64]) -> i64 {
    values.iter().filter(|x| *x % 2 == 0 ).sum()
}

pub fn leak_buffer(input: &[u8]) -> usize {
    input.iter().filter(|x| **x != 0).count()
}

pub fn normalize(input: &str) -> String {
    input
        .split_whitespace()
        .collect::<String>()
        .to_lowercase()
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

pub fn use_after_free() -> i32 {
    let b = Box::new(42_i32);
    let val = *b;
    // Box автоматически освободится при выходе из функции
    val // возвращаем значение до освобождения
}


#[cfg(test)]
mod tests {
    use super::*;

    /// Регрессионный тест: раньше был выход за границу при любом вызове
    #[test]
    fn regression_test_off_by_one_no_panic() {
        let data = [1, 2, 3];
        let result = sum_even(&data);
        assert_eq!(result, 2); // 2 — единственное чётное
    }

    /// Дополнительно: проверяем пустой срез — граница len=0,
    /// и цикл `0..=0` тоже выходил за границу
    #[test]
    fn regression_test_empty_slice() {
        let data: [i64; 0] = [];
        let result = sum_even(&data);
        assert_eq!(result, 0);
    }

    #[test]
    fn regression_test_average_positive_positive_absence() {
        // проверка на отсутсвие положительных чисел
        assert!((average_positive(&[-1, -2, -3])).abs() < f64::EPSILON);
        // проверка на пустой массив
        assert!((average_positive(&[])).abs() < f64::EPSILON);
        // проверка на массив нулей
        assert!((average_positive(&[0; 3])).abs() < f64::EPSILON);
    }

    #[test]
    fn regression_test_leak_buffer_correct_count() {
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

    #[test]
        fn regression_test_normalize() {
            // базовый случай – только пробелы
            assert_eq!(normalize("Hello World"), "helloworld");
            // табуляция должна быть удалена
            assert_eq!(normalize("a\tb"), "ab");
            // перевод строки должен быть удалён
            assert_eq!(normalize("line1\nline2"), "line1line2");
            // возврат каретки
            assert_eq!(normalize("word\rnext"), "wordnext");
            // смесь пробелов, табуляций, переносов
            let input = "  Hello \t World \n Rust  ";
            let expected = "helloworldrust";
            assert_eq!(normalize(input), expected);
            // "пустой" случай
            assert_eq!(normalize(""), "");
            // полный случай
            assert_eq!(normalize("   \t \n  "), "");
        }
}