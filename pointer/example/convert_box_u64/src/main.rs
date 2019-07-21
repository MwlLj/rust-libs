use pointer::convert::box_u64;

use std::net::TcpListener;
use std::net::TcpStream;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::prelude::*;
use std::thread;
use std::time;

fn box2u64Test() {
    let v = Box::new(1);
    let addr = box_u64::box2u64(v);
    println!("addr: {}", addr);
}

fn u642boxTest() {
    let v = Box::new(1);
    let addr = box_u64::box2u64(v);
    let result = box_u64::u642box::<u32>(addr);
    println!("{:?}", result);
}

fn fd2streamTest() {
	thread::spawn(|| {
		let mut fd: u64 = 0;
	    let listener = match TcpListener::bind("0.0.0.0:1234") {
	        Ok(l) => l,
	        Err(err) => {
	            assert!(false);
	            return;
	        }
	    };
	    for stream in listener.incoming() {
	    	let stream = stream.unwrap();
		    // let mut writer = BufWriter::new(&stream);
		    // writer.write_all("hello 1".as_bytes()).expect("write all error");
		    // writer.flush().expect("flush error");
	    	println!("new connect");
	        fd = box_u64::box2u64(Box::new(stream.try_clone().unwrap()));
	        break;
	    }
	    let stream = box_u64::u642box::<TcpStream>(fd);
	    let mut writer = BufWriter::new(&*stream);
	    writer.write_all("hello 2".as_bytes()).expect("write all error");
	    writer.flush().expect("flush error");
	});
	thread::sleep(time::Duration::from_secs(1));
	let mut stream = match TcpStream::connect("127.0.0.1:1234") {
		Ok(s) => s,
		Err(err) => {
			println!("connect error, err: {}", err);
			return;
		}
	};
	let mut buf = [0; 10];
	let n = stream.read(&mut buf).expect("read error");
	println!("{:?}", &buf[..n]);
}

fn main() {
    // box2u64Test();
    // u642boxTest();
    fd2streamTest();
}
