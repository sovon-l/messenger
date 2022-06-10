
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