use crate::{NetReceiver, NetSender};
use std::{
    io::{ErrorKind, Read, Write},
    net::{TcpListener, TcpStream, UdpSocket},
    sync::mpsc,
    thread,
    time::Duration,
};

pub struct TCPSender;

impl NetSender for TCPSender {
    fn name() -> &'static str {
        "Unicast TCP"
    }

    fn send(addr: &str, msg: &str) {
        TcpStream::connect(addr)
            .unwrap()
            .write_all(msg.as_bytes())
            .unwrap();
    }
}

pub struct TCPReceiver {
    listener: TcpListener,
}

impl NetReceiver for TCPReceiver {
    fn new(addr: &str) -> Self {
        let listener = TcpListener::bind(addr).unwrap();
        listener.set_nonblocking(true).unwrap();
        Self { listener }
    }

    fn name() -> &'static str {
        "Unicast TCP"
    }

    fn addr(&self) -> String {
        self.listener.local_addr().unwrap().to_string()
    }

    fn receive(&self, rx: mpsc::Receiver<()>) {
        let mut threads = Vec::new();
        for stream in self.listener.incoming() {
            if let Ok(_) = rx.try_recv() {
                break;
            }
            match stream {
                Ok(mut stream) => threads.push(thread::spawn(move || {
                    let mut buf = [0; 1024];
                    let addr = stream.peer_addr().unwrap();
                    match stream.read(&mut buf) {
                        Ok(size) => {
                            println!(
                                "Receive message from {addr}: {}",
                                String::from_utf8_lossy(&buf[..size])
                            );
                        }
                        Err(err) if err.kind() == ErrorKind::WouldBlock => {
                            eprintln!("Receive empty from {addr}")
                        }
                        Err(err) => {
                            eprintln!("Failed to read from connection: {err}");
                        }
                    }
                })),
                Err(err) if err.kind() == ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(100))
                }
                Err(err) => eprintln!("Failed to accept connection: {err}"),
            }
        }
        for thread in threads {
            if let Err(err) = thread.join() {
                eprintln!("Failed to join thread: {err:#?}");
            }
        }
    }
}

pub struct UDPSender;

impl NetSender for UDPSender {
    fn name() -> &'static str {
        "Unicast UDP"
    }

    fn send(addr: &str, msg: &str) {
        UdpSocket::bind("0.0.0.0:0")
            .unwrap()
            .send_to(msg.as_bytes(), addr)
            .unwrap();
    }
}

pub struct UDPReceiver {
    socket: UdpSocket,
}

impl NetReceiver for UDPReceiver {
    fn new(addr: &str) -> Self {
        let socket = UdpSocket::bind(addr).unwrap();
        socket
            .set_read_timeout(Some(Duration::from_millis(100)))
            .unwrap();
        Self { socket }
    }

    fn name() -> &'static str {
        "Unicast UDP"
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
