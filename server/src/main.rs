use std::io::Write;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

    println!("Phone server listening on port 7878");

    for mut stream in listener.incoming() {
        let mut stream = stream.unwrap();

        println!("Client connected!");

        stream.write_all(b"HELLO FROM PHONE\n").unwrap();
    }
}