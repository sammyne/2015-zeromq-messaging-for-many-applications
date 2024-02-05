use std::time::Duration;

use zmq::{Context, SocketType};

fn main() {
    let ctx = Context::new();

    let responder = ctx.socket(SocketType::REP).unwrap();
    responder.bind("tcp://*:5555").unwrap();

    loop {
        let msg = responder.recv_bytes(0).unwrap();
        let msg = std::str::from_utf8(msg.as_slice()).unwrap();
        println!("Received {msg}");

        std::thread::sleep(Duration::from_secs(1));

        let reply = b"World";
        responder.send(reply.as_ref(), 0).unwrap();
    }
}
