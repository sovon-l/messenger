
pub trait Channel<M: Send> {
    type Sender: ChannelSender<M>;
    type Receiver: ChannelReceiver<Message = M>;
    fn new() -> (Self::Sender, Self::Receiver);
}

// if fail to send, return the msg object
pub trait ChannelSender<M: Send> {
    fn send(&mut self, msg: M) -> Result<(), M>;
}

pub trait ChannelReceiver {
    type Message: Send;
    fn recv(&mut self) -> Result<Self::Message, ()>;
}
