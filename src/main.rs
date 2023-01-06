mod cli;
mod clients;

use clap::Parser;
use cli::{modes::RunMode, Cli};
use clients::{Receiver, Sender};
use std::net::{SocketAddr, UdpSocket};
use tracing_subscriber::{fmt, EnvFilter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = fmt::Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting tracing default failed");

    let cli = Cli::parse();
    let handler = match cli.mode {
        RunMode::Sender {
            port,
            target_ip,
            target_port,
            data_rate,
            packet_size,
            sleep_adjust,
            output_file: _,
        } => {
            // Bind to a socket address.
            let recv_socket = UdpSocket::bind(format!("127.0.0.1:{}", port)).unwrap();
            recv_socket.set_nonblocking(true)?;

            // Create the target socket address.
            let target_address = SocketAddr::new(target_ip, target_port);

            let sender = Sender::new(recv_socket, target_address);
            sender.run(data_rate, packet_size, sleep_adjust)
        }
        RunMode::Receiver {
            port,
            output_file: _,
        } => {
            let receiver = Receiver::new(port);
            receiver.run()
        }
    };
    handler.join().unwrap();
    Ok(())
}
