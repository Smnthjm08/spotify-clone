use std::collections::{HashMap, VecDeque};

use crate::{
    lock_table::LockTable,
    transaction::{Transaction, TransactionId},
};

#[derive(Debug, Default)]
pub struct SchedulerMetrics {
    pub scheduling_rounds: usize,
    pub transactions_scheduled: usize,
    pub total_deferrals: usize,
    pub max_waiting_queue: usize,
    pub max_batch_size: usize,
    pub total_batch_size: usize,
}

impl SchedulerMetrics {
    pub fn average_batch_size(&self) -> f64 {
        if self.scheduling_rounds == 0 {
            0.0
        } else {
            self.total_batch_size as f64 / self.scheduling_rounds as f64
        }
    }
}

pub struct Scheduler {
    pending: VecDeque<Transaction>,
    waiting: VecDeque<Transaction>,
    running: HashMap<TransactionId, Transaction>,
    lock_table: LockTable,
    pub metrics: SchedulerMetrics,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            pending: VecDeque::new(),
            waiting: VecDeque::new(),
            running: HashMap::new(),
            lock_table: LockTable::new(),
            metrics: SchedulerMetrics::default(),
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
                self.metrics.total_deferrals += 1;
                deferred.push_back(tx);
            }
        }

        self.waiting = deferred;
        self.metrics.max_waiting_queue = self.metrics.max_waiting_queue.max(self.waiting.len());

        self.metrics.scheduling_rounds += 1;
        self.metrics.transactions_scheduled += runnable.len();
        self.metrics.max_batch_size = self.metrics.max_batch_size.max(runnable.len());
        self.metrics.total_batch_size += runnable.len();

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
