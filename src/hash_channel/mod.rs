pub mod crossbeam;

use std::hash::Hash;

// ----------------------
// requirements to be inner channel of a hashed channel

pub trait Channel<M: Send> {
    type Sender: ChannelSender<Message = M>;
    type Receiver: ChannelReceiver<Message = M>;
    fn new() -> (Self::Sender, Self::Receiver);
}

// if fail to send, return the msg object
pub trait ChannelSender {
    type Message: Send;
    fn send(&self, msg: Self::Message) -> Result<(), Self::Message>;
}

pub trait ChannelReceiver {
    type Message: Send;
    fn recv(&mut self) -> Result<Self::Message, ()>;
}

// ------------------------

pub struct HashChannelSender<S: ChannelSender> {
    senders: Vec<S>,
}

impl<M: Hash, S: ChannelSender<Message = M>> HashChannelSender<S> {
    pub fn send(&self, msg: S::Message) -> Result<(), S::Message> {
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

// This is better and sound function signature but requires https://github.com/rust-lang/rust/issues/52662
// pub fn new_hash_channel<M, C>(n: usize) -> (HashChannelSender<C::Sender>, HashChannelReceiver<C::Receiver>)
// where
//     M: Hash + Send,
//     C: Channel<M, Sender: ChannelSender<Message = M>, Receiver: ChannelReceiver<Message = M>>,
pub fn new_hash_channel<M, S, R, C>(n: usize) -> (HashChannelSender<C::Sender>, HashChannelReceiver<C::Receiver>)
where
    M: Hash + Send,
    S: ChannelSender<Message = M>,
    R: ChannelReceiver<Message = M>,
    C: Channel<M, Sender = S, Receiver = R>,
{
    let mut senders = vec![];
    let mut receivers = vec![];
    for _ in 0..n {
        let (tx, rx) = C::new();
        senders.push(tx);
        receivers.push(rx);
    }
    (
        HashChannelSender { senders },
        HashChannelReceiver { receivers },
    )
}

// pub fn new_hash_channel<
//     M: Hash + Send,
//     S: ChannelSender<Message = M>,
//     R: ChannelReceiver<Message = M>,
//     C: Channel<
//         M,
//         Sender = S,
//         Receiver = R,
//     >,
// >(n: usize) -> (HashChannelSender<S>, HashChannelReceiver<R>) {
//     let mut senders = vec![];
//     let mut receivers = vec![];
//     for _ in 0..n {
//         let (tx, rx) = C::new();
//         senders.push(tx);
//         receivers.push(rx);
//     }
//     (
//         HashChannelSender { senders },
//         HashChannelReceiver { receivers },
//     )
// }
