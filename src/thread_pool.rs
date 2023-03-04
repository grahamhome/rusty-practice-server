pub struct ThreadPool;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool
    }

    pub fn execute<F>(&self, function:F)
        where
            F: FnOnce() + Send + 'static,
    {

    }
}