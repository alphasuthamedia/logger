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

fn ping_iface(iface: &str) -> bool {
    Command::new("ping")
        .args(["-I", iface, "-c", "3", "8.8.8.8"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn ping_any() -> bool {
    Command::new("ping")
        .args(["-c", "3", "8.8.8.8"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn net_test(producer: &FutureProducer) {
    let mut current_network_status = NetworkStatus::Up;

    let _ = producer.send(
        FutureRecord::to("logging")
            .key("key")
            .payload(current_network_status.as_str()),
        Duration::from_secs(0),
    );

    let publish = |label: &str, state: &NetworkStatus| {
        let now = Local::now();
        let mut text = now.format("%d %B %Y - %H:%M:%S ").to_string();
        text.push_str(label);
        text.push(' ');
        text.push_str(state.as_str());
        let _ = producer.send(
            FutureRecord::to("logging")
                .key("key")
                .payload(text.as_str()),
            Duration::from_secs(0),
        );
        let _ = producer.flush(Duration::from_secs(3));
    };

    loop {
        thread::sleep(Duration::from_secs(300));

        let eth0_up = ping_iface("eth0");

        if eth0_up {
            match &current_network_status {
                NetworkStatus::Up => continue,
                NetworkStatus::Down => {
                    publish("eth0", &NetworkStatus::Up);
                    current_network_status = NetworkStatus::Up;
                }
            }
        } else {
            let any_up = ping_any();
            match (&current_network_status, any_up) {
                (NetworkStatus::Up, true) => continue,
                (NetworkStatus::Up, false) => {
                    publish("internet", &NetworkStatus::Down);
                    current_network_status = NetworkStatus::Down;
                }
                (NetworkStatus::Down, false) => continue,
                (NetworkStatus::Down, true) => {
                    publish("internet", &NetworkStatus::Up);
                    current_network_status = NetworkStatus::Up;
                }
            }
        }
    }
}
