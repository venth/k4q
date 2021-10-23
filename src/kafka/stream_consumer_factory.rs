use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::StreamConsumer;

pub trait StreamConsumerFactory: shaku::Interface {
    fn create(&self) -> StreamConsumer;
}

#[derive(shaku::Component)]
#[shaku(interface = StreamConsumerFactory)]
pub struct KafkaStreamConsumerFactory {}

impl StreamConsumerFactory for KafkaStreamConsumerFactory {
    fn create(&self) -> StreamConsumer {
        rdkafka::ClientConfig::new()
            .set("bootstrap.servers", "localhost:9092")
            .set("message.timeout.ms", "5000")
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "true")
            .set_log_level(RDKafkaLogLevel::Debug)
            .create::<StreamConsumer>()
            .expect("Cannot create Kafka Consumer")
    }
}

