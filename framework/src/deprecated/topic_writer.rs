use crate::application::traits::TopicWriterT;

pub struct TopicWriterKafka {}

impl TopicWriterT for TopicWriterKafka {
    async fn publish(&mut self, message: MessageSchema) -> () {}
}
