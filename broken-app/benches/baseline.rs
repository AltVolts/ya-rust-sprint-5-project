use broken_app::{
    algo::{slow_dedup, slow_fib},
    average_positive, leak_buffer, normalize, sum_even,
};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

fn time_it(mut f: impl FnMut()) -> Duration {
    let start = Instant::now();
    f();
    start.elapsed()
}

fn main() {
    let data: Vec<i64> = (0..50_000).collect();
    let fib_n = 32;
    let dedup_data: Vec<u64> = (0..5_000).flat_map(|n| [n, n]).collect();
    let leak_data: Vec<u8> = (0..10_000).map(|x| x as u8).collect();
    let text = "  Hello   \t World \n  ".repeat(100);
    let avg_nums: Vec<i64> = (-10..10_000).collect();

    let mut timings: HashMap<&str, Vec<std::time::Duration>> = HashMap::new();

    let runs = 3;
    for _ in 0..runs {
        timings.entry("sum_even").or_default().push(time_it(|| {
            let _ = sum_even(&data);
        }));

        timings.entry("leak_buffer").or_default().push(time_it(|| {
            let _ = leak_buffer(&leak_data);
        }));

        timings.entry("normalize").or_default().push(time_it(|| {
            let _ = normalize(&text);
        }));

        timings
            .entry("average_positive")
            .or_default()
            .push(time_it(|| {
                let _ = average_positive(&avg_nums);
            }));

        timings.entry("slow_fib").or_default().push(time_it(|| {
            let _ = slow_fib(fib_n);
        }));

        timings.entry("slow_dedup").or_default().push(time_it(|| {
            let _ = slow_dedup(&dedup_data);
        }));
    }

    let mut averages: Vec<(&str, f64)> = timings
        .iter()
        .map(|(name, durations)| {
            let sum_ns: u128 = durations.iter().map(|d| d.as_nanos()).sum();
            let avg_ns = sum_ns as f64 / runs as f64;
            (*name, avg_ns)
        })
        .collect();

    averages.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("{:<25} {:>12}", "Function", "Avg time (µs)");
    println!("{}", "-".repeat(40));
    for (name, avg_ns) in &averages {
        let avg_us = avg_ns / 1_000.0;
        println!("{:<25} {:>10.2}", name, avg_us);
    }

    let total_ns: f64 = averages.iter().map(|(_, t)| t).sum();
    println!("\nShare of total time:");
    for (name, avg_ns) in &averages {
        let share = avg_ns / total_ns * 100.0;
        println!("  {:<25} {:>5.1}%", name, share);
    }
}
