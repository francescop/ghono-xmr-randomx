use crate::utils::work::Work;
use cn_stratum::client::{ErrorReply, Job, JobAssignment, MessageHandler, RequestId};
use log::*;
use std::sync::Arc;

pub struct Client {
    work: Arc<Work>,
}

impl Client {
    pub fn new(job: Job) -> Self {
        let work = Arc::new(Work::new(job));
        Client { work }
    }

    pub fn work(&self) -> Arc<Work> {
        Arc::clone(&self.work)
    }
}

impl MessageHandler for Client {
    fn job_command(&mut self, j: Job) {
        debug!("new job: {:?}", j);
        self.work.set_current(j);
    }

    fn error_reply(&mut self, _id: RequestId, error: ErrorReply) {
        warn!(
            "received error: {:?}, assuming that indicates a stale share",
            error
        );
    }

    fn status_reply(&mut self, _id: RequestId, status: String) {
        if status == "OK" {
            debug!("received status OK");
        } else {
            info!("received status {:?}, assuming that means OK", status);
        }
    }

    fn job_reply(&mut self, _id: RequestId, _job: Box<JobAssignment>) {
        warn!("unexpected job reply...");
    }
}
