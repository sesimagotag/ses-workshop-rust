use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    _workers: Vec<Worker>,
    tx: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(worker_count: usize) -> Self {
        let (tx, rx) = mpsc::channel();

        let rx = Arc::new(Mutex::new(rx));

        let mut workers = Vec::with_capacity(worker_count);

        for n in 0..worker_count {
            workers.push(Worker::new(n, Arc::clone(&rx)))
        }

        ThreadPool {
            _workers: workers,
            tx,
        }
    }

    pub fn execute<F>(&self, callback: F)
        where
            F: FnOnce() + Send + 'static,
    {
        self.tx
            .send(Box::new(callback))
            .expect("Thread shut down too early");
    }
}

struct Worker {
    _id: usize,
    _handle: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let handle = thread::spawn(move || loop {
            let result = rx.lock().unwrap().recv();
            match result {
                Ok(rx) => {
                    println!("Worker {} got a job; executing.", id);
                    rx()
                }
                Err(_) => {
                    println!("Worker {} signing off", id);
                    break;
                }
            }
        });
        Worker {
            _id: id,
            _handle: handle,
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_threadpool() {
        let count = AtomicUsize::new(0);
        let worker_count = 4;
        let job_count = worker_count * 20;
        let pool = ThreadPool::new(worker_count);

        /// Task 2:
        ///     use the thread pool to execute [job_count] increments
        ///     on the variable [count].
        ///     beware the compiler outputs and hints.

        let count = count.load(Ordering::SeqCst);
        assert_eq!(count, job_count);
    }
}
