use zmq::{Context, SocketType};

fn main() {
  let ctx = Context::new();

  println!("Connecting to hello world server...");

  let requester = ctx.socket(SocketType::REQ).unwrap();
  requester.connect("tcp://localhost:5555").unwrap();

  for i in 0..10 {
    let request = "Hello";
    println!("Sending {request} {i}...");
    requester.send(request.as_bytes(), 0).unwrap();

    let reply = requester.recv_bytes(0).unwrap();
    let reply = std::str::from_utf8(&reply).unwrap();
    println!("Received {reply} {i}");
  }
}