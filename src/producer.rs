use std::io;

use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    ClientConfig
};

fn get_producer() -> FutureProducer{
    let producer:FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "welcome-mullet-8994-eu1-kafka.upstash.io:9092")
        .set("sasl.mechanism", "SCRAM-SHA-256")
        .set("security.protocol", "SASL_SSL")
        .set("sasl.username", "d2VsY29tZS1tdWxsZXQtODk5NCRzwpzqzIg5zdZl7re7eowvQuVzEUNyZJ7NSAw")
        .set("sasl.password", "ZGI4MmY4NDAtOWQ4Mi00Yzg4LTkwNjktODIwZDUyMTE1OWY2")
        .create()
        .expect("Producer creation error");
    producer
}

async fn produce(producer:FutureProducer, input:String){
    let data = FutureRecord::to("rust-kafka-demo").payload(&input[..]).key("data");
    producer.send(data, None).await.expect("data produce error");
}

#[tokio::main]
async fn main() {
    let producer = get_producer();
    let mut input = String::new();

loop {
    println!("Enter input: ");
    io::stdin().read_line(&mut input).expect("input error");
    input = input.trim().to_string();
    
    if input != "."{
        tokio::join!(produce(producer.clone(), input.clone()));
        input.clear();
    }else {
        break;
    }
}


}
