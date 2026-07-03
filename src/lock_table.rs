use std::collections::HashMap;

use crate::transaction::{AccountId, Transaction};

#[derive(Default)]
pub struct LockTable {
    read_locks: HashMap<AccountId, usize>,
    write_locks: HashMap<AccountId, usize>,
}

impl LockTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn can_lock(&self, tx: &Transaction) -> bool {
        for account in &tx.reads {
            if self.write_locks.get(account).copied().unwrap_or(0) > 0 {
                return false;
            }
        }

        for account in &tx.writes {
            if self.read_locks.get(account).copied().unwrap_or(0) > 0 {
                return false;
            }

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
            if let Some(count) = self.read_locks.get_mut(account) {
                *count -= 1;

                if *count == 0 {
                    self.read_locks.remove(account);
                }
            }
        }
        for account in &tx.writes {
            if let Some(count) = self.write_locks.get_mut(account) {
                *count -= 1;

                if *count == 0 {
                    self.write_locks.remove(account);
                }
            }
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
        let mut table = LockTable::new();
        let writer = tx(0, vec![], vec![1]);
        let reader = tx(1, vec![1], vec![]);
        table.lock(&writer);
        assert!(!table.can_lock(&reader));
        table.unlock(&writer);
        assert!(table.can_lock(&reader));
    }

    #[test]
    fn two_readers_ok() {
        let mut table = LockTable::new();
        let a = tx(0, vec![1], vec![]);
        let b = tx(1, vec![1], vec![]);
        table.lock(&a);
        assert!(table.can_lock(&b));
    }

    #[test]
    fn write_blocks_write() {
        let mut table = LockTable::new();
        let a = tx(0, vec![], vec![1]);
        let b = tx(1, vec![], vec![1]);
        table.lock(&a);
        assert!(!table.can_lock(&b));
    }
}
