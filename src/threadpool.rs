use std::{
	sync::mpsc,
	sync::Arc,
	sync::Mutex,
	thread
};

pub struct Threadpool {
	workers: Vec<Worker>,
	sender: mpsc::Sender<Job>
}

impl Threadpool {
	/// create a new threadpool
	/// 
	/// size: number of threads in the threadpool
	/// 
	/// #panics
	/// The `new` function panics if size is zero
 	pub fn new(size: usize) -> Threadpool {
		assert!(size > 0);

		let (sender, receiver) = mpsc::channel();

		let receiver = Arc::new(Mutex::new(receiver));

		let mut workers = Vec::with_capacity(size);

		for id in 0..size {
			workers.push(Worker::new(id, Arc::clone(&receiver)));
		}

		Threadpool { workers, sender }
	}

	/// execute a job
	///
	/// f: a closure which will be held and executed by any of threads
	/// in the threadpool
	pub fn execute<F>(&self, f: F)
	where
	F: FnOnce() + Send + 'static
	{
		let job = Box::new(f);
		self.sender.send(job).unwrap();
	}
}

struct Worker {
	id: usize,
	handle: thread::JoinHandle<Arc<Mutex<mpsc::Receiver<Job>>>>
}

impl Worker {
	/// create a new worker
	///
	/// id: create a worker with this id
	fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
		let mut handle = thread::spawn(move || {
			loop {
				let job = receiver.lock().unwrap().recv().unwrap();
				job();
			}
		});
		Worker { id, handle }
	}
}

type Job = Box<dyn FnOnce() + Send + 'static>;
