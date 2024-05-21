# Basic configuration of Kafka in Rust using upstash.

## Dependecies
```
tokio = { version = "1.21.2" , features = ["rt-multi-thread", "macros"]}
futures = { version = "0.3.24", features = ["async-await"], default-features = false}
rdkafka = { git = "https://github.com/implrust/rust-rdkafka", tag = "v0.29.0-sasl", features = ["cmake-build","ssl","sasl"]}
```

## To run producer
```console
cargo run --bin producer
```

## To run consumer
```console
cargo run --bin consumer
```
