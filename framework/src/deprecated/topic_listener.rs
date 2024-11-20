use crate::application::traits::TopicListenerT;

pub struct TopicsListenerKafka<MessageSchema> {}

impl<MessageSchema> TopicListenerT<MessageSchema> for TopicsListenerKafka<MessageSchema> {
    fn listen<F, Fut>(&mut self, callback: F)
    where
        F: FnMut(MessageSchema) -> Fut,
        Fut: std::future::Future<Output = ()>,
    {
    }
}
