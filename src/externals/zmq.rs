
pub type Sender = zmq::Socket;

impl<M: Into<zmq::Message> + Clone + Send> crate::traits::ChannelSender<M> for Sender {
    fn send(&mut self, msg: M) -> Result<(), M> {
        match zmq::Socket::send(self, &msg, zmq::DONTWAIT) {
            Ok(m) => Ok(m),
            Err(e) => {
                log::error!("fail to send zmq msg becoz {}", e);
                Err(msg)
            },
        }
    }
}
