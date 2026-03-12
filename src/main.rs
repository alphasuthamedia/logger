use rdkafka::{config::ClientConfig, producer::BaseProducer};
use std::{
    sync::{Arc, Mutex},
    thread,
};

mod btrfs_scrub;
mod net_cutter;

fn main() {
    let mut threads: Vec<thread::JoinHandle<()>> = vec![];
    let producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .unwrap_or_else(|e| panic!("kafkane error {e}"));
    let producer = Arc::new(Mutex::new(producer));

    let producer_clone = Arc::clone(&producer);

    threads.push(thread::spawn(move || {
        let prod = producer_clone.lock().unwrap();
        net_cutter::net_cutter(&*prod);
    }));

    threads.push(thread::spawn(move || {
        let prod = producer_clone.lock().unwrap();
        btrfs_scrub::btrfs_scrub(&*prod)
    }));
    for t in threads.into_iter() {
        t.join().unwrap()
    }
}
