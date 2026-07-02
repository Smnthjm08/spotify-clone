use std::time::Instant;

use crate::{
    executor::execute_parallel,
    metrics::{RunResult, print_comparison},
    scheduler::Scheduler,
    sequential::execute_sequential,
    workload::generate_workload,
};

mod conflict;
mod executor;
mod lock_table;
mod metrics;
mod scheduler;
mod sequential;
mod transaction;
mod workload;

const TRANSACTION_COUNT: usize = 1000;
const ACCOUNT_COUNT: u32 = 200;
const WORKER_UNITS: u32 = 10;

fn run_scenario(label: &str, contention: f64) {
    println!("\n── {} (contention: {:.0}%) ──", label, contention * 100.0);

    let workload = generate_workload(
        TRANSACTION_COUNT,
        ACCOUNT_COUNT,
        contention,
        WORKER_UNITS,
        42,
    );

    // Sequential baseline
    let seq_workload = workload.clone();

    let t = Instant::now();

    execute_sequential(seq_workload);
    let seq_result = RunResult {
        label: "Sequential",
        transaction_count: TRANSACTION_COUNT,
        duration: t.elapsed(),
    };

    // Parallel scheduler
    let mut sched = Scheduler::new();

    for tx in workload {
        sched.enqueue(tx);
    }
    let t = Instant::now();
    execute_parallel(&mut sched);

    let par_result = RunResult {
        label: "Parallel",
        transaction_count: TRANSACTION_COUNT,
        duration: t.elapsed(),
    };

    print_comparison(&seq_result, &par_result);
}

fn main() {
    println!("Mini Sealevel Runtime");

    println!(
        "Transactions: {} Accounts: {} Workers: {}",
        TRANSACTION_COUNT, ACCOUNT_COUNT, WORKER_UNITS
    );

    run_scenario("Low contention", 0.10);
    run_scenario("Medium contention", 0.40);
    run_scenario("High contention", 0.80);

    println!();
}
