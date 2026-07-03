use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use mini_sealevel::{
    executor::execute_parallel, scheduler::Scheduler, sequential::execute_sequential,
    workload::generate_workload,
};

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

fn bench_scheduler(c: &mut Criterion) {
    let mut group = c.benchmark_group("sealevel_scheduler");

    for &(label, contention) in &CONTENTION_LEVELS {

        group.bench_with_input(
            BenchmarkId::new("sequential", &label),
            &contention,
            |b, &c| {
                b.iter(|| {
                    let workload =
                        generate_workload(TRANSACTION_COUNT, ACCOUNT_COUNT, c, WORK_UNITS, 42);
                    execute_sequential(&workload);
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("parallel", &label),
            &contention,
            |b, &c| {
                b.iter(|| {
                    let workload =
                        generate_workload(TRANSACTION_COUNT, ACCOUNT_COUNT, c, WORK_UNITS, 42);
                    let mut sched = Scheduler::new();
                    for tx in workload {
                        sched.enqueue(tx);
                    }
                    execute_parallel(&mut sched);
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_scheduler);
criterion_main!(benches);
