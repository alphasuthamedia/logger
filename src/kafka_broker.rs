use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication};
use rdkafka::config::ClientConfig;

use crate::KAFKA_SERVER;
pub async fn start_boker() {
    // generic tsb untuk context (DefaultContext)
    let admin: AdminClient<_> = ClientConfig::new()
        .set(
            "bootstrap.servers",
            KAFKA_SERVER.get().expect("KAFKA_SERVER hasnt been set yet"),
        )
        .create()
        .unwrap();

    let topics = vec![NewTopic::new("logging", 1, TopicReplication::Fixed(1))];
    match admin.create_topics(&topics, &AdminOptions::new()).await {
        Ok(results) => {
            for r in results {
                println!("Topic creation result: {:?}", r);
            }
        }
        Err(e) => println!("Failed to create topic: {:?}", e),
    }
}
