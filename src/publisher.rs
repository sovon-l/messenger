pub trait Publisher<M> {
    fn publish(&mut self, msg: M);
}

pub fn publisher_loop<M>(rcv: crossbeam_channel::Receiver<M>, mut publisher: impl Publisher<M>) {
    loop {
        match rcv.recv() {
            Ok(msg) => publisher.publish(msg),
            Err(e) => log::error!("{}", e),
        }
    }
}

// below are implementation examples

// 1. zmq publisher
pub type ZmqPublisher = zmq::Socket;

impl<M: Into<zmq::Message> + Clone> Publisher<M> for ZmqPublisher {
    fn publish(&mut self, msg: M) {
        self.send(&msg, zmq::DONTWAIT).unwrap()
    }
}

// 2.bus publisher
pub type BusPublisher<M> = bus::Bus<M>;

impl<M> Publisher<M> for BusPublisher<M> {
    fn publish(&mut self, msg: M) {
        self.broadcast(msg)
    }
}
