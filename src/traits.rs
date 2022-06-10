
pub trait Channel {
    type Message: Send;
    type Sender: ChannelSender<Self::Message>;
    type Receiver: ChannelReceiver<Message = Self::Message>;
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
