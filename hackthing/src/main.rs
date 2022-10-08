use std::borrow::Borrow;
use nix::sys::socket::{AddressFamily, connect, socket, SockFlag, SockType, UnixAddr};

#[cfg(target_os = "linux")]
fn main() {
    let addr = UnixAddr::new("/home/serena/Code/ladyserena/aurae-collector/meow.sock").unwrap();
    let my_sock = socket(AddressFamily::Unix, SockType::Stream, SockFlag::empty(), None).unwrap();
    connect(my_sock,addr.borrow()).unwrap();
    // TODO fork and exec the client binary and

}
