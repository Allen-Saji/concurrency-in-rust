# Rust Multithreading & Concurrency

A collection of practical programs exploring multithreading and concurrency patterns in Rust, demonstrating lock-free programming, thread pools, synchronization primitives, and real-world concurrent architectures.

## Programs Overview

### 1. **Producer-Consumer Queue** (`producer_consumer_queue.rs`)

**Concepts**: Mutex, Condvar, Arc, Graceful Shutdown

A thread-safe queue implementation using the producer-consumer pattern. Demonstrates:

- Thread synchronization with `Mutex` and `Condvar`
- Efficient thread coordination (threads sleep when idle instead of busy-waiting)
- Graceful shutdown mechanism for clean resource cleanup
- 4 producer threads generate 1M items, 4 consumer threads process them

**Key Learning**: Building thread-safe data structures and coordinating multiple threads efficiently.

---

### 2. **Parallel Prime Finder** (`prime_finder.rs`)

**Concepts**: Atomics, Lock-Free Programming, Work Stealing

Finds all prime numbers up to 100 million using 10 worker threads. Demonstrates:

- Lock-free synchronization with `AtomicUsize`
- Work-stealing pattern (threads compete for work atomically)
- `fetch_add()` for atomic counter operations
- Memory ordering (`SeqCst` vs `Relaxed`)

**Key Learning**: High-performance parallel computation without mutex overhead.

---

### 3. **Thread Pool Web Server** (`thread_pool_server.rs`)

**Concepts**: Thread Pool Pattern, mpsc Channel, TCP Networking

A production-ready HTTP server using a fixed-size thread pool. Demonstrates:

- Thread pool pattern to prevent resource exhaustion
- Message passing with `mpsc::channel`
- Shared receiver pattern (`Arc<Mutex<Receiver>>`)
- Graceful shutdown via `Drop` trait
- Handles `/sleep` endpoint (5-second delay) and fast responses

**Key Learning**: Building scalable servers with bounded concurrency.

---

### 4. **Optimistic Locking with CAS** (`optimistic_locking.rs`)

**Concepts**: Compare-And-Swap, Lock-Free Updates, Conflict Resolution

Demonstrates optimistic concurrency control with multiple threads competing to update a shared counter. Shows:

- `compare_exchange()` operation (atomic check-and-update)
- Conflict detection and retry logic
- Lock-free updates (no blocking)
- Success vs failure cases with real thread contention
- Statistics tracking (retries per thread)

**Key Learning**: Lock-free algorithms and handling concurrent updates without traditional locks.

---

## Core Concepts Covered

### Synchronization Primitives

- **Mutex**: Mutual exclusion locks for thread-safe access
- **Condvar**: Condition variables for efficient thread waiting
- **Atomics**: Lock-free operations via CPU instructions
- **Arc**: Atomic reference counting for shared ownership

### Concurrency Patterns

- **Producer-Consumer**: Queue-based work distribution
- **Thread Pool**: Fixed workers processing dynamic tasks
- **Work Stealing**: Threads compete for available work
- **Optimistic Locking**: Assume no conflicts, retry on collision

### Advanced Topics

- Memory ordering (`SeqCst`, `Relaxed`, `Acquire`, `Release`)
- Graceful shutdown mechanisms
- Lock-free vs lock-based approaches
- Message passing with channels
- Atomic operations (`fetch_add`, `compare_exchange`, `load`, `store`)

---

## Running the Examples

Since these are standalone Rust files without Cargo, compile and run them directly:

```bash
# Compile
rustc filename.rs

# Run
./filename
```

### Examples:

```bash
# Producer-Consumer Queue
rustc producer_consumer_queue.rs
./producer_consumer_queue

# Parallel Prime Finder
rustc prime_finder.rs
./prime_finder

# Thread Pool Server (then visit http://127.0.0.1:7878/sleep in browser)
rustc thread_pool_server.rs
./thread_pool_server

# Optimistic Locking
rustc optimistic_locking.rs
./optimistic_locking
```

On Windows:

```bash
rustc filename.rs
filename.exe
```

---

## Requirements

- Rust compiler (rustc) - Install from [rustup.rs](https://rustup.rs/)

---

## Real-World Applications

These patterns are used in:

- **Web Servers**: Nginx, Apache (thread pools)
- **Databases**: PostgreSQL, Redis (connection pools, atomic operations)
- **Game Engines**: Unity, Unreal (job systems, parallel rendering)
- **Data Processing**: Apache Spark, Hadoop (MapReduce-style parallelism)
- **Operating Systems**: Kernel schedulers, memory allocators
- **High-Frequency Trading**: Lock-free queues for microsecond latency

---
