use std::{
    sync::{mpsc::{self, Receiver}, Arc, Mutex},
    thread
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

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

        let (sender, receiver) = mpsc::channel();
        
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // create threads
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(self: &Self, f: F)
    where
        // we need Send to transfer the closure from one thread to another
        // and 'static because we don’t know how long the thread will take to execute. 
        F: FnOnce() + Send + 'static, 
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }


    // EXERCISE: Do the same as new but with proper error handling:
    // pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {}
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {   
                let job = receiver.lock().unwrap().recv().unwrap();
                // Note: lock is "unlocked" here, since the lock's mutexGuard is dropped after the let statement.
                println!("Worker {id} got a job; executing.");
                job();
            }
        });
        Worker { id, thread }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;