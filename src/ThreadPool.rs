struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    pub fn new(thread_count: usize) -> Self {
        Self {
            workers: (1..thread_count).map(|x| Worker::new(x)).collect(),
        }
    }
}

struct Worker {
    id: usize,
}

impl Worker {
    pub fn new(id: usize) -> Self {
        Self { id }
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
    fn worker_constructs() {
        Worker::new(0);
    }
}
