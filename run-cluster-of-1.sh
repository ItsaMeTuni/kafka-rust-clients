x-terminal-emulator -e kafka_2.12-2.4.1/bin/zookeeper-server-start.sh kafka_2.12-2.4.1/config/zookeeper.properties
sleep 2
x-terminal-emulator -e kafka_2.12-2.4.1/bin/kafka-server-start.sh kafka_2.12-2.4.1/config/server.properties
