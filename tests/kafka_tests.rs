use std::time::Duration;

use futures::executor;
use log;
use log4rs;
use rdkafka::producer::{FutureProducer, FutureRecord};

mod kafka;

#[test]
fn kafka_tests() {
    log4rs::init_file("config/log4rs-test.yaml", Default::default()).unwrap();

    kafka::while_is_running_do(produces_records)
}

fn produces_records(producer: &FutureProducer) {
    executor::block_on(
        async {
            producer
                .send(FutureRecord::to("test")
                          .payload("213 message")
                          .key(&format!("Key {}", 12)),
                      Duration::from_secs(0), )
                .await
                .unwrap();
            log::debug!("Record produced");
        });
}
