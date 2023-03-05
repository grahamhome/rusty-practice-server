use std::{
    sync::{mpsc, Arc, Mutex},
    thread
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // Get an exclusive lock on the receiver and block until a message is received.
                // Release the lock as soon as "job" is assigned a value.
                let job = receiver.lock().unwrap().recv().unwrap();
                job();
            }
        });
        Worker {
            id,
            thread,
        }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool.
    /// The size is the number of threads in the pool.
    /// # Panics
    /// Panics if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {workers, sender}
    }

    pub fn execute<F>(&self, function:F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(function);
        self.sender.send(job).unwrap()
    }
}