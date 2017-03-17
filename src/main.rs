use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:4551").unwrap();

    // ignore the Result
    let _ = stream.write_all(b"my test string");
    let mut request = String::new();
    stream.read_to_string(&mut request).ok();
    println!("{}", request);
    //    let _ = stream.read(&mut [0; 128]); // ignore here too
}
