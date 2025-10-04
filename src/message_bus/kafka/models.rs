use log::info;
use rdkafka::client::ClientContext;
use std::collections::HashMap;
use std::fmt::Display;

use rdkafka::consumer::{BaseConsumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::topic_partition_list::TopicPartitionList;
use serde::{Deserialize, Serialize};

pub struct CustomContext;
impl ClientContext for CustomContext {}
impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, _: &BaseConsumer<Self>, rebalance: &Rebalance) {
        info!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, _: &BaseConsumer<Self>, rebalance: &Rebalance) {
        info!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        info!("Committing offsets: {:?}", result);
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct IncomingMessage<T> {
    pub headers: HashMap<String, String>,
    pub payload: T,
}

impl<T> Display for IncomingMessage<T>
where
    for<'a> T: Serialize + Deserialize<'a>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::json!(self))
    }
}
