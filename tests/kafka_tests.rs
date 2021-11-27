use std::sync::Arc;
use std::time::Duration;

use assert_cmd;
use futures::executor;
use log4rs;
use predicates::prelude::predicate;
use rdkafka::message::ToBytes;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::producer::future_producer::OwnedDeliveryResult;
use rdkafka::util::Timeout;

mod kafka;

#[test]
fn kafka_tests() {
    log4rs::init_file("config/log4rs-test.yaml", Default::default()).unwrap();

    kafka::while_runs_do(produces_records)
}

fn produces_records(producer: Arc<FutureProducer>) {
    vec![
        FutureRecord::to("topic1").payload("payload").key("123"),
        FutureRecord::to("topic2").payload("payload").key("123"), ]
        .send_in_series(producer.as_ref())
        .for_each(|result| match result {
            Ok((partition_id, offset)) => log::debug!("Successful sent. Partition: {}, offset: {}", partition_id, offset),
            Err((err, _)) => log::error!("Error occurred during record sending. Error: {}", err)
        });


    let mut cmd = assert_cmd::Command::cargo_bin("k4fq").unwrap();
    cmd
        .arg("query")
        .arg("topic1,")
        .arg("topic2")
        .arg("key")
        .arg("eq")
        .arg("123")
        .assert()
        .stderr(predicate::str::contains("\"record_key\":\"123\""))
        .success();
}

trait KafkaSender<'a> {
    fn send_in_series(self, producer: &'a FutureProducer) -> Box<dyn Iterator<Item=OwnedDeliveryResult> + 'a>;
}


impl<'a, 'b: 'a, K: ToBytes + ?Sized, P: ToBytes + ?Sized> KafkaSender<'a> for Vec<FutureRecord<'b, K, P>> {
    fn send_in_series(self,
                      producer: &'a FutureProducer)
                      -> Box<dyn Iterator<Item=OwnedDeliveryResult> + 'a> {
        let result = self.into_iter()
            .map(move |rec|
                producer.send(rec, Timeout::After(Duration::from_secs(1))))
            .map(executor::block_on);
        Box::new(result)
    }
}
