mod utils;

use std::io::BufRead;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use utils::client::Client;
use utils::config::Config;
use utils::worker::SubmitWorker;
use utils::worker::Worker;

use cn_stratum::client::PoolClient;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

use log::*;

const AGENT: &str = "pow#er/0.2.0";

fn main() {
    env_logger::init();

    let panicker = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        eprintln!("panicked");
        panicker(info);
        std::process::exit(1);
    }));

    let args = clap::App::new("Pow#er")
        .author("Francesco Palazzini <palazzini.francesco@gmail.com>")
        .arg(
            clap::Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let cfg: Config = std::fs::read_to_string(args.value_of("config").unwrap())
        .map(|s| toml::from_str(&s))
        .unwrap()
        .unwrap();
    debug!("config: {:?}", &cfg);

    let mut rx_flags = randomx_rs::RandomXFlag::FLAG_DEFAULT;

    if cfg.randomx.hard_aes {
        rx_flags = rx_flags | randomx_rs::RandomXFlag::FLAG_HARD_AES;
    }

    if cfg.randomx.jit {
        rx_flags = rx_flags | randomx_rs::RandomXFlag::FLAG_JIT;
    }

    if cfg.randomx.argon2_avx2 {
        rx_flags = rx_flags | randomx_rs::RandomXFlag::FLAG_ARGON2_AVX2;
    }

    if cfg.randomx.full_mem {
        rx_flags = rx_flags | randomx_rs::RandomXFlag::FLAG_FULL_MEM;
    }

    if cfg.randomx.large_pages {
        rx_flags = rx_flags | randomx_rs::RandomXFlag::FLAG_LARGE_PAGES;
    }

    if cfg.randomx.argon2_ssse3 {
        rx_flags = rx_flags | randomx_rs::RandomXFlag::FLAG_ARGON2_SSSE3;
    }

    let client = PoolClient::connect(
        &cfg.pool.address,
        &cfg.pool.login,
        &cfg.pool.pass,
        cfg.pool.keepalive_s.map(Duration::from_secs),
        AGENT,
        Client::new,
    )
    .unwrap();
    let work = client.handler().work();
    let pool = client.write_handle();
    thread::Builder::new()
        .name("poolclient".into())
        .spawn(move || client.run())
        .unwrap();

    let core_ids = core_affinity::get_core_ids().unwrap();
    let worker_count = cfg.randomx.cores.len();
    let mut workerstats = Vec::with_capacity(cfg.randomx.cores.len());

    let (tx, rx): (Sender<_>, Receiver<_>) = mpsc::channel();

    let submit_worker = SubmitWorker {};

    thread::Builder::new()
        .name("sender".into())
        .spawn(move || {
            submit_worker.submit_share(rx, pool);
        })
        .unwrap();

    for (i, w) in cfg.randomx.cores.into_iter().enumerate() {
        let hash_count = Arc::new(AtomicUsize::new(0));
        workerstats.push(Arc::clone(&hash_count));
        let core = core_ids[w as usize];
        debug!("starting worker{} on core {:?}", i, w);
        let worker = Worker {
            hash_count,
            work: Arc::clone(&work),
            core,
            worker_id: i as u32,
            step: worker_count as u32,
        };

        let thread_tx = tx.clone();

        thread::Builder::new()
            .name(format!("worker{}", i))
            .spawn(move || {
                core_affinity::set_for_current(core);
                worker.run(rx_flags, thread_tx)
            })
            .unwrap();
    }

    let mut prevstats: Vec<_> = workerstats
        .iter()
        .map(|w| w.load(Ordering::Relaxed))
        .collect();

    let start = Instant::now();
    let mut prev_start = start;
    let mut total_hashes = 0;
    let stdin = std::io::stdin();
    let mut await_input = stdin.lock().lines();
    loop {
        info!("worker stats (since last):");
        let now = Instant::now();
        let cur_dur = now - prev_start;
        let total_dur = now - start;
        prev_start = now;
        let mut cur_hashes = 0;
        for (i, (prev, new)) in prevstats.iter_mut().zip(&workerstats).enumerate() {
            let new = new.load(Ordering::Relaxed);
            let cur = new - *prev;
            println!("\t{}: {} H/s", i, (cur as f32) / dur_to_f32(&cur_dur));
            cur_hashes += cur;
            *prev = new;
        }
        total_hashes += cur_hashes;
        println!(
            "\ttotal (since last): {} H/s",
            (cur_hashes as f32) / dur_to_f32(&cur_dur)
        );
        println!(
            "\ttotal (all time): {} H/s",
            (total_hashes as f32) / dur_to_f32(&total_dur)
        );
        await_input.next();
    }
}

fn dur_to_f32(dur: &Duration) -> f32 {
    (dur.as_secs() as f32) + (dur.subsec_nanos() as f32) / 1_000_000_000.0
}
