pub fn main() {
    let (tx, rx) = messenger::hash_channel::new(4);

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
}