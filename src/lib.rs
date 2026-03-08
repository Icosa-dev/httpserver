use std::{
    fs::{self, File},
    sync::{Arc, Mutex, mpsc},
    thread,
};

use anyhow::{Context, Result};
use serde::{Deserialize, ser::Error};

#[deprecated]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

#[deprecated]
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    #[deprecated]
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, reciever) = mpsc::channel();

        let reciever = Arc::new(Mutex::new(reciever));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&reciever)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    #[deprecated]
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
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

#[deprecated]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

#[deprecated]
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
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
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub ip: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn get_socket(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

pub fn get_server_config() -> Result<ServerConfig> {
    let contents = fs::read_to_string("config.toml")
        .context("Server config couldn't be read")?;
    let config: ServerConfig = toml::from_str(&contents)
        .context("Server config could not be deserialized")?;
    Ok(config)
}
