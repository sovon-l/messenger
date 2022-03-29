
pub trait Listener<M> {
    fn on_msg(&mut self, msg: M);
}

pub trait Subscriber<M> {
    fn init(&mut self);
    fn recv(&mut self) -> Result<M, Box<dyn std::error::Error>>;
}

pub fn subscribe<M>(
    mut subscriber: impl Subscriber<M>,
    mut listener: impl Listener<M>,
) {
    subscriber.init();
    while let Ok(msg) = subscriber.recv() {
        listener.on_msg(msg);
    }
}