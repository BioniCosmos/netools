use clap::{Parser, Subcommand, ValueEnum};
use netools::{NetReceiver, NetSender, broadcast, multicast, unicast};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Send a unicast message
    #[command(name = "ucast-send")]
    UnicastSend {
        /// Server address to connect to
        #[arg(short, long, default_value = "127.0.0.1:8080")]
        addr: String,

        /// Message to send
        #[arg(short, long)]
        msg: String,

        /// Protocol to use
        #[arg(short, long)]
        proto: Protocol,
    },

    /// Receive unicast messages
    #[command(name = "ucast-recv")]
    UnicastReceive {
        /// Server address to bind to
        #[arg(short, long, default_value = "127.0.0.1:8080")]
        addr: String,

        /// Protocol to use
        #[arg(short, long)]
        proto: Protocol,
    },

    /// Send a multicast message
    #[command(name = "mcast-send")]
    MulticastSend {
        /// Multicast group address
        #[arg(short, long, default_value = "224.0.0.1:8080")]
        addr: String,

        /// Message to send
        #[arg(short, long)]
        msg: String,
    },

    /// Receive multicast messages
    #[command(name = "mcast-recv")]
    MulticastReceive {
        /// Multicast group address
        #[arg(short, long, default_value = "224.0.0.1:8080")]
        addr: String,
    },

    /// Send a broadcast message
    #[command(name = "bcast-send")]
    BroadcastSend {
        /// Broadcast address
        #[arg(short, long, default_value = "255.255.255.255:8080")]
        addr: String,

        /// Message to send
        #[arg(short, long)]
        msg: String,
    },

    /// Receive broadcast messages
    #[command(name = "bcast-recv")]
    BroadcastReceive {
        /// Port to bind to
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },
}

#[derive(Clone, ValueEnum)]
enum Protocol {
    TCP,
    UDP,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::UnicastSend { addr, msg, proto } => match proto {
            Protocol::TCP => unicast::TCPSender::run(addr, msg),
            Protocol::UDP => unicast::UDPSender::run(addr, msg),
        },
        Commands::UnicastReceive { addr, proto } => match proto {
            Protocol::TCP => unicast::TCPReceiver::new(addr).run(),
            Protocol::UDP => unicast::UDPReceiver::new(addr).run(),
        },
        Commands::MulticastSend { addr, msg } => multicast::Sender::run(addr, msg),
        Commands::MulticastReceive { addr } => multicast::Receiver::new(addr).run(),
        Commands::BroadcastSend { addr, msg } => broadcast::Sender::run(addr, msg),
        Commands::BroadcastReceive { port } => broadcast::Receiver::new(*port).run(),
    }
}
