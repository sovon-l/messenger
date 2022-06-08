pub type CrossbeamSender<M> = crossbeam_channel::Sender<M>;

impl<M: Send> crate::hash_channel::ChannelSender for CrossbeamSender<M> {
    type Message = M;
    fn send(&self, msg: M) -> Result<(), M> {
        match crossbeam_channel::Sender::<M>::send(self, msg) {
            Ok(m) => Ok(m),
            Err(e) => Err(e.into_inner()),
        }
    }
}

pub type CrossbeamReceiver<M> = crossbeam_channel::Receiver<M>;

impl<M: Send> crate::hash_channel::ChannelReceiver for CrossbeamReceiver<M> {
    type Message = M;
    fn recv(&mut self) -> Result<M, ()> {
        match crossbeam_channel::Receiver::<M>::recv(self) {
            Ok(m) => Ok(m),
            Err(e) => {
                log::error!("{}", e);
                Err(())
            },
        }
    }
}

pub type CrossbeamChannels<M> = (
    CrossbeamSender<M>,
    CrossbeamReceiver<M>,
);

impl<M: std::hash::Hash + Send> crate::hash_channel::Channel<M> for CrossbeamChannels<M> {
    type Sender = CrossbeamSender<M>;
    type Receiver = crossbeam_channel::Receiver<M>;
    fn new() -> (Self::Sender, Self::Receiver) {
        crossbeam_channel::unbounded()
    }
}
