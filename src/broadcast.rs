use crate::{NetReceiver, NetSender};
use std::{
    io::ErrorKind,
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
    sync::mpsc,
    time::Duration,
};

pub struct Sender;

impl NetSender for Sender {
    fn name() -> &'static str {
        "Broadcast"
    }

    fn send(addr: &str, msg: &str) {
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        socket.set_broadcast(true).unwrap();
        socket.send_to(msg.as_bytes(), addr).unwrap();
    }
}

pub struct Receiver {
    socket: UdpSocket,
}

impl Receiver {
    pub fn new(port: u16) -> Self {
        let socket =
            UdpSocket::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port)).unwrap();
        socket
            .set_read_timeout(Some(Duration::from_millis(100)))
            .unwrap();
        Self { socket }
    }
}

impl NetReceiver for Receiver {
    fn new(_: &str) -> Self {
        unimplemented!()
    }

    fn name() -> &'static str {
        "Broadcast"
    }

    fn addr(&self) -> String {
        self.socket.local_addr().unwrap().to_string()
    }

    fn receive(&self, rx: mpsc::Receiver<()>) {
        let mut buf = [0; 1024];
        loop {
            if rx.try_recv().is_ok() {
                break;
            }
            match self.socket.recv_from(&mut buf) {
                Ok((size, addr)) => println!(
                    "Receive message from {addr}: {}",
                    String::from_utf8_lossy(&buf[..size])
                ),
                Err(err)
                    if err.kind() == ErrorKind::WouldBlock || err.kind() == ErrorKind::TimedOut => {
                }
                Err(err) => eprintln!("Failed to receive data: {err}"),
            }
        }
    }
}
