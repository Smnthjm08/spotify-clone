# Mini Sealevel Runtime

## Design and Implementation Report

---

# Abstract

Sealevel is Solana's parallel transaction execution runtime that enables validators to process non-conflicting transactions simultaneously by leveraging explicit account access declarations.

This project implements a simplified version of Sealevel in Rust to demonstrate the core concepts behind account-based parallel execution. The implementation models transactions with explicit read and write account sets, detects conflicts through account locking, schedules non-conflicting transactions into runnable batches, and executes them concurrently using Rayon.

The project intentionally excludes production components such as BankingStage, AccountsDB, fee-priority scheduling, GPU execution, and dependency graph scheduling in order to focus on the scheduling algorithm itself.

---

# 1. Introduction

Most blockchain runtimes execute transactions sequentially.

While this guarantees correctness, it underutilizes modern multi-core processors because many transactions operate on completely independent state.

Consider two transactions:

```
Tx1:
Write Account A

Tx2:
Write Account B
```

Since the two transactions access different accounts, they do not interfere with each other and may safely execute concurrently.

Solana's Sealevel runtime exploits this observation by requiring every transaction to declare the accounts it intends to read and write before execution. The runtime then schedules non-conflicting transactions in parallel while preserving deterministic execution.

The objective of this project is to build a miniature version of this scheduling model.

---

# 2. Objectives

The goals of the project are:

- Understand Sealevel's account-based execution model.
- Implement explicit read/write account declarations.
- Detect transaction conflicts.
- Build a scheduler that dispatches only non-conflicting transactions.
- Execute runnable transactions in parallel.
- Compare parallel execution against a sequential baseline.
- Measure scheduler behavior under varying contention levels.

---

# 3. Background

## 3.1 Sequential Execution

Traditional blockchain runtimes process transactions one at a time.

```
Tx1

↓

Tx2

↓

Tx3
```

Although simple, this prevents independent transactions from utilizing multiple CPU cores.

---

## 3.2 Sealevel

Sealevel changes this model.

Every transaction specifies:

- accounts it reads
- accounts it writes

The scheduler analyzes these account lists before execution.

If two transactions do not conflict, they may execute simultaneously.

Example:

```
Tx1
Read A
Write B

Tx2
Read C
Write D

↓

No shared accounts

↓

Execute Together
```

---

# 4. System Design

The runtime consists of six major components.

```
                Workload Generator
                        │
                        ▼
                 Pending Queue
                        │
                        ▼
                   Scheduler
                        │
                  Lock Table Check
                ┌─────────┴─────────┐
                │                   │
             Runnable          Waiting Queue
                │                   ▲
                ▼                   │
        Parallel Executor           │
                │                   │
                ▼                   │
            Transaction Done────────┘
                │
                ▼
           Release Locks
```

Each component has a single responsibility.

---

# 5. Transaction Model

Transactions contain four fields:

```rust
pub struct Transaction {
    id: TransactionId,
    reads: Vec<AccountId>,
    writes: Vec<AccountId>,
    work_units: u32,
}
```

## Design Decisions

### Transaction ID

Provides unique identification for scheduling and completion.

### Read Set

Accounts that may only be read.

Multiple transactions may read the same account simultaneously.

### Write Set

Accounts whose state may change.

Writes require exclusive access.

### Work Units

Represents simulated execution cost.

Instead of executing real smart contracts, the runtime performs CPU work proportional to this value.

---

# 6. Conflict Detection

Two transactions conflict when one transaction writes an account accessed by another.

Conflict rules:

| Access Pattern | Conflict |
| -------------- | -------- |
| Read – Read    | No       |
| Read – Write   | Yes      |
| Write – Read   | Yes      |
| Write – Write  | Yes      |

The project includes a standalone conflict detection module used to model these rules and verify correctness through unit tests.

---

# 7. Account Lock Table

The lock table models Sealevel's account locking semantics.

Two lock maps are maintained:

```
Read Locks

Write Locks
```

Scheduling rules:

- Read locks block writers.
- Write locks block readers.
- Write locks block other writers.
- Read locks do not block other readers.

After transaction completion, all locks are immediately released.

This ensures conflicting transactions never execute concurrently.

---

# 8. Scheduler

The scheduler maintains three collections.

```
Pending

Waiting

Running
```

Scheduling proceeds in rounds.

For every transaction:

1. Check whether all required locks are available.
2. If available:
   - acquire locks
   - move transaction to Running
3. Otherwise:
   - move transaction to Waiting

After execution completes:

- locks are released
- waiting transactions are reconsidered

The scheduler continues until all queues become empty.

Scheduling policy:

- FIFO
- Batch dispatch
- Lock-based conflict resolution

---

# 9. Parallel Execution

Runnable transactions are executed using Rayon.

```
Runnable Batch

↓

Rayon Parallel Iterator

↓

Execute Simultaneously
```

Execution is simulated using CPU work rather than smart contract execution.

The objective is to measure scheduler behavior rather than VM performance.

---

# 10. Synthetic Workload Generation

A workload generator produces configurable transaction streams.

Parameters:

- transaction count
- account count
- contention factor
- execution cost
- random seed

Transactions are generated such that contention increases as more transactions access a small set of "hot" accounts.

Benchmark scenarios:

| Scenario      | Contention |
| ------------- | ---------- |
| No Contention | 0%         |
| Low           | 25%        |
| Medium        | 50%        |
| High          | 75%        |
| Very High     | 90%        |

---

# 11. Scheduler Metrics

Execution time alone does not explain scheduler behavior.

The runtime therefore records additional metrics.

Collected metrics include:

- Scheduling rounds
- Transactions scheduled
- Deferred transactions
- Maximum batch size
- Average batch size
- Maximum waiting queue

These metrics explain how contention influences scheduling efficiency.

---

# 12. Benchmarking

Two execution models are compared.

### Sequential

Transactions execute one after another.

### Parallel

Non-conflicting transactions execute simultaneously.

Criterion benchmarks provide statistically rigorous execution measurements.

---

# 13. Results

Expected behavior:

Low contention:

- Large runnable batches
- Few deferred transactions
- High CPU utilization
- Significant speedup

High contention:

- Smaller runnable batches
- Larger waiting queues
- More scheduling rounds
- Speedup approaches sequential execution

This behavior matches Sealevel's design philosophy: available parallelism depends entirely on account conflicts.

---

# 14. Comparison with Solana

| Feature                  | Mini Runtime | Solana |
| ------------------------ | ------------ | ------ |
| Read/Write Account Lists | ✓            | ✓      |
| Account Locking          | ✓            | ✓      |
| Parallel Scheduling      | ✓            | ✓      |
| Batch Dispatch           | ✓            | ✓      |
| FIFO Scheduler           | ✓            | ✗      |
| Fee Priority             | ✗            | ✓      |
| Dependency Graph         | ✗            | ✓      |
| BankingStage             | ✗            | ✓      |
| AccountsDB               | ✗            | ✓      |
| GPU Execution            | ✗            | ✓      |

The implementation focuses exclusively on the scheduling algorithm while omitting production infrastructure.

---

# 15. Limitations

The project intentionally simplifies many aspects of Solana.

Not implemented:

- BankingStage
- AccountsDB
- Fee-priority scheduling
- Dependency graph scheduler
- Continuous scheduling
- GPU execution
- Compute-unit pricing
- Address Lookup Tables
- Banking pipeline

These simplifications keep the implementation compact while preserving the core scheduling concepts.

---

# 16. Future Work

Possible extensions include:

- Fee-priority scheduling using BinaryHeap
- Dependency graph scheduling
- Continuous scheduler pipeline
- Zipf-distributed workloads
- Compute-budget simulation
- Work-stealing executor
- More realistic smart contract execution
- Interactive scheduler visualization

---

# 17. Lessons Learned

This project demonstrates that efficient parallel execution depends less on the execution engine itself and more on correctly identifying independent work.

By explicitly declaring account access before execution, the scheduler can safely execute non-conflicting transactions concurrently while preserving correctness.

Although significantly smaller than Solana's production runtime, this implementation captures the central idea behind Sealevel: **parallelism emerges from account independence rather than speculative execution.**

---

# Conclusion

Mini Sealevel Runtime successfully demonstrates the core principles behind Solana's parallel transaction scheduler.

The implementation models explicit account access, detects conflicts through account locking, schedules runnable transactions, executes them concurrently, and evaluates performance under varying contention levels.

While simplified compared to the production validator, the project provides a practical understanding of how account-aware scheduling enables scalable parallel execution in modern blockchain runtimes.
