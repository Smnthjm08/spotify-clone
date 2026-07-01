use crate::transaction::{AccountId, Transaction};
use rand::prelude::*;

// todo!()
// 1–3 reads
// 1 write
pub fn generate_workload(
    transaction_count: usize,
    total_accounts: u32,
    contention_factor: f64,
    work_units: u32,
    seed: u64,
) -> Vec<Transaction> {
    let mut rng = StdRng::seed_from_u64(seed);

    let hot_pool_size = ((total_accounts as f64 * 0.1) as u32).max(2);
    let hot_accounts: Vec<AccountId> = (0..hot_pool_size).collect();
    let all_accounts: Vec<AccountId> = (0..total_accounts).collect();

    (0..transaction_count)
        .map(|i| {
            let use_hot = rng.random_bool(contention_factor);

            let write_account = if use_hot {
                *hot_accounts.choose(&mut rng).unwrap()
            } else {
                *all_accounts.choose(&mut rng).unwrap()
            };

            // Each tx: one write account + one or two disjoint reads
            let read_account = loop {
                let candidate = *all_accounts.choose(&mut rng).unwrap();
                if candidate != write_account {
                    break candidate;
                }
            };

            Transaction::new(
                i as u64,
                vec![read_account],
                vec![write_account],
                work_units,
            )
        })
        .collect()
}
