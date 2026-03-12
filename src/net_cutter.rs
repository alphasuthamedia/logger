use chrono::Local;
use rdkafka::producer::{BaseProducer, BaseRecord};
use std::{process::Command, process::Stdio, thread, time::Duration};

const IFACE: &str = "eth0";
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

pub fn net_cutter(producer: &BaseProducer) {
    let mut current_network_status = NetworkStatus::Up;
    // testing
    let _ = producer.send(BaseRecord::to("logging").key("key").payload("Hello"));

    let toggle_iface_state = |state: NetworkStatus| {
        Command::new("ip")
            .args(["link", "set", IFACE, state.as_str()])
            .status()
            .unwrap();
    };

    let publish_state = |state: NetworkStatus| {
        let now = Local::now();
        let mut publised_text = now.format("%d %B %Y - %H:%M:%S ").to_string();
        publised_text.push_str(IFACE);
        publised_text.push_str(" ");
        publised_text.push_str(state.as_str());

        let _ = producer.send(
            BaseRecord::to("logging")
                .key("key")
                .payload(publised_text.as_str()),
        );
    };

    loop {
        thread::sleep(Duration::from_secs(300));
        let ping_status = Command::new("ping")
            .args(["-I", IFACE, "-c", "3", "8.8.8.8"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .unwrap();

        match (&current_network_status, ping_status.success()) {
            (NetworkStatus::Up, true) => continue, // on + berhasil = repeat
            (NetworkStatus::Up, false) => {
                // on + gagal = down
                toggle_iface_state(NetworkStatus::Down);
                publish_state(NetworkStatus::Down);
                current_network_status = NetworkStatus::Down;
            }
            (NetworkStatus::Down, false) => continue, // off + gagal = repeat
            (NetworkStatus::Down, true) => {
                // off + berhasil = up
                toggle_iface_state(NetworkStatus::Up);
                publish_state(NetworkStatus::Up);
                current_network_status = NetworkStatus::Up;
            }
        }
    }
}
