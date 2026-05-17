use broken_app::{algo, leak_buffer, normalize, sum_even};

fn main() {
    let _ = algo::slow_fib(31);

    let iterations = 50000;
    for _ in 0..iterations {
        let nums = [1, 2, 3, 4, 5, 6];
        let _ = sum_even(&nums);

        let data = [1_u8, 0, 2, 3];
        let _ = leak_buffer(&data);

        let text = " Hello World\t\n";
        let _ = normalize(text);

        let v = vec![1, 2, 2, 3, 1, 4, 4, 5, 5, 6];
        let _ = algo::slow_dedup(&v);
    }
}
