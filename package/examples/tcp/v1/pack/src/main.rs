use package::tcp::v1::{pack, Body};

use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let mut stream = match TcpStream::connect("127.0.0.1:50000") {
        Ok(s) => s,
        Err(err) => {
            println!("connect error, err: {}", err);
            return;
        }
    };
    let body = pack::pack(&mut Body{
        protoNo: Vec::from("v1.0"),
        messageId: Vec::from("123456"),
        param: Vec::from("{\"name\": \"jake\"}"),
        extra: Vec::from("success"),
        data: Vec::from("hello")
    });
    println!("{:?}", &body);
    stream.write_all(body.as_slice());
    stream.flush();
}
