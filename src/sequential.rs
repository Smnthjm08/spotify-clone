use std::hint::black_box;

use crate::transaction::Transaction;

fn execute_transaction(tx: &Transaction) {
    let mut acc: u64 = 0;

    for i in 0..tx.work_units as u64 * 1000 {
        acc = black_box(acc.wrapping_add(i))
    }

    let _ = black_box(acc);
}
// pub fn execute_sequential(transactions: &[Transaction]) {
pub fn execute_sequential(transactions: Vec<Transaction>) {
    for tx in transactions {
        execute_transaction(&tx);
    }
}
