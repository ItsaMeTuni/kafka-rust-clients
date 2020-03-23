
use std::time::Duration;
use kafka::producer::{Producer, Record, RequiredAcks};

fn main()
{
    println!("Initializing producer...");

    let mut producer = 
        Producer::from_hosts(vec!("localhost:9092".to_owned()))
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .unwrap();

    producer.send(&Record::from_value("test", "Hello from Rust producer!".as_bytes())).unwrap();

    println!("Message sent successfully!");
}
