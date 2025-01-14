use crate::domain::model::K4fqError;
use crate::kafka::kafka_reader::{SpecificTopicReader, TimeoutAwareKafkaReader};
use rdkafka::metadata::{Metadata, MetadataTopic};

pub fn new<'a>(
    metadata_reader: SpecificTopicReader<'a, Metadata>,
) -> SpecificTopicReader<&'a MetadataTopic> {
    /*    let topic_name_supplier = || topic_name.clone();
        let topic = m! {
                metadata <- ResultT::lift(self.fetch_metadata_for(topic_name_supplier()));
                let topic = Self::first_of(metadata.topics());
                let topic_metadata = topic.ok_or(K4fqError::KafkaError(format!("Cannot find topic: {:?}", topic_name_supplier())));


            ResultT::lift(m! {
                    partitions <- self.fetch_partitions_for(topic_metadata);

                    Reader::unit(partitions.map(move |p| Topic::new(topic_name_supplier(), p)))
                })
            };
    */
    SpecificTopicReader::new(move |c| {
        TimeoutAwareKafkaReader::unit(Result::Err(K4fqError::NotSupported))
    })
}
