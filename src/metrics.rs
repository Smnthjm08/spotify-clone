use std::time::Duration;

#[derive(Debug)]
pub struct RunResult {
    pub label: &'static str,
    pub transaction_count: usize,
    pub duration: Duration,
}

impl RunResult {
    pub fn throughput(&self) -> f64 {
        self.transaction_count as f64 / self.duration.as_secs_f64()
    }
}

pub fn print_comparison(sequential: &RunResult, parallel: &RunResult) {
    let speedup = sequential.duration.as_secs_f64() / parallel.duration.as_secs_f64();

    println!(
        "  {:<12} {:>8.1} ms  ({:.0} tx/s)",
        sequential.label,
        sequential.duration.as_secs_f64() * 1000.0,
        sequential.throughput()
    );
    println!(
        "  {:<12} {:>8.1} ms  ({:.0} tx/s)",
        parallel.label,
        parallel.duration.as_secs_f64() * 1000.0,
        parallel.throughput()
    );
    println!("  Speedup:     {:>8.2}x", speedup);
}
