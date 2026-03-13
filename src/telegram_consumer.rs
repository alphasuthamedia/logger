use futures::StreamExt;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use reqwest::Client;
use reqwest::multipart;

// untuk ada biiar biisa ngakes root - main.rs
use crate::CHAT_ID;
use crate::TOKEN;
pub async fn telegram_consumer() {
    let token = TOKEN.get().expect("TOKEN hasnt been set yet");
    let chat_id = CHAT_ID.get().expect("CHAT_ID hasnt been set yet");
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("group.id", "logger") // bebas bikin nama
        .create()
        .unwrap_or_else(|e| panic!("kafkane - consumer error {}", e));

    consumer
        .subscribe(&["logging"])
        .expect("Can't subscribe to specified topic");

    let telegram = Client::new();
    while let Some(msg) = consumer.stream().next().await {
        let msg = msg.expect("Kafka error - consumer");
        let payload = msg
            .payload_view::<str>()
            .and_then(|r| r.ok())
            .unwrap_or("<EMPTY>");

        let form = multipart::Form::new()
            .text("chat_id", chat_id.clone())
            .text("text", payload.to_string());

        telegram
            .post(format!("https://api.telegram.org/bot{token}/sendMessage"))
            .multipart(form)
            .send()
            .await
            .expect("gagal kirim");
    }
    println!("Consumer Done ~")
}
