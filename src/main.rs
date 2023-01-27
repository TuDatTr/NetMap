mod cli;
mod clients;
mod gps;

use clap::Parser;
use cli::{modes::GpsMode, modes::RunMode, Cli};
use clients::{Receiver, Sender};
use gps::{Host, Phone};
use std::net::{SocketAddr, UdpSocket};
use tracing_subscriber::{fmt, EnvFilter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = fmt::Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting tracing default failed");

    let cli = Cli::parse();
    let client_handler = match cli.mode {
        RunMode::Sender {
            port,
            target_ip,
            target_port,
            packet_count,
            packet_size,
            gps_mode,
            gps_device,
            output_file: _,
        } => {
            let _gps_handler = match gps_mode {
                GpsMode::Host => {
                    let host = Host::new(gps_device);
                    host.run()
                }
                GpsMode::Phone => {
                    let phone = Phone::new(gps_device);
                    phone.run()
                }
            };
            // Bind to a socket address.
            let recv_socket = UdpSocket::bind(format!("0.0.0.0:{}", port)).unwrap();
            recv_socket.set_nonblocking(true)?;

            // Create the target socket address.
            let target_address = SocketAddr::new(target_ip, target_port);

            let sender = Sender::new(recv_socket, target_address);
            sender.run(packet_count, packet_size)
        }
        RunMode::Receiver {
            port,
            output_file: _,
        } => {
            let receiver = Receiver::new(port);
            receiver.run()
        }
    };

    let handlers = vec![client_handler];

    for thread in handlers {
        thread.join().unwrap();
    }
    Ok(())
}
