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
const WORK_UNITS: u32 = 10;

const CONTENTION_LEVELS: [(&str, f64); 5] = [
    ("No Contention", 0.0),
    ("Low Contention", 0.25),
    ("Medium Contention", 0.50),
    ("High Contention", 0.75),
    ("Very High Contention", 0.90),
];

fn run_scenario(label: &str, contention: f64) {
    println!();
    println!("-------------------------------------------------");
    println!("{label}");
    println!("Contention : {:.0}%", contention * 100.0);
    println!("-------------------------------------------------");

    let workload = generate_workload(TRANSACTION_COUNT, ACCOUNT_COUNT, contention, WORK_UNITS, 42);

    // Sequential baseline
    let seq_workload = workload.clone();

    let start = Instant::now();
    execute_sequential(&seq_workload);

    let seq_result = RunResult {
        label: "Sequential",
        transaction_count: TRANSACTION_COUNT,
        duration: start.elapsed(),
    };

    // Parallel scheduler
    let mut scheduler = Scheduler::new();

    for tx in workload {
        scheduler.enqueue(tx);
    }

    let start = Instant::now();
    execute_parallel(&mut scheduler);

    let par_result = RunResult {
        label: "Parallel",
        transaction_count: TRANSACTION_COUNT,
        duration: start.elapsed(),
    };

    print_comparison(&seq_result, &par_result);
}

fn main() {
    println!("=================================================");
    println!("        Mini Sealevel Runtime Benchmark");
    println!("=================================================");
    println!("Transactions : {}", TRANSACTION_COUNT);
    println!("Accounts     : {}", ACCOUNT_COUNT);
    println!("Work Units   : {}", WORK_UNITS);
    println!("Rayon Threads: {}", rayon::current_num_threads());
    println!("=================================================");

    for (label, contention) in CONTENTION_LEVELS {
        run_scenario(label, contention);
    }

    println!();
}
