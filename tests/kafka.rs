use std::panic;
use std::sync::Arc;

use rdkafka::ClientConfig;
use rdkafka::producer::FutureProducer;
use testcontainers::{clients, Container};
use testcontainers_modules::kafka;

pub fn while_runs_do<'a, F>(f: F) where
    F: FnOnce(Arc<FutureProducer>) -> () + panic::UnwindSafe {
    let cli = clients::Cli::default();
    // it seems that #run function is wrongly implemented - it passes to its result `docker` reference
    // which prevents to store the result in a struct :(
    let server = cli.run(kafka::Kafka::default());

    server.start();
    let producer = Arc::new(create_producer(&server));

    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| { f(producer.clone()); }));

    server.stop();

    if let Err(err) = result {
        panic::resume_unwind(err);
    }
}

fn create_producer(kafka_node: &Container<kafka::Kafka>) -> FutureProducer {
    let bootstrap_servers = format!("localhost:{}",
                                    kafka_node.get_host_port_ipv4(kafka::KAFKA_PORT));
    ClientConfig::new()
        .set("bootstrap.servers", &bootstrap_servers)
        .set("message.timeout.ms", "5000")
        .create::<FutureProducer>()
        .expect("Failed to create Kafka FutureProducer")
}
