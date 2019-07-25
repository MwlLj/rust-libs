use socket::fd::tcp;

use std::net::TcpListener;
use std::net::TcpStream;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::prelude::*;
use std::thread;
use std::time;

fn stream2fdTest() {
    let listener = match TcpListener::bind("0.0.0.0:1234") {
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
	        fd = tcp::stream2fd(stream.try_clone().unwrap());
	        break;
	    }
	    let mut stream = tcp::fd2stream(fd);
	    stream.write("hello 2".as_bytes());
	    // let mut writer = BufWriter::new(&stream);
	    // writer.write_all("hello 2".as_bytes()).expect("write all error");
	    // writer.flush().expect("flush error");
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

fn fd2streamTakeerrorTest() {
    let listener = match TcpListener::bind("0.0.0.0:1234") {
        Ok(l) => l,
        Err(err) => {
            assert!(false);
            return;
        }
    };
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let fd = tcp::stream2fd(stream.try_clone().unwrap());
        println!("fd: {}", fd);
        match stream.take_error() {
            Ok(_) => {
                println!("success");
            },
            Err(err) => {
                println!("err: {}", err);
            }
        }
    }
    let stream = tcp::fd2stream(1);
    match stream.take_error() {
        Ok(_) => {
        },
        Err(err) => {
            println!("err: {}", err);
        }
    }
}

fn main() {
    // stream2fdTest();
    // fd2streamTest();
    fd2streamTakeerrorTest();
}
