use std::{process::Command, thread, time::Duration};

use rdkafka::producer::{BaseProducer, BaseRecord, Producer};

pub fn btrfs_scrub(producer: &BaseProducer) {
    loop {
        // kodinganku rusak btrfs poolku kena scrub terus menerus jir
        thread::sleep(Duration::from_hours(24 * 30));

        let status = Command::new("btrfs")
            .args(["scrub", "start", "-B", "/mnt/btrfs_pool/"])
            .status()
            .unwrap();

        if !status.success() {
            let msg = format!("btrfs scrub failed with exit code: {}", status);
            let _ = producer.send(BaseRecord::to("logging").key("key").payload(msg.as_str()));
        } else {
            let scrub_status = Command::new("btrfs")
                .args(["scrub", "status", "/mnt/btrfs_pool/"])
                .output()
                .unwrap();
            let msg = String::from_utf8_lossy(&scrub_status.stdout).to_string();
            let _ = producer.send(BaseRecord::to("logging").key("key").payload(msg.as_str()));
            let _ = producer.flush(Duration::from_secs(0));
        }
    }
}
