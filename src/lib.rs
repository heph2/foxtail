use std::{collections::VecDeque, time::SystemTime};
use std::sync::{Arc, Mutex};

pub struct Job {
    id: u32,
    status: JobStatus,
    payload: Vec<u8>,
    timestamp: SystemTime,
    heartbeat: SystemTime,
}

pub enum JobStatus {
    PENDING,
    PICKED,
    PROCESSED,
    FAILED,
}

pub struct InMemQueue {
    jobs: Arc<Mutex<VecDeque<Job>>>,
}

impl Job {
    pub fn new(id: u32, payload: &[u8]) -> Self {
        let now = SystemTime::now();
        Job {
            id,
            status: JobStatus::PENDING,
            payload: payload.to_vec(),
            timestamp: now,
            heartbeat: now,
        }
    }

    pub fn update_heartbeat(&mut self) {
        self.heartbeat = SystemTime::now();
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_status(&self) -> &JobStatus {
        &self.status
    }

    pub fn timestamp(&self) -> &SystemTime {
        &self.timestamp
    }

    pub fn heartbeat(&self) -> &SystemTime {
        &self.heartbeat
    }
}

pub trait JobQueue {
    fn new() -> Self;
    fn enqueue(&self, j: Job) -> Result<(), String>;
    fn get(&self, id_job: u32) -> Option<Job>;
    fn dequeue(&self, id_job: u32) -> Result<(), String>;
    fn len(&self) -> usize;
}

impl JobQueue for InMemQueue {
    fn new() -> Self {
        InMemQueue {
            jobs: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    fn enqueue(&self, j: Job) -> Result<(), String> {
        let mut queue = self.jobs.lock().map_err(|_| "failed to lock".to_string())?;
        queue.push_back(j);
        Ok(())
    }

    fn get(&self, id_job: u32) -> Option<Job> {
        let queue = self.jobs.lock().expect("Failed to lock the queue");
        if let Some(pos) = queue.iter().position(|job| job.id == id_job) {
            return queue.get(pos).cloned();
        }
        None
    }

    fn dequeue(&self, id_job: u32) -> Result<(), String> {
        let mut queue = self.jobs.lock().expect("Failed to lock queue");
        if let Some(pos) = queue.iter().position(|job| job.id == id_job) {
            queue.remove(pos).unwrap();
            return Ok(());
        }

        Err(format!("Job with ID {} not found", id_job))
    }

    fn len(&self) -> usize {
        let queue = self.jobs.lock().expect("Failed to lock queue");
        queue.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enqueue_test() {
        // here i create my job, with id
        let j = Job::new(1, b"Hello, World!");

        // Then i need to create a Queue
        let mut q = InMemQueue::new();

        match q.enqueue(j) {
            Ok(_) => println!("Job enqueueed successfully."),
            Err(e) => println!("Error {}", e),
        }

        // Now i need to retrieve the Job again
        let res_job = q.get(1);

        // Here we should understand how to assert with Options
        match res_job {
            Some(x) => assert_eq!(x.id, 1),
            None => println!("NOOOOOO!"),
        }
    }
    #[test]
    fn dequeue_test() {
        // here i create my job, with id
        let j = Job::new(1, b"Hello, World!");

        // Then i need to create a Queue
        let mut q = InMemQueue::new();

        match q.enqueue(j) {
            Ok(_) => println!("Job enqueueed successfully."),
            Err(e) => println!("Error {}", e),
        }

        assert_eq!(q.len(), 1); // contains 1 element

        match q.dequeue(1) {
            Ok(_) => println!("Job dequeued successfully."),
            Err(e) => println!("Error {}", e),
        }

        assert_eq!(q.len(), 0); // contains no element
    }
}
