/* ====================================================================================================
  OPTIMISTIC LOCKING WITH COMPARE-AND-SWAP (CAS) - SUCCESS & FAILURE DEMONSTRATION
====================================================================================================

OVERVIEW:
  Demonstrates optimistic locking with REAL conflicts between threads. Multiple threads compete
  to update the same shared value, causing compare-and-swap failures and retries. This shows
  both successful updates and conflict detection in action.

WHAT IS OPTIMISTIC LOCKING:
  A concurrency control strategy based on the assumption that conflicts between threads are
  infrequent. Rather than acquiring exclusive locks upfront (pessimistic locking), threads
  work optimistically and only check for conflicts at commit time.

PROGRAM FLOW:
  1. Create shared counter starting at 0
  2. Spawn 5 threads that each try to increment the counter 10 times
  3. Each thread:
     - Reads current value (snapshot)
     - Calculates new value (current + 1)
     - Attempts compare-and-swap
     - On success: moves to next increment
     - On failure: retries with updated value
  4. Display final result and statistics

CORE OPERATIONS:
  • load(Ordering): Takes a snapshot of current value from shared memory
  
  • compare_exchange(expected, new, success_order, failure_order):
    Atomically performs: "If value == expected, set to new; else return actual value"
    Returns: Ok(old_value) on success, Err(actual_value) on failure
    This is a SINGLE ATOMIC CPU instruction - cannot be interrupted mid-operation

  • The Retry Loop: Continuously attempts operation until successful
    Each retry uses the most recent value, ensuring eventual success

OPTIMISTIC VS PESSIMISTIC LOCKING:

  Pessimistic (Mutex):
    1. Acquire lock (block other threads)
    2. Read data
    3. Modify data
    4. Release lock
    ✗ Other threads wait even if no actual conflict occurs
    ✓ Guaranteed no conflicts

  Optimistic (CAS):
    1. Read data (no lock)
    2. Modify locally (no lock)
    3. Atomic commit with conflict detection
    4. Retry if conflict detected
    ✓ Multiple threads can read simultaneously
    ✓ No blocking unless actual conflict occurs
    ✗ Must handle retry logic

WHEN TO USE OPTIMISTIC LOCKING:
  ✓ Conflicts are rare (low contention scenarios)
  ✓ Operations are fast (quick calculations)
  ✓ Lock-free progress is important
  ✗ Don't use when conflicts are frequent (wasted retries)
  ✗ Don't use for complex multi-step operations

COMPARE-AND-SWAP GUARANTEES:
  • Atomicity: The check-and-update happens as one indivisible operation
  • Linearizability: Operations appear to occur in some sequential order
  • Lock-Freedom: At least one thread always makes progress system-wide
  • No Deadlock: No locks means no possibility of circular waiting

REAL-WORLD EXAMPLES:
  • Database transactions (snapshot isolation)
  • Version control systems (Git merge conflicts)
  • Concurrent data structures (lock-free stacks, queues)
  • Memory allocators (jemalloc uses CAS extensively)
  • High-frequency trading systems (low latency critical)

MEMORY ORDERING:
  Ordering::SeqCst (Sequential Consistency):
  • Strongest ordering guarantee
  • All threads see all operations in the same global order
  • Used here for simplicity and correctness
  • Weaker orderings (Acquire/Release) possible for performance optimization

==================================================================================================== */

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));
    let num_threads = 5;
    let increments_per_thread = 10;
    
    println!("Starting {} threads, each will increment counter {} times", num_threads, increments_per_thread);
    println!("Expected final value: {}\n", num_threads * increments_per_thread);

    let mut handles = vec![];

    for thread_id in 0..num_threads {
        let counter_clone = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            let mut success_count = 0;
            let mut retry_count = 0;

            for i in 0..increments_per_thread {
                let mut current = counter_clone.load(Ordering::SeqCst);
                
                loop {
                    let new_value = current + 1;
                    
                    thread::sleep(Duration::from_micros(10));
                    
                    let result = counter_clone.compare_exchange(
                        current,
                        new_value,
                        Ordering::SeqCst,
                        Ordering::SeqCst
                    );
                    
                    match result {
                        Ok(_) => {
                            println!("  [Thread {}] ✓ SUCCESS: Increment {} - Changed {} to {}", 
                                thread_id, i + 1, current, new_value);
                            success_count += 1;
                            break;
                        }
                        Err(actual_value) => {
                            println!("  [Thread {}] ✗ CONFLICT: Expected {}, but found {} - RETRYING...", 
                                thread_id, current, actual_value);
                            retry_count += 1;
                            current = actual_value;
                        }
                    }
                }
            }
            
            println!("\n[Thread {}] FINISHED - Successes: {}, Retries: {}", 
                thread_id, success_count, retry_count);
        });
        
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_value = counter.load(Ordering::SeqCst);
    println!("\n===========================================");
    println!("FINAL COUNTER VALUE: {}", final_value);
    println!("EXPECTED VALUE: {}", num_threads * increments_per_thread);
    println!("RESULT: {}", if final_value == num_threads * increments_per_thread { "✓ CORRECT" } else { "✗ ERROR" });
}