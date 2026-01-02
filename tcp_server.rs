use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4); // Limit to 4 worker threads

    println!("Server running on 127.0.0.1:7878 with 4 worker threads");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection established!");

                pool.execute(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    match stream.read(&mut buffer) {
        Ok(_) => {
            println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

            let get_sleep = b"GET /sleep HTTP/1.1\r\n";

            let (status_line, content) = if buffer.starts_with(get_sleep) {
                thread::sleep(Duration::from_secs(5));
                ("HTTP/1.1 200 OK", "Slow Response\n")
            } else {
                ("HTTP/1.1 200 OK", "Fast Response\n")
            };

            let response = format!(
                "{}\r\nContent-Length: {}\r\n\r\n{}",
                status_line,
                content.len(),
                content
            );

            if let Err(e) = stream.write_all(response.as_bytes()) {
                eprintln!("Failed to write response: {}", e);
                return;
            }

            if let Err(e) = stream.flush() {
                eprintln!("Failed to flush stream: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to read from stream: {}", e);
        }
    }
}