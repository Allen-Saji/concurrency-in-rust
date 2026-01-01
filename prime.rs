use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

fn is_prime(n: usize) -> bool {
    if n <= 1 { return false; }
    if n <= 3 { return true; }
    if n % 2 == 0 || n % 3 == 0 { return false; }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 { return false; }
        i += 6;
    }
    true
}

fn main() {
    let limit = 100_000_000;
    let num_threads = 10;
    
    // Arc (Atomic Reference Counter) allows multiple threads to own the same data.
    // AtomicUsize allows threads to safely update a shared counter without "locking."
    let counter = Arc::new(AtomicUsize::new(2)); // Start at 2
    let total_primes = Arc::new(AtomicUsize::new(0));
    
    let mut handles = vec![];
    let start_total = Instant::now();

    for t in 0..num_threads {
        let counter_ref = Arc::clone(&counter);
        let total_ref = Arc::clone(&total_primes);

        let handle = thread::spawn(move || {
            let thread_start = Instant::now();
            let mut local_count = 0;

            loop {
                // Fetch the next number and increment the global counter atomically
                let num = counter_ref.fetch_add(1, Ordering::SeqCst);
                
                if num > limit { break; } 

                if is_prime(num) {
                    local_count += 1;
                }
            }

            // Add this thread's findings to the global total
            total_ref.fetch_add(local_count, Ordering::Relaxed);
            
            println!("Thread {:2}: Finished in {:?}.", t, thread_start.elapsed());
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("--------------------------------------");
    println!("Total Primes Found: {}", total_primes.load(Ordering::SeqCst));
    println!("Total Execution Time: {:?}", start_total.elapsed());
}