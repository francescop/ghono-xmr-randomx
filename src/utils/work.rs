use cn_stratum::client::{
    ErrorReply, Job, JobAssignment, MessageHandler, PoolClient, PoolClientWriter, RequestId,
};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct JobId(usize);

pub struct Work {
    job_id: AtomicUsize,
    job: Mutex<Job>,
}

impl Work {
    pub fn new(job: Job) -> Self {
        let job_id = AtomicUsize::new(0);
        let job = Mutex::new(job);
        Work { job_id, job }
    }
    pub fn is_current(&self, jid: JobId) -> bool {
        jid == JobId(self.job_id.load(Ordering::Relaxed))
    }
    pub fn current(&self) -> (JobId, Job) {
        (
            JobId(self.job_id.load(Ordering::Acquire)),
            self.job.lock().unwrap().clone(),
        )
    }
    pub fn set_current(&self, j: Job) {
        *self.job.lock().unwrap() = j;
        self.job_id.fetch_add(1, Ordering::Release);
    }
}
