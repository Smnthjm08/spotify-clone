use std::collections::{HashMap, VecDeque};

use crate::{
    lock_table::LockTable,
    transaction::{Transaction, TransactionId},
};

pub struct Scheduler {
    pending: VecDeque<Transaction>,
    waiting: VecDeque<Transaction>,
    running: HashMap<TransactionId, Transaction>,
    lock_table: LockTable,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            pending: VecDeque::new(),
            waiting: VecDeque::new(),
            running: HashMap::new(),
            lock_table: LockTable::new(),
        }
    }

    pub fn enqueue(&mut self, tx: Transaction) {
        self.pending.push_back(tx);
    }

    pub fn complete(&mut self, tx_id: TransactionId) {
        if let Some(tx) = self.running.remove(&tx_id) {
            self.lock_table.unlock(&tx);
        }
    }

    pub fn schedule(&mut self) -> Vec<Transaction> {
        // Drain all pending into a temp vec, try to run each
        let mut deferred = VecDeque::new();
        // let mut dispatched: Option<Transaction> = None;
        let mut runnable = Vec::new();

        // Try pending first, then waiting
        let mut candidates = std::mem::take(&mut self.pending);
        candidates.append(&mut std::mem::take(&mut self.waiting));

        for tx in candidates {
            // if dispatched.is_none() && self.lock_table.can_lock(&tx) {
            //     self.lock_table.lock(&tx);
            //     let id = tx.id;
            //     self.running.insert(id, tx.clone());
            //     dispatched = Some(tx);
            // } else {
            //     deferred.push_back(tx);
            // }
            if self.lock_table.can_lock(&tx) {
                self.lock_table.lock(&tx);
                self.running.insert(tx.id, tx.clone());
                runnable.push(tx);
            } else {
                deferred.push_back(tx);
            }
        }

        self.waiting = deferred;

        runnable
    }

    // pub fn drain_runnable(&mut self) -> Vec<Transaction> {
    //     let mut batch = Vec::new();
    //     while let Some(tx) = self.schedule() {
    //         batch.push(tx);
    //     }
    //     batch
    // }

    pub fn has_pending(&self) -> bool {
        !self.pending.is_empty() || !self.waiting.is_empty()
    }

    pub fn has_running(&self) -> bool {
        !self.running.is_empty()
    }

    pub fn is_done(&self) -> bool {
        !self.has_pending() && !self.has_running()
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}
