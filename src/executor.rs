use crate::scheduler::{Scheduler, SchedulerMetrics};
use crate::transaction::Transaction;
use rayon::prelude::*;
use std::hint::black_box;

fn execute_transaction(tx: &Transaction) {
    let mut acc: u64 = 0;
    for i in 0..tx.work_units as u64 * 1000 {
        acc = black_box(acc.wrapping_add(i));
    }
    let _ = black_box(acc);
}

pub fn execute_parallel(scheduler: &mut Scheduler) -> SchedulerMetrics {
    while !scheduler.is_done() {
        let batch = scheduler.schedule();

        if batch.is_empty() {
            debug_assert!(scheduler.is_done(), "Scheduler reached an impossible state");
            break;
        }

        batch.par_iter().for_each(execute_transaction);

        for tx in batch {
            scheduler.complete(tx.id);
        }
    }

    std::mem::take(&mut scheduler.metrics)
}
