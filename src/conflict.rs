use std::collections::HashSet;

use crate::transaction::Transaction;

/// Returns true if two transactions cannot execute in parallel.
pub fn conflict(a: &Transaction, b: &Transaction) -> bool {
    let a_writes: HashSet<_> = a.writes.iter().collect();
    let b_writes: HashSet<_> = b.writes.iter().collect();

    let write_read_conflict = b.reads.iter().any(|r| a_writes.contains(r));
    let read_write_conflict = a.reads.iter().any(|r| b_writes.contains(r));

    let write_write_conflict = a_writes.iter().any(|w| b_writes.contains(w));

    write_read_conflict || read_write_conflict || write_write_conflict
}

/// Conflict rules:
/// - write/read
/// - read/write
/// - write/write
/// - read/read is allowed

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transaction::Transaction;

    fn tx(id: u64, reads: Vec<u32>, writes: Vec<u32>) -> Transaction {
        Transaction::new(id, reads, writes, 1)
    }

    #[test]
    fn read_read_is_not_conflict() {
        todo!()
    }

    #[test]
    fn write_read_is_conflict() {
        todo!()
    }

    #[test]
    fn read_write_is_conflict() {
        todo!()
    }

    #[test]
    fn write_write_is_conflict() {
        todo!()
    }

    #[test]
    fn disjoint_transactions_do_not_conflict() {
        todo!()
    }
}
