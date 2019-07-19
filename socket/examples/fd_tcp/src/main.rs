use socket::fd::tcp;

use std::net::TcpListener;

fn stream2fdTest() {
    let listener = match TcpListener::bind("0.0.0.0:12345") {
        Ok(l) => l,
        Err(err) => {
            assert!(false);
            return;
        }
    };
    for stream in listener.incoming() {
        println!("{}", tcp::stream2fd(stream.unwrap().try_clone().unwrap()));
    }
}

fn main() {
    stream2fdTest();
}
