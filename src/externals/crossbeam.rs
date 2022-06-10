pub type Sender<M> = crossbeam_channel::Sender<M>;

impl<M: Send> crate::traits::ChannelSender<M> for Sender<M> {
    fn send(&mut self, msg: M) -> Result<(), M> {
        match crossbeam_channel::Sender::<M>::send(self, msg) {
            Ok(m) => Ok(m),
            Err(e) => Err(e.into_inner()),
        }
    }
}

pub type Receiver<M> = crossbeam_channel::Receiver<M>;

impl<M: Send> crate::traits::ChannelReceiver for Receiver<M> {
    type Message = M;
    fn recv(&mut self) -> Result<M, ()> {
        match crossbeam_channel::Receiver::<M>::recv(self) {
            Ok(m) => Ok(m),
            Err(e) => {
                log::error!("{}", e);
                Err(())
            }
        }
    }
}

pub type CrossbeamChannels<M> = (Sender<M>, Receiver<M>);

impl<M: std::hash::Hash + Send> crate::traits::Channel<M> for CrossbeamChannels<M> {
    type Sender = Sender<M>;
    type Receiver = crossbeam_channel::Receiver<M>;
    fn new() -> (Self::Sender, Self::Receiver) {
        crossbeam_channel::unbounded()
    }
}
