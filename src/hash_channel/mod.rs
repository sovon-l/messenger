use std::hash::Hash;

// create hash channels that distribute message to corresponding receiver according to hash of msg
pub fn new<T: Hash>(n: usize) -> (HashChannelSender<T>, HashChannelReceiver<T>) {
    let mut senders = vec![];
    let mut receivers = vec![];
    for _ in 0..n {
        let (tx, rx) = crossbeam_channel::unbounded();
        senders.push(tx);
        receivers.push(rx);
    }
    (
        HashChannelSender {
            tx: senders,
            n,
        },
        HashChannelReceiver {
            rx: receivers,
        },
    )
}

pub struct HashChannelSender<H: Hash> {
    tx: Vec<crossbeam_channel::Sender<H>>,
    n: usize,
}

impl<H: Hash> HashChannelSender<H> {
    pub fn send(&self, msg: H) -> Result<(), crossbeam_channel::SendError<H>> {
        use std::hash::Hasher;

        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        msg.hash(&mut hasher);
        let idx = hasher.finish() % self.n as u64;
        self.tx[idx as usize].send(msg)
    }
}

pub struct HashChannelReceiver<H: Hash> {
    pub rx: Vec<crossbeam_channel::Receiver<H>>,
}