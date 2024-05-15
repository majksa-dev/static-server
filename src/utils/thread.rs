use essentials::{debug, error, info};
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

#[derive(Debug)]
pub struct ZeroThreads;

#[derive(Debug)]
pub struct ChannelClosed;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            debug!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                match thread.join() {
                    Ok(_) => debug!("Worker {} shut down", worker.id),
                    Err(e) => error!("Worker {} panicked: {:?}", worker.id, e),
                }
            }
        }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, ZeroThreads> {
        if size == 0 {
            Err(ZeroThreads)?
        }
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        Ok(ThreadPool {
            workers,
            sender: Some(sender),
        })
    }

    pub fn execute<F>(&self, f: F) -> Result<(), ChannelClosed>
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender
            .as_ref()
            .and_then(|sender| sender.send(job).ok())
            .ok_or(ChannelClosed)
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            info!("Worker {id} connected.");
            loop {
                debug!("Worker {id} waiting for job.");
                let message = match receiver.lock() {
                    Ok(receiver) => receiver.recv(),
                    Err(err) => {
                        error!("Worker {id} error: {err}");
                        continue;
                    }
                };
                match message {
                    Ok(job) => {
                        debug!("Worker {id} received a job; executing.");
                        job();
                    }
                    Err(_) => {
                        info!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                };
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
