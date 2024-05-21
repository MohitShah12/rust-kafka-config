use futures::StreamExt;
use rdkafka::{
    consumer::{StreamConsumer, Consumer},
    ClientConfig, Message
};

fn get_consumer(group:&str) -> StreamConsumer{
    let consumer:StreamConsumer = ClientConfig::new()
        .set("group.id", group)
        .set("bootstrap.servers", "welcome-mullet-8994-eu1-kafka.upstash.io:9092")
        .set("sasl.mechanism", "SCRAM-SHA-256")
        .set("security.protocol", "SASL_SSL")
        .set("sasl.username", "d2VsY29tZS1tdWxsZXQtODk5NCRzwpzqzIg5zdZl7re7eowvQuVzEUNyZJ7NSAw")
        .set("sasl.password", "ZGI4MmY4NDAtOWQ4Mi00Yzg4LTkwNjktODIwZDUyMTE1OWY2")
        // .set("auto.offset.reset", "earliest")
        // .set("auto.offset.reset", "largest")
        .create()
        .expect("Consumer creation error");
    consumer
}

async fn consume(consumer:StreamConsumer){
    while let Some(Ok(msg)) = consumer.stream().next().await{
        if let Some(Ok(data)) = msg.payload_view::<str>(){
            println!("data: {:?}", data);
        }
    }
}

#[tokio::main]
async fn main() {
    let subscribe = true;
    // let subscribe = false;
    let topic = "rust-kafka-demo";

    // let group = "g1";
    // let group = "g2";
    let group = "g3";
    let consumer = get_consumer(group);

    if subscribe{
        consumer.subscribe(&[topic]).unwrap();
        tokio::join!(consume(consumer));
    }else {
        consumer.unsubscribe();
    }

}