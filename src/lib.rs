use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::Arc;
use std::sync::Mutex;

struct Worker {
    thread: thread::JoinHandle<()>,
    id: usize,
    receiver: Arc<Mutex<Receiver<Job>>>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {});
        Worker {
            id,
            thread,
            receiver
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>
}

pub struct Job {

}

/**
The ThreadPool will create a channel and hold on to the sending side of the channel.
Each Worker will hold on to the receiving side of the channel.
We’ll create a new Job struct that will hold the closures we want to send down the channel.
The execute method will send the job it wants to execute down the sending side of the channel.
In its thread, the Worker will loop over its receiving side of the channel and execute the closures of any jobs it receives.
 */

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (tx, rx) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(rx));
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender: tx
        }
    }

    pub fn execute<F>(&self, f: F)
        where 
            F: FnOnce() + Send + 'static
    {

    }
}