use chrono::Local;
use rdkafka::producer::{BaseProducer, BaseRecord, Producer};
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

fn set_iface(iface: &str, up: bool) {
    let state = if up { "up" } else { "down" };
    Command::new("ip")
        .args(["link", "set", iface, state])
        .status()
        .unwrap();
}

fn publish(producer: &BaseProducer, label: &str, state: &NetworkStatus) {
    let now = Local::now();
    let mut text = now.format("%d %B %Y - %H:%M:%S ").to_string();
    text.push_str(label);
    text.push(' ');
    text.push_str(state.as_str());

    producer
        .send(BaseRecord::to("logging").key("key").payload(text.as_str()))
        .unwrap();

    producer.flush(Duration::from_secs(0)).unwrap();
}

pub fn link_cut(producer: &BaseProducer) {
    let mut current_network_status = NetworkStatus::Up;

    publish(producer, "init", &current_network_status);

    loop {
        thread::sleep(Duration::from_secs(60));

        match &current_network_status {
            NetworkStatus::Up => {
                if !ping_iface("eth0") {
                    set_iface("eth0", false);
                    current_network_status = NetworkStatus::Down;
                    publish(producer, "eth0", &NetworkStatus::Down);
                }
            }
            NetworkStatus::Down => {
                set_iface("eth0", true); // nyalain dulu
                if ping_iface("eth0") {
                    // baru ping
                    current_network_status = NetworkStatus::Up;
                    publish(producer, "eth0", &NetworkStatus::Up);
                } else {
                    set_iface("eth0", false); // gagal, matiin lagi
                }
            }
        }
    }
}
