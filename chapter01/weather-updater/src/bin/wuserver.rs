use rand::Rng;
use zmq::{Context, SocketType};

fn main() {
    let ctx = Context::new();

    let publisher = ctx.socket(SocketType::PUB).unwrap();

    publisher.bind("tcp://*:5556").unwrap();

    let mut r = rand::thread_rng();

    loop {
        let zipcode = r.gen_range(0..100000);
        let temperature = r.gen_range(-80..135);
        let rel_humidity = r.gen_range(10..60);

        let update = format!("{zipcode:05} {temperature} {rel_humidity}");

        publisher.send(update.as_bytes(), 0).unwrap();
    }
}
