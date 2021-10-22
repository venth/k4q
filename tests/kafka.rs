use rdkafka::ClientConfig;
use rdkafka::producer::FutureProducer;
use testcontainers::{clients, Container, images};
use testcontainers::images::kafka::Kafka;

pub fn while_is_running_do<F>(f: F) where F: Fn(&FutureProducer) {
    let cli = clients::Cli::default();
    // it seems that #run function is wrongly implemented - it passes to its result `docker` reference
    // which prevents to store the result in a struct :(
    let server = cli.run(images::kafka::Kafka::default());

    server.start();
    let producer = create_producer(&server);

    f(&producer);

    server.stop();
}

struct Test<'a> {
    cli: Box<clients::Cli>,
    server: Box<Container<'a, images::kafka::Kafka>>,
}

fn create_producer(kafka_node: &Container<Kafka>) -> FutureProducer {
    let bootstrap_servers = format!("localhost:{}",
                                    kafka_node.get_host_port(images::kafka::KAFKA_PORT));
    ClientConfig::new()
        .set("bootstrap.servers", &bootstrap_servers)
        .set("message.timeout.ms", "5000")
        .create::<FutureProducer>()
        .expect("Failed to create Kafka FutureProducer")
}
