use std::net::TcpStream;
use std::net::TcpListener;

#[cfg(target_os="linux")]
use std::os::unix::io::AsRawFd;
#[cfg(target_os="linux")]
use std::os::unix::io::IntoRawFd;
#[cfg(target_os="linux")]
use std::os::unix::io::FromRawFd;

#[cfg(target_os="windows")]
use std::os::windows::io::AsRawSocket;
#[cfg(target_os="windows")]
use std::os::windows::io::IntoRawSocket;
#[cfg(target_os="windows")]
use std::os::windows::io::FromRawSocket;

#[cfg(target_os="linux")]
pub fn stream2fd(stream: TcpStream) -> u64 {
    TcpStream::into_raw_fd(stream) as u64
}

#[cfg(target_os="linux")]
pub fn fd2stream(fd: u64) -> TcpStream {
    unsafe {
        TcpStream::from_raw_fd(fd as i32)
    }
}

#[cfg(target_os="windows")]
pub fn stream2fd(stream: TcpStream) -> u64 {
    TcpStream::into_raw_socket(stream)
}

#[cfg(target_os="windows")]
pub fn fd2stream(fd: u64) -> TcpStream {
    unsafe {
        TcpStream::from_raw_socket(fd)
    }
}

#[test]
fn stream2fdTest() {
    let listener = match TcpListener::bind("0.0.0.0:1234") {
        Ok(l) => l,
        Err(err) => {
            assert!(false);
            return;
        }
    };
    let stream = listener.incoming().next().unwrap();
    assert!(stream2fd(stream.unwrap().try_clone().unwrap()) > 0);
}
