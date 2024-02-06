use std::io::{self, Write};
use std::{thread, time};

use zmq::{Context, SocketType};

fn atoi(s: &str) -> i64 {
    s.parse().unwrap()
}

fn main() {
    let context = Context::new();
    let receiver = context.socket(SocketType::PULL).unwrap();
    assert!(receiver.connect("tcp://localhost:5557").is_ok());

    let sender = context.socket(SocketType::PUSH).unwrap();
    assert!(sender.connect("tcp://localhost:5558").is_ok());

    loop {
        let string = receiver.recv_string(0).unwrap().unwrap();
        println!("{}.", string);
        let _ = io::stdout().flush();
        thread::sleep(time::Duration::from_millis(atoi(&string) as u64));
        sender.send("", 0).unwrap();
    }
}
