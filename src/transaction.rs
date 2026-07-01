pub type AccountId = u32;
pub type TransactionId = u64;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: TransactionId,
    pub reads: Vec<AccountId>,
    pub writes: Vec<AccountId>,
    /// Simulated execution cost — used to spin/sleep during execute
    pub work_units: u32,
}

impl Transaction {
    pub fn new(id: u64, reads: Vec<AccountId>, writes: Vec<AccountId>, work_units: u32) -> Self {
        Self {
            id,
            reads,
            writes,
            work_units,
        }
    }
}
