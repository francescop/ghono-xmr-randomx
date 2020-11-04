use crate::utils::pack_nonce;
use crate::utils::unhexlify;
use crate::utils::work::Work;
use byteorder::{ByteOrder, LE};
use cn_stratum::client::{
    ErrorReply, Job, JobAssignment, MessageHandler, PoolClient, PoolClientWriter, RequestId,
};
use core_affinity::CoreId;
use hex::FromHex;
use log::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};

pub struct Worker {
    pub hash_count: Arc<AtomicUsize>,
    pub work: Arc<Work>,
    pub core: CoreId,
    pub worker_id: u32,
    pub step: u32,
}

impl Worker {
    pub fn run(self, rx_flags: randomx_rs::RandomXFlag, tx: Sender<Res>) -> () {
        debug!("init worker");

        let (_, job) = self.work.current();

        let mut current_seed = job.seed_hash();
        let seed_hash = <[u8; 32]>::from_hex(&job.seed_hash()).unwrap();

        let rx_cache = randomx_rs::RandomXCache::new(rx_flags, &seed_hash).unwrap();
        let rx_dataset = randomx_rs::RandomXDataset::new(rx_flags, &rx_cache, 0).unwrap();
        let rx_vm =
            randomx_rs::RandomXVM::new(rx_flags, Some(&rx_cache), Some(&rx_dataset)).unwrap();

        loop {
            trace!("getting work");
            let (jid, job) = self.work.current();

            let target = job.target();
            let mut blob_hash = job.blob.clone();

            if current_seed != job.seed_hash() {
                current_seed = job.seed_hash();
                let current_seed_hash = unhexlify(&job.seed_hash()).expect("unhexlify error");
                debug!("new seed: {:?}", current_seed_hash);
                debug!("reinit vm");
                drop(&rx_cache);
                drop(&rx_dataset);
                let rx_cache = randomx_rs::RandomXCache::new(rx_flags, &current_seed_hash).unwrap();
                let rx_dataset = randomx_rs::RandomXDataset::new(rx_flags, &rx_cache, 0).unwrap();
                rx_vm
                    .reinit_cache(&rx_cache)
                    .expect("error reinitializing cache");
                rx_vm
                    .reinit_dataset(&rx_dataset)
                    .expect("error reinitializing cache");
            }

            let start = (u32::from(blob_hash[42]) << 24) + self.worker_id;
            let nonce_seq = (start..).step_by(self.step as usize);
            let mut rx_hash = [0u8; 32];
            for nonce in nonce_seq {
                if !self.work.is_current(jid) {
                    break;
                }
                pack_nonce(&mut blob_hash, &nonce.to_le_bytes());
                rx_hash = rx_vm.calculate_hash(&mut blob_hash).unwrap();
                if LE::read_u64(&rx_hash[24..]) <= target {
                    debug!("submitting share {:?}", blob_hash);
                    tx.send(Res {
                        job: job.clone(),
                        nonce: nonce,
                        hash: rx_hash.clone(),
                    });
                    /*
                    self.pool
                        .lock()
                        .unwrap()
                        .submit(&job, nonce, &rx_hash)
                        .unwrap();
                    */
                }
                self.hash_count.fetch_add(1, Ordering::Relaxed);
            }
        }
    }
}

use crate::utils;
pub struct Res {
    pub job: cn_stratum::client::Job,
    pub nonce: u32,
    pub hash: [u8; 32],
}

pub struct SubmitWorker {}

impl SubmitWorker {
    pub fn submit_share(self, rx: Receiver<Res>, pool: Arc<Mutex<PoolClientWriter>>) {
        loop {
            let res = rx.recv();
            match res {
                Ok(r) => {
                    debug!("ready to submit nonce: {}", r.nonce);
                    pool.lock().unwrap().submit(&r.job, r.nonce, &r.hash);
                }
                Err(_) => error!("error submitting share"),
            }
        }
    }
}
