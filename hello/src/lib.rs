use std::{
    thread,
    sync::{mpsc, Arc, Mutex}
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// create a new ThreadPool
    ///
    /// the size is the number of threads in the pool
    ///
    /// # Panics
    ///
    /// - The `new` function will panic if the size is zero.
    /// - The internal worker will panic if the OS cannot spawn new threads.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            let worker = Worker::new(id, Arc::clone(& receiver));
            workers.push(worker);
        }

        ThreadPool {workers, sender}
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker{
    thread: thread::JoinHandle<()>,
    id: usize,
}

impl Worker {
    fn new(id:usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let job_closure = move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {id} got a job; executing.");
            job();
        };
        let thread = thread::spawn(job_closure);
        Worker { id, thread }
    }
}

