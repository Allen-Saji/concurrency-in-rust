/* ====================================================================================================
  MULTI-THREADED SAFE QUEUE (Producer-Consumer Pattern)
====================================================================================================

OVERVIEW:
  A demonstration of building a thread-safe concurrent queue in Rust without external crates.
  The program simulates a producer-consumer scenario where multiple threads produce data and
  multiple threads consume it, all accessing a shared queue safely.

PROGRAM FLOW:
  1. Initialize a SharedQueue wrapped in Arc for multi-threaded ownership
  2. Spawn 4 consumer threads that continuously dequeue and process items
  3. Spawn 4 producer threads that each enqueue 250,000 integers (1M total)
  4. Wait for all producers to complete their work
  5. Send shutdown signal to notify consumers no more data is coming
  6. Wait for consumers to drain remaining items and exit
  7. Display final statistics (time taken, items processed)

KEY STRUCTURES:
  • State<T>: Internal structure holding the VecDeque and shutdown flag
  • SharedQueue<T>: Thread-safe wrapper using Mutex and Condvar

CORE FUNCTIONS:
  • new(): Creates an empty queue with shutdown=false
  • enqueue(item): Adds item to queue back, notifies one waiting consumer
  • dequeue(): Removes item from queue front; blocks if empty until data arrives or shutdown
  • send_shutdown(): Sets shutdown flag and wakes all sleeping consumers
  • size(): Returns current queue length

CONCURRENCY MECHANISMS:
  • Mutex<State<T>>: Ensures exclusive access to queue and shutdown flag
  • Condvar: Allows threads to sleep efficiently when waiting for data
  • Arc<SharedQueue<T>>: Enables safe shared ownership across threads

GRACEFUL SHUTDOWN:
  Producers finish → send_shutdown() called → Consumers drain queue → All threads exit cleanly.
  The dequeue() function returns None when both the queue is empty AND shutdown is signaled,
  allowing consumers to distinguish between "temporarily empty" and "permanently done".

==================================================================================================== */

use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Instant;

struct State<T> {
    queue: VecDeque<T>,
    shutdown: bool,
}

struct SharedQueue<T> {
    state: Mutex<State<T>>,
    condvar: Condvar,
}

impl<T> SharedQueue<T> {
    fn new() -> Self {
        SharedQueue {
            state: Mutex::new(State {
                queue: VecDeque::new(),
                shutdown: false,
            }),
            condvar: Condvar::new(),
        }
    }

    fn enqueue(&self, item: T) {
        let mut state = self.state.lock().unwrap();
        
        if state.shutdown {
            panic!("Cannot enqueue items after shutdown signal!");
        }

        state.queue.push_back(item);
        self.condvar.notify_one();
    }

    fn dequeue(&self) -> Option<T> {
        let mut state = self.state.lock().unwrap();

        loop {
            if let Some(item) = state.queue.pop_front() {
                return Some(item);
            }

            if state.shutdown {
                return None;
            }

            state = self.condvar.wait(state).unwrap();
        }
    }

    fn send_shutdown(&self) {
        let mut state = self.state.lock().unwrap();
        state.shutdown = true;
        self.condvar.notify_all();
    }

    fn size(&self) -> usize {
        let state = self.state.lock().unwrap();
        state.queue.len()
    }
}

fn main() {
    const TOTAL_ITEMS: usize = 1_000_000;
    const NUM_PRODUCERS: usize = 4;
    const NUM_CONSUMERS: usize = 4;

    let items_per_producer = TOTAL_ITEMS / NUM_PRODUCERS;
    let shared_queue = Arc::new(SharedQueue::<i32>::new());
    let start_time = Instant::now();
    let mut consumer_handles = vec![];
    let mut producer_handles = vec![];

    println!("--- Starting Simulation ---");
    println!("Producers: {}, Consumers: {}, Total Items: {}", NUM_PRODUCERS, NUM_CONSUMERS, TOTAL_ITEMS);

    for id in 0..NUM_CONSUMERS {
        let q = Arc::clone(&shared_queue);
        let handle = thread::spawn(move || {
            let mut count = 0;
            while let Some(_) = q.dequeue() {
                count += 1;
            }
            println!("Consumer {} finished. Processed {} items.", id, count);
        });
        consumer_handles.push(handle);
    }

    for id in 0..NUM_PRODUCERS {
        let q = Arc::clone(&shared_queue);
        let handle = thread::spawn(move || {
            for j in 0..items_per_producer {
                let val = (id * items_per_producer + j) as i32;
                q.enqueue(val);
            }
        });
        producer_handles.push(handle);
    }

    for h in producer_handles {
        h.join().unwrap();
    }
    println!("All Producers finished writing.");

    shared_queue.send_shutdown();

    for h in consumer_handles {
        h.join().unwrap();
    }

    let duration = start_time.elapsed();
    println!("--- All operations complete ---");
    println!("Final Queue Size: {} (Should be 0)", shared_queue.size());
    println!("Time taken: {:.2?}", duration);
}