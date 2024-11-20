// todo most likely these two traits are to be removed, as topics manager will handle it all.
pub trait TopicListenerT<MessageSchema> {
    fn listen<F, Fut>(&mut self, callback: F)
    where
        F: FnMut(MessageSchema) -> Fut,
        Fut: Future<Output = ()>;
}

pub trait TopicWriterT<MessageSchema> {
    fn publish(&mut self, message: MessageSchema) -> impl Future<Output = ()>;
}

pub trait TopicsManagerT {
    // todo maybe I actually don't need topic creation methods in the trait.
    // because maybe I'm not gonna store topic/queues/exchange objects.
    // Or maybe I will, this might be a good idead generally to store them.
    fn create_topic_handlers<MessageSchema>(
        &mut self,
    ) -> (
        impl TopicListenerT<MessageSchema>,
        impl TopicWriterT<MessageSchema>,
    );
}
