use do_notation::m;
use rdkafka::metadata::{Metadata, MetadataTopic};
use crate::domain::model;
use crate::kafka::kafka_reader::{SpecificTopicReader, TimeoutAwareKafkaReader};
use crate::kafka::timeout_aware_stream_consumer::TimeoutAwareStreamConsumer;
use crate::monads::Reader;
use crate::monads::ResultT;

pub fn new<'a>(metadata_reader: SpecificTopicReader<'a, Metadata>)
               -> SpecificTopicReader<&'a MetadataTopic> {
    let topic_name_supplier = || topic_name.clone();
    let topic = m! {
            metadata <- ResultT::lift(self.fetch_metadata_for(topic_name_supplier()));
            let topic = Self::first_of(metadata.topics());
            let topic_metadata = topic.ok_or(K4fqError::KafkaError(format!("Cannot find topic: {:?}", topic_name_supplier())));


        ResultT::lift(m! {
                partitions <- self.fetch_partitions_for(topic_metadata);

                Reader::unit(partitions.map(move |p| Topic::new(topic_name_supplier(), p)))
            })
        };
}