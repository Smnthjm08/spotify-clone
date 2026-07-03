# Mini Sealevel Runtime

A simplified implementation of Solana's **Sealevel** parallel transaction execution model written in Rust.

This project demonstrates how transactions that access **disjoint account state** can execute concurrently while conflicting transactions are safely serialized using **read/write account locking**. It models the core scheduling ideas behind Sealevel without the complexity of Solana's production runtime.

---

## Overview

Traditional blockchain runtimes execute transactions sequentially, even when they operate on completely independent state. This leaves modern multi-core processors underutilized.

Solana's **Sealevel Runtime** takes a different approach. Every transaction declares the accounts it intends to **read** and **write** before execution. Using this information, the runtime identifies non-conflicting transactions and schedules them for parallel execution.

This project recreates the core scheduling concepts behind Sealevel by implementing:

- Explicit transaction read/write account declarations
- Read/write conflict detection
- Account lock management
- FIFO scheduler
- Parallel execution using Rayon
- Sequential baseline for comparison
- Synthetic workload generation
- Scheduler metrics
- Criterion benchmarks

The implementation intentionally omits production components such as BankingStage, AccountsDB, GPU execution, and fee-priority scheduling in order to focus on the scheduling algorithm itself.

---

# Motivation

Modern CPUs contain multiple cores, yet many blockchain runtimes still execute transactions one at a time.

If two transactions modify completely different accounts, there is no reason they cannot execute simultaneously.

The objective of this project is to demonstrate how account-aware scheduling enables safe parallel execution while preserving correctness.

---

# Project Structure

```
mini-sealevel/
│
├── benches/
│   └── scheduler_bench.rs
│
├── src/
│   ├── conflict.rs
│   ├── executor.rs
│   ├── lock_table.rs
│   ├── main.rs
│   ├── metrics.rs
│   ├── scheduler.rs
│   ├── sequential.rs
│   ├── transaction.rs
│   └── workload.rs
│
├── Cargo.toml
└── README.md
```

---

# Architecture

```
                   Workload Generator
                           │
                           ▼
                    Pending Queue
                           │
                           ▼
                      Scheduler
                           │
                 Account Lock Table
                   /             \
                  /               \
          Runnable Batch      Waiting Queue
                  │               ▲
                  ▼               │
          Rayon Parallel Executor │
                  │               │
                  ▼               │
             Transaction Complete
                  │
                  ▼
             Release Locks
                  │
                  └───────────────► Repeat
```

---

# Transaction Model

Each transaction explicitly declares:

- Accounts it reads
- Accounts it writes
- Simulated execution cost (`work_units`)

```rust
pub struct Transaction {
    pub id: TransactionId,
    pub reads: Vec<AccountId>,
    pub writes: Vec<AccountId>,
    pub work_units: u32,
}
```

This mirrors the design used by Solana's runtime, where transactions specify account access before execution.

---

# Conflict Detection

Two transactions conflict when:

- Write ↔ Read
- Read ↔ Write
- Write ↔ Write

Read ↔ Read is allowed.

Examples:

| Transaction A | Transaction B | Conflict |
| ------------- | ------------- | -------- |
| Read A        | Read A        | ❌ No    |
| Write A       | Read A        | ✅ Yes   |
| Read A        | Write A       | ✅ Yes   |
| Write A       | Write A       | ✅ Yes   |

---

# Lock Table

The lock table maintains two hash maps:

- Read Locks
- Write Locks

Rules:

- Multiple readers may access the same account simultaneously.
- Writers require exclusive access.
- Locks are released immediately after transaction completion.

This models Sealevel's account locking semantics in a simplified form.

---

# Scheduler

The scheduler maintains three queues:

- Pending
- Waiting
- Running

Scheduling proceeds as follows:

1. Collect pending transactions.
2. Check lock availability.
3. Dispatch runnable transactions.
4. Defer conflicting transactions.
5. Execute runnable batch in parallel.
6. Release locks.
7. Repeat until all transactions complete.

Scheduling policy:

- FIFO ordering
- Read/write lock validation
- Batch dispatch
- Parallel execution using Rayon

---

# Parallel Executor

Runnable transactions are executed using Rayon:

```rust
batch.par_iter().for_each(execute_transaction);
```

Execution is simulated using CPU work (`work_units`) to model transaction processing time.

---

# Workload Generator

Synthetic workloads are generated with configurable:

- Number of transactions
- Number of accounts
- Contention level
- Work units
- Random seed

Five contention scenarios are evaluated:

| Scenario      | Contention |
| ------------- | ---------- |
| No Contention | 0%         |
| Low           | 25%        |
| Medium        | 50%        |
| High          | 75%        |
| Very High     | 90%        |

As contention increases, more transactions compete for the same accounts, reducing available parallelism.

---

# Scheduler Metrics

The runtime reports several scheduling statistics:

- Scheduling rounds
- Transactions scheduled
- Deferred transactions
- Maximum batch size
- Average batch size
- Maximum waiting queue

These metrics provide insight into scheduler behavior beyond execution time.

Example:

```
Scheduler Metrics
-----------------
Rounds                : 28
Transactions Scheduled: 1000
Deferred              : 582
Maximum Batch Size    : 81
Average Batch Size    : 36.4
Maximum Waiting Queue : 247
```

---

# Benchmarking

The project compares:

- Sequential execution
- Parallel execution

using identical workloads.

Example output:

```
Low Contention

Sequential      50.78 ms
Parallel        18.70 ms

Speedup         2.71x
```

Criterion benchmarks are also included for statistically rigorous performance measurements.

Run:

```bash
cargo bench
```

---

# Running

Clone the repository:

```bash
git clone <repo-url>
cd mini-sealevel
```

Run the benchmark program:

```bash
cargo run --release
```

Run tests:

```bash
cargo test
```

Run Criterion benchmarks:

```bash
cargo bench
```

---

# Current Limitations

This project intentionally simplifies several aspects of Solana's production runtime.

Not implemented:

- BankingStage
- AccountsDB
- Fee-priority scheduling
- Dependency graph scheduler
- Continuous scheduling
- GPU/SIMD execution
- Compute budgets
- Address Lookup Tables

The objective is to focus on understanding Sealevel's scheduling algorithm rather than reproducing the entire validator pipeline.

---

# Future Work

Possible extensions include:

- Fee-priority scheduling using BinaryHeap
- Dependency graph scheduling
- Continuous scheduling pipeline
- Zipf-distributed workloads
- Compute-unit budgeting
- Work-stealing executor
- More realistic transaction models
- Performance visualization

---

# References

- Solana Sealevel Runtime Blog
- Solana Validator Architecture
- Rayon Parallel Iterator Documentation
- Criterion Benchmarking Framework
