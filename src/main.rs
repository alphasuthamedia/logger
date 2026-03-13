use rdkafka::producer::DefaultProducerContext;
use rdkafka::{config::ClientConfig, producer::ThreadedProducer};
use std::env;
use std::ops::Deref;
use std::sync::{Arc, OnceLock};

mod btrfs_scrub;
mod net_monitor;
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

    let producer: Arc<ThreadedProducer<DefaultProducerContext>> = Arc::new(
        ClientConfig::new()
            .set(
                "bootstrap.servers",
                KAFKA_SERVER.get().expect("KAFKA_SERVER hasnt been set yet"),
            )
            .create()
            .expect("kafkane error"),
    );

    let consumer_handle = tokio::spawn(telegram_consumer::telegram_consumer());

    // yang perlu dimutex-kan adalah yang polling, takutnya poling hancur karena race condition,
    // kalau up kebuffer mah lock-free ke partisi lokal, aman. ThreadedProducer itu punya thread
    // sendiri buat ngurusin polling2an, jadi udah diurusin dari sana, kita gak bikin pollingan
    // sendiri, jadi aman full dari race condition, gausah dibikin mutex.
    let producer_clone = Arc::clone(&producer);
    let net_test_handle = tokio::task::spawn_blocking(move || {
        net_monitor::net_status(&*producer_clone);
    });

    let producer_clone = Arc::clone(&producer);
    let btrfs_handle = tokio::task::spawn_blocking(move || {
        btrfs_scrub::btrfs_scrub(producer_clone.deref());
    });

    // tunggu semua selesai
    consumer_handle.await.unwrap();
    net_test_handle.await.unwrap();
    btrfs_handle.await.unwrap();
}
