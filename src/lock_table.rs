use std::collections::HashMap;

use crate::transaction::{AccountId, Transaction};

#[derive(Default)]
pub struct LockTable {
    pub read_locks: HashMap<AccountId, usize>,
    pub write_locks: HashMap<AccountId, usize>,
}

impl LockTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn can_lock(&self, tx: &Transaction) -> bool {
        for account in &tx.writes {
            if self.read_locks.get(account).copied().unwrap_or(0) > 0 {
                return false;
            }
        }

        for account in &tx.reads {
            if self.write_locks.get(account).copied().unwrap_or(0) > 0 {
                return false;
            }
        }

        true
    }

    // only be called after can_lock()
    pub fn lock(&mut self, tx: &Transaction) {
        for account in &tx.reads {
            *self.read_locks.entry(*account).or_insert(0) += 1;
        }

        for account in &tx.writes {
            *self.write_locks.entry(*account).or_insert(0) += 1;
        }
    }

    pub fn unlock(&mut self, tx: &Transaction) {
        for account in &tx.reads {
            let count = self.read_locks.entry(*account).or_insert(0);
            *count = count.saturating_sub(1);
        }
        for account in &tx.writes {
            let count = self.write_locks.entry(*account).or_insert(0);
            *count = count.saturating_sub(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transaction::Transaction;

    fn tx(id: u64, reads: Vec<u32>, writes: Vec<u32>) -> Transaction {
        Transaction::new(id, reads, writes, 1)
    }

    #[test]
    fn write_blocks_read() {
        todo!()
    }

    #[test]
    fn two_readers_ok() {
        todo!()
    }

    #[test]
    fn write_blocks_write() {
        todo!()
    }
}
