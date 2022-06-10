
pub fn publisher_loop<M: Send>(
    mut rcv: impl crate::traits::ChannelReceiver<Message = M>, 
    mut publisher: impl crate::traits::ChannelSender<M>,
) {
    loop {
        match rcv.recv() {
            Ok(msg) => {
                let _ = publisher.send(msg);
            },
            Err(()) => {
                log::error!("error: publisher_loop cannot receive msg");
            },
        }
    }
}

pub trait Listener<M> {
    fn on_msg(&mut self, msg: M);
}

pub trait Subscriber<M: Send> : crate::traits::ChannelReceiver<Message = M> {
    fn init(&mut self) {}
    fn recv(&mut self) -> Option<M> {
        crate::traits::ChannelReceiver::recv(self).ok()
    }
}

pub fn subscriber_loop<M: Send>(mut subscriber: impl Subscriber<M>, mut listener: impl Listener<M>) {
    subscriber.init();
    while let Some(msg) = Subscriber::recv(&mut subscriber) {
        listener.on_msg(msg);
    }
    log::warn!("a subscription dropped!");
}

// below are implementation examples (old code from subscriber)

// // 1. crossbeam subscriber
// pub type CrossbeamSubscriber<M> = crossbeam_channel::Receiver<M>;

// impl<M> Subscriber<M> for CrossbeamSubscriber<M> {
//     fn recv(&mut self) -> Option<M> {
//         match crossbeam_channel::Receiver::<M>::recv(self) {
//             Ok(m) => Some(m),
//             Err(e) => {
//                 log::error!(
//                     "{}",
//                     serde_json::json!({"error":e.to_string(), "msg":"bus fail to recv msg"})
//                         .to_string()
//                 );
//                 None
//             }
//         }
//     }
// }

// // 2. bus subscriber
// pub type BusSubscriber<M> = bus::BusReader<M>;

// impl<M: Clone + Sync> Subscriber<M> for BusSubscriber<M> {
//     fn recv(&mut self) -> Option<M> {
//         match bus::BusReader::recv(self) {
//             Ok(m) => Some(m),
//             Err(e) => {
//                 log::error!(
//                     "{}",
//                     serde_json::json!({"error":e.to_string(), "msg":"bus fail to recv msg"})
//                         .to_string()
//                 );
//                 None
//             }
//         }
//     }
// }

// // 3. zmq subscriber
// pub type ZmqSubscriber = zmq::Socket;

// // TODO: can implement for appropriate lifetime for better perf?
// impl<M: From<Vec<u8>>> Subscriber<M> for ZmqSubscriber {
//     fn init(&mut self) {
//         self.set_subscribe(b"").unwrap();
//     }
//     fn recv(&mut self) -> Option<M> {
//         match self.recv_bytes(0) {
//             Ok(m) => Some(M::from(m)),
//             Err(e) => {
//                 log::error!(
//                     "{}",
//                     serde_json::json!({"error":e.to_string(), "msg":"zmq fail to recv byte"})
//                         .to_string()
//                 );
//                 None
//             }
//         }
//     }
// }
