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
        "Multicast"
    }

    fn send(addr: &str, msg: &str) {
        UdpSocket::bind("0.0.0.0:0")
            .unwrap()
            .send_to(msg.as_bytes(), addr)
            .unwrap();
    }
}

pub struct Receiver {
    socket: UdpSocket,
}

impl NetReceiver for Receiver {
    fn new(addr: &str) -> Self {
        let socket_addr = addr.parse::<SocketAddr>().unwrap();
        let socket = UdpSocket::bind(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            socket_addr.port(),
        ))
        .unwrap();
        let IpAddr::V4(addr) = socket_addr.ip() else {
            panic!("IPv4 only")
        };
        socket
            .join_multicast_v4(&addr, &Ipv4Addr::UNSPECIFIED)
            .unwrap();
        socket
            .set_read_timeout(Some(Duration::from_millis(100)))
            .unwrap();
        println!("Multicast group address: {addr}");
        Self { socket }
    }

    fn name() -> &'static str {
        "Multicast"
    }

    fn addr(&self) -> String {
        self.socket.local_addr().unwrap().to_string()
    }

    fn receive(&self, rx: mpsc::Receiver<()>) {
        let mut buf = [0; 1024];
        loop {
            if let Ok(_) = rx.try_recv() {
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
