use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

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

        let workers = (0..size)
            .map(|x| Worker::new(x, Arc::clone(&receiver)))
            .collect();

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// Add a job to the queue.
    ///
    /// Once there's an idle thread, passed function will be executed
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    /// Drop ThreadPool.
    ///
    /// Drops `sender` which causes threads to break out of their loops;
    /// stops when all threads have been `join`ed
    ///
    /// # Panics
    ///
    /// The function panics on thread join errors
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
    /// Create a new Worker.
    ///
    /// Each worker aggregates a thread waiting to pop a message from the
    /// receiver and execute passed function or closure
    ///
    /// # Panics
    ///
    /// The 'new' function panics if acquiring mutex via `lock()` returns an error
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn thread_pool_constructs() {
        ThreadPool::new(4);
    }

    #[test]
    fn thread_pool_executes_closure() {
        ThreadPool::new(4).execute(|| {
            let a = 3;
            let _b = a + 4;
        });
    }

    #[test]
    fn thread_pool_executes_function() {
        fn example_fun() {
            let a = 4;
            let _b = a - 1;
        }
        ThreadPool::new(4).execute(&example_fun);
    }

    #[test]
    fn worker_constructs() {
        let (_, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));

        let _worker = Worker::new(0, Arc::clone(&receiver));
    }

    use std::sync::atomic::{AtomicI8, Ordering};
    #[test]
    fn worker_executes_jobs() {
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));

        let visiting_var = Arc::new(AtomicI8::new(0));
        let cloned_visiting = Arc::clone(&visiting_var);

        const TO_ADD: i8 = 100;
        let job = Box::new(move || {
            cloned_visiting.fetch_add(TO_ADD, Ordering::SeqCst);
        });

        let worker = Worker::new(0, Arc::clone(&receiver));
        sender.send(job).unwrap();

        drop(sender);
        worker.thread.unwrap().join().unwrap();

        assert_eq!(visiting_var.load(Ordering::SeqCst), TO_ADD);
    }
}
