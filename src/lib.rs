pub mod broadcast;
pub mod multicast;
pub mod unicast;

use std::sync::mpsc;

pub trait NetSender {
    fn run(addr: &str, msg: &str) {
        println!("{} sender started", Self::name());
        Self::send(addr, msg);
    }

    fn name() -> &'static str;
    fn send(addr: &str, msg: &str);
}

pub trait NetReceiver {
    fn run(&self) {
        println!("{} receiver started on {}", Self::name(), self.addr());
        let (tx, rx) = mpsc::channel();
        ctrlc::set_handler(move || {
            println!("stopping");
            tx.send(()).unwrap();
        })
        .unwrap();
        self.receive(rx);
    }

    fn new(addr: &str) -> Self;
    fn name() -> &'static str;
    fn addr(&self) -> String;
    fn receive(&self, rx: mpsc::Receiver<()>);
}
