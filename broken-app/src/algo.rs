pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let mut out = values.to_vec();
    out.sort();
    out.dedup();
    out
}

pub fn slow_fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0;
            let mut b = 1;
            for _ in 2..=n {
                let next = a + b;
                a = b;
                b = next;
            }
            b
        }
    }
}
