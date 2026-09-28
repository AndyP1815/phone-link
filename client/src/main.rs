use std::io::Read;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("192.168.122.254:7878").unwrap();

    println!("Connected to phone server!");

    let mut buffer = [0; 1024];

    let bytes_read = stream.read(&mut buffer).unwrap();

    let message = String::from_utf8_lossy(&buffer[..bytes_read]);

    println!("Phone says: {}", message);
}