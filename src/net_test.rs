use chrono::Local;
use rdkafka::producer::{FutureProducer, FutureRecord, Producer};
use std::{process::Command, process::Stdio, thread, time::Duration};

enum NetworkStatus {
    Up,
    Down,
}
impl NetworkStatus {
    fn as_str(&self) -> &str {
        match self {
            NetworkStatus::Up => "up",
            NetworkStatus::Down => "down",
        }
    }
}

pub fn net_test(producer: &FutureProducer) {
    let mut current_network_status = NetworkStatus::Up;
    // testing
    let _ = producer.send(
        FutureRecord::to("logging")
            .key("key")
            .payload(current_network_status.as_str()),
        Duration::from_secs(0),
    );

    let publish_state = |state: NetworkStatus| {
        let now = Local::now();
        let mut publised_text = now.format("%d %B %Y - %H:%M:%S ").to_string();
        publised_text.push_str("Internet ");
        publised_text.push_str(state.as_str());

        let _ = producer.send(
            FutureRecord::to("logging")
                .key("key")
                .payload(publised_text.as_str()),
            Duration::from_secs(0),
        );

        let _ = producer.flush(Duration::from_secs(3));
    };

    loop {
        thread::sleep(Duration::from_secs(300));
        let ping_status = Command::new("ping")
            .args(["-c", "3", "8.8.8.8"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .unwrap();

        match (&current_network_status, ping_status.success()) {
            (NetworkStatus::Up, true) => continue, // on + berhasil = repeat
            (NetworkStatus::Up, false) => {
                // on + gagal = down
                publish_state(NetworkStatus::Down);
                current_network_status = NetworkStatus::Down;
            }
            (NetworkStatus::Down, false) => continue, // off + gagal = repeat
            (NetworkStatus::Down, true) => {
                // off + berhasil = up
                publish_state(NetworkStatus::Up);
                current_network_status = NetworkStatus::Up;
            }
        }
    }
}
