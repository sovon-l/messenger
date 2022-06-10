pub fn main() {
    let (tx, rx) = messenger::hash_channel::new_hash_channel::<
        String,
        messenger::externals::crossbeam::Sender<String>,
        messenger::externals::crossbeam::Receiver<String>,
        messenger::externals::crossbeam::CrossbeamChannels<String>,
    >(8);

    let msgs: Vec<String> = vec![
        " Lorem ",
        " ipsum ",
        " dolor ",
        " sit ",
        " amet, ",
        " consectetur ",
        " adipiscing ",
        " elit, ",
        " sed ",
        " doneiusmod ",
        " tempor ",
        " incididunt ",
        " ut ",
        " labore ",
        " et ",
        " dolore ",
        " magna ",
        " aliqua. ",
        " Ut ",
        " enim ",
        " adnminim ",
        " veniam, ",
        " quis ",
        " nostrud ",
        " exercitation ",
        " ullamco ",
        " laboris ",
        " nisi ",
        " utnaliquip ",
        " ex ",
        " ea ",
        " commodo ",
        " consequat. ",
        " Duis ",
        " aute ",
        " irure ",
        " dolor ",
        " innreprehenderit ",
        " in ",
        " voluptate ",
        " velit ",
        " esse ",
        " cillum ",
        " dolore ",
        " eu ",
        " fugiat ",
        " nullanpariatur. ",
        " Excepteur ",
        " sint ",
        " occaecat ",
        " cupidatat ",
        " non ",
        " proident, ",
        " sunt ",
        " innculpa ",
        " qui ",
        " officia ",
        " deserunt ",
        " mollit ",
        " anim ",
        " id ",
        " est ",
        " laborum ",
    ].into_iter().map(|s| s.to_string()).collect();

    for msg in msgs.into_iter() {
        tx.send(msg).unwrap();
    }

    let mut counter = 0;
    for rx in rx.receivers.into_iter() {
        println!("counter {}", counter);
        let mut size = 0;
        while !rx.is_empty() {
            let _ = rx.recv().unwrap();
            // println!("{}", msg);
            size += 1;
        }
        println!("size {}", size);
        counter += 1;
    }
}
