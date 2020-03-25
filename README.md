Run
```
./download-kafka.sh
```

To build the rust clients run (requires rust to be installed)
```
./build-rust-clients.sh
```

Run this to create the topic the rust clients use
(this must be run before starting the rust clients)
```
./kafka_2.12-2.4.1/bin/kafka-topics.sh --create --topic test --bootstrap-server localhost:9092
```

To start a cluster of 1 broker run
```
./run-cluster-of-1.sh
```

To run the rust producer
```
./rust-clients/kafka-producer/target/debug/kafka-producer
```

To run the rust consumer
```
./rust-clients/kafka-consumer/target/debug/kafka-consumer
```
