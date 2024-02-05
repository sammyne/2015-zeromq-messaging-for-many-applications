use std::env;

use zmq::{Context, SocketType};

fn atoi(s: &str) -> i64 {
    s.parse().unwrap()
}

fn main() {
    println!("Collecting updates from weather server...");

    let context = Context::new();
    let subscriber = context.socket(SocketType::SUB).unwrap();
    assert!(subscriber.connect("tcp://localhost:5556").is_ok());

    let filter = env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| "10001".to_string());
    assert!(subscriber.set_subscribe(filter.as_bytes()).is_ok());

    let mut total_temp = 0;

    for _ in 0..10 {
        let string = subscriber.recv_string(0).unwrap().unwrap();
        let chks: Vec<i64> = string.split(' ').map(atoi).collect();
        let (_zipcode, temperature, _relhumidity) = (chks[0], chks[1], chks[2]);

        total_temp += temperature;
    }

    println!(
        "Average temperature for zipcode {} was {}F",
        filter,
        total_temp / 100
    );
}
