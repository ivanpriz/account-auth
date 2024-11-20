use crate::application::traits::TopicsManagerT;

pub struct TopicsManagerKafka {}

impl TopicsManagerT for TopicsManagerKafka {
    fn create_topic(&mut self, topic_name: &str) -> impl std::future::Future<Output = ()> {
        todo!()
    }

    fn listen_to_topic<MessageSchema, F, Fut>(&mut self, callback: F)
    where
        F: FnMut(MessageSchema) -> Fut,
        Fut: std::future::Future<Output = ()>,
    {
        todo!()
    }

    fn publish_to_topic<MessageSchema>(
        &mut self,
        message: MessageSchema,
        topic_name: &str,
    ) -> impl std::future::Future<Output = ()> {
        todo!()
    }
}
