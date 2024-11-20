use std::future::Future;

pub trait TopicsManagerT {
    // todo maybe I actually don't need topic creation methods in the trait.
    // because maybe I'm not gonna store topic/queues/exchange objects.
    // Or maybe I will, this might be a good idead generally to store them.

    fn create_topic(&mut self, topic_name: &str) -> impl Future<Output = ()>;

    fn listen_to_topic<MessageSchema, F, Fut>(&mut self, callback: F)
    where
        F: FnMut(MessageSchema) -> Fut,
        Fut: Future<Output = ()>;

    fn publish_to_topic<MessageSchema>(
        &mut self,
        message: MessageSchema,
        topic_name: &str,
    ) -> impl Future<Output = ()>;
}
