use std::{process::Command, thread, time::Duration};

use rdkafka::producer::{FutureProducer, FutureRecord, Producer};

pub fn btrfs_scrub(producer: &FutureProducer) {
    loop {
        let status = Command::new("btrfs")
            .args(["scrub", "start", "-B", "/mnt/btrfs_pool/"])
            .status()
            .unwrap();

        if !status.success() {
            let msg = format!("btrfs scrub failed with exit code: {}", status);
            let _ = producer.send(
                FutureRecord::to("logging").key("key").payload(msg.as_str()),
                Duration::from_secs(0), // fire-and-forget tapi nanti ditagih (ttp async)
            );
        } else {
            let scrub_status = Command::new("btrfs")
                .args(["scrub", "status", "/mnt/btrfs_pool/"])
                .output()
                .unwrap();
            let msg = String::from_utf8_lossy(&scrub_status.stdout).to_string();
            let _ = producer.send(
                FutureRecord::to("logging").key("key").payload(msg.as_str()),
                Duration::from_secs(0),
            );
            let _ = producer.flush(Duration::from_secs(4));
        }

        thread::sleep(Duration::from_hours(24 * 30));
    }
}
