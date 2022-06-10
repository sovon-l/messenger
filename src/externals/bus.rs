
pub type Sender<M> = bus::Bus<M>;

impl<M: Send> crate::traits::ChannelSender<M> for Sender<M> {
    fn send(&mut self, msg: M) -> Result<(), M> {
        self.try_broadcast(msg)
    }
}
