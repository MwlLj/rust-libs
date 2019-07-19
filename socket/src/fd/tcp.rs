use std::net::TcpStream;
use std::net::TcpListener;

#[cfg(target_os="linux")]
    use std::os::unix::io::AsRawFd;

#[cfg(target_os="windows")]
use std::os::windows::io::AsRawSocket;

#[cfg(target_os="linux")]
pub fn stream2fd(stream: TcpStream) -> u64 {
    TcpStream::as_raw_fd(&stream) as u64
}

#[cfg(target_os="windows")]
pub fn stream2fd(stream: TcpStream) -> u64 {
    TcpStream::as_raw_socket(&stream)
}

#[test]
fn stream2fdTest() {
    let listener = match TcpListener::bind("0.0.0.0:12345") {
        Ok(l) => l,
        Err(err) => {
            assert!(false);
            return;
        }
    };
    let stream = listener.incoming().next().unwrap();
    assert!(stream2fd(stream.unwrap().try_clone().unwrap()) > 0);
}
