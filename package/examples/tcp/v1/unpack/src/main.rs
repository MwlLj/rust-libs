use package::tcp::v1::{unpack, Body};
use package::tcp::v1::unpack::ResultCode;

use std::net::TcpListener;

fn main() {
    let listener = match TcpListener::bind("0.0.0.0:50000") {
        Ok(l) => l,
        Err(err) => {
            println!("tcp bind error, err: {}", err);
            return;
        }
    };
    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(err) => {
                println!("stream error");
                continue;
            }
        };
        let stream = match stream.try_clone() {
            Ok(s) => s,
            Err(err) => {
                println!("try clone error, err: {}", err);
                continue;
            }
        };
        unpack::listen(stream, &mut |body: &mut Body, code: ResultCode| -> bool {
            println!("body: {:?}, code: {:?}", &body, code);
            true
        }, true);
    }
}
