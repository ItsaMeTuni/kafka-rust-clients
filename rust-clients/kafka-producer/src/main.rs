
use std::thread;
use std::time::Duration;
use kafka::producer::{Producer, Record, RequiredAcks};

fn main()
{
    println!("Already sending messages.");

    let mut producer = 
        Producer::from_hosts(vec!("localhost:9092".to_owned()))
        .with_ack_timeout(Duration::from_secs(5))
        .with_required_acks(RequiredAcks::One)
        .create()
        .unwrap();

    let mut counter = 0;

    loop
    {
        counter += 1;
        let result = producer.send(&Record::from_value("test", format!("Hello from Rust producer! {:?}", counter).as_bytes()));

        if result.is_err()
        {
            eprintln!("{:?}", result.unwrap_err());
            return;
        }

        thread::sleep(Duration::from_millis(500));
    }
}
