use broken_app::algo;

fn main() {
    for _ in 0..5 {
        let _ = algo::slow_fib(40);
    }

    let dedup_data: Vec<u64> = (0..20_000).flat_map(|n| [n, n]).collect();
    for _ in 0..5 {
        let _ = algo::slow_dedup(&dedup_data);
    }
}