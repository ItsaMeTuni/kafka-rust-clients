
use kafka::consumer::{Consumer, FetchOffset};

fn main()
{
    println!("Initializing consumer...");

    let mut consumer = Consumer::from_hosts(vec!("localhost:9092".to_owned()))
        .with_topic("test".to_owned())
        .with_fallback_offset(FetchOffset::Earliest)
        .create()
        .unwrap();

    loop
    {
        for messages in consumer.poll().unwrap().iter()
        {
            for msg in messages.messages()
            {
                println!("Received message: {:?}", String::from_utf8(Vec::from(msg.value)).unwrap() );
            }

            consumer.consume_messageset(messages).unwrap();
        }

        consumer.commit_consumed().unwrap();
    }
}
