use rdkafka::{config::ClientConfig, producer::FutureProducer};
use std::env;
use std::ops::Deref;
use std::sync::{Arc, Mutex, OnceLock};

mod btrfs_scrub;
mod telegram_consumer;

pub static TOKEN: OnceLock<String> = OnceLock::new();
pub static CHAT_ID: OnceLock<String> = OnceLock::new();
pub static KAFKA_SERVER: OnceLock<String> = OnceLock::new();

#[tokio::main]
async fn main() {
    TOKEN
        .set(env::var("TELEGRAM_TOKEN").expect("TELEGRAM_TOKEN not set"))
        .ok();
    CHAT_ID
        .set(env::var("CHAT_ID").expect("CHAT_ID not set"))
        .ok();
    KAFKA_SERVER
        .set(env::var("KAFKA_SERVER").expect("KAFKA_SERVER not set"))
        .ok();

    let producer: FutureProducer = ClientConfig::new()
        .set(
            "bootstrap.servers",
            KAFKA_SERVER.get().expect("KAFKA_SERVER hasnt been set yet"),
        )
        .create()
        .expect("kafkane error");

    let producer = Arc::new(Mutex::new(producer));

    let consumer_handle = tokio::spawn(telegram_consumer::telegram_consumer());

    // let producer_clone = Arc::clone(&producer);
    // let net_cutter_handle = tokio::task::spawn_blocking(move || {
    //     let prod = producer_clone.lock().unwrap();
    //     net_cutter::net_cutter(&*prod);
    // });

    let producer_clone = Arc::clone(&producer);
    let btrfs_handle = tokio::task::spawn_blocking(move || {
        let prod = producer_clone.lock().unwrap();
        btrfs_scrub::btrfs_scrub(prod.deref());
    });

    // tunggu semua selesai
    consumer_handle.await.unwrap();
    // net_cutter_handle.await.unwrap();
    btrfs_handle.await.unwrap();
}
