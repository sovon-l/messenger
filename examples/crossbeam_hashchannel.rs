pub fn main() {
    let (tx, rx) = messenger::hash_channel::new_hash_channel::<
        String,
        messenger::hash_channel::crossbeam::CrossbeamChannels<String>,
    >(8);

    let msgs: Vec<String> = vec![
        "hello".to_string(),
        "world".to_string(),
        "rust".to_string(),
        "is".to_string(),
        "awesome".to_string(),
    ];

    for msg in msgs.into_iter() {
        tx.send(msg).unwrap();
    }

    let mut counter = 0;
    for rx in rx.receivers.into_iter() {
        println!("counter {}", counter);
        while !rx.is_empty() {
            let msg = rx.recv().unwrap();
            println!("{}", msg);
        }
        counter += 1;
    }
}
