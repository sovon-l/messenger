
use std::hash::Hash;

pub struct HashChannelSender<M: Hash + Send, S: crate::traits::ChannelSender<M>> {
    senders: Vec<S>,
    // TODO: remove phantom data
    p: std::marker::PhantomData<M>,
}

impl<M: Hash + Send, S: crate::traits::ChannelSender<M>> HashChannelSender<M, S> {
    pub fn send(&mut self, msg: M) -> Result<(), M> {
        use std::hash::Hasher;
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        msg.hash(&mut hasher);
        let idx = hasher.finish() % self.senders.len() as u64;
        self.senders[idx as usize].send(msg)
    }
}

pub struct HashChannelReceiver<R> {
    pub receivers: Vec<R>,
}

// ref: https://github.com/rust-lang/rust/issues/52662
pub fn new_hash_channel<M, C>(n: usize) -> (HashChannelSender<M, C::Sender>, HashChannelReceiver<C::Receiver>)
where
    M: Hash + Send,
    C: crate::traits::Channel<Message = M>,
    <C as crate::traits::Channel>::Sender: crate::traits::ChannelSender<M>,
    <C as crate::traits::Channel>::Receiver: crate::traits::ChannelReceiver<Message = M>,
{
    let mut senders = vec![];
    let mut receivers = vec![];
    for _ in 0..n {
        let (tx, rx) = C::new();
        senders.push(tx);
        receivers.push(rx);
    }
    (
        HashChannelSender { senders, p: std::marker::PhantomData::<M>::default() },
        HashChannelReceiver { receivers },
    )
}
