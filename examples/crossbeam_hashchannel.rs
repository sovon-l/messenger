static MSGS: [&'static str; 63] = [
    "Lorem",
    "ipsum",
    "dolor",
    "sit",
    "amet",
    "consectetur",
    "adipiscing",
    "elit",
    "sed",
    "doneiusmod",
    "tempor",
    "incididunt",
    "ut",
    "labore",
    "et",
    "dolore",
    "magna",
    "aliqua",
    "Ut",
    "enim",
    "adnminim",
    "veniam",
    "quis",
    "nostrud",
    "exercitation",
    "ullamco",
    "laboris",
    "nisi",
    "utnaliquip",
    "ex",
    "ea",
    "commodo",
    "consequat",
    "Duis",
    "aute",
    "irure",
    "dolor",
    "innreprehenderit",
    "in",
    "voluptate",
    "velit",
    "esse",
    "cillum",
    "dolore",
    "eu",
    "fugiat",
    "nullanpariatur",
    "Excepteur",
    "sint",
    "occaecat",
    "cupidatat",
    "non",
    "proident",
    "sunt",
    "innculpa",
    "qui",
    "officia",
    "deserunt",
    "mollit",
    "anim",
    "id",
    "est",
    "laborum",
];

pub fn main() {
    let (mut tx, rx) = messenger::hash_channel::new_hash_channel::<
        String,
        messenger::externals::crossbeam::CrossbeamChannels<_>,
    >(8);

    let msgs: Vec<String> = MSGS.iter().map(|s| s.to_string()).collect();

    for msg in msgs.into_iter() {
        tx.send(msg).unwrap();
    }

    let mut sizes = vec![];
    for rx in rx.receivers.into_iter() {
        let mut size = 0;
        while !rx.is_empty() {
            let _ = rx.recv().unwrap();
            size += 1;
        }
        sizes.push(size);
    }
    print!("container: ");
    for (k, _) in sizes.iter().enumerate() {
        print!("{:3} ", k);
    }
    println!();
    print!("sizes:     ");
    for s in sizes.iter() {
        print!("{:3} ", s);
    }
    println!();
}
