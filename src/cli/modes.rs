use clap::{Subcommand, ValueEnum};
use std::fmt;
use std::net::IpAddr;
use std::path::PathBuf;
use std::str::FromStr;

const DEFAULT_TARGET_IP: &str = "127.0.0.1";
const DEFAULT_SENDER_PORT: u16 = 1234;
const DEFAULT_RECEIVER_PORT: u16 = 4321;
const DEFAULT_DATA_RATE: f64 = 1.0; // in mbps
const DEFAULT_PACKET_SIZE: usize = 1500; // in bytes
const DEFAULT_SLEEP_ADJUST: u32 = 10;

#[derive(Debug, Clone, PartialEq, PartialOrd, Subcommand)]
pub enum RunMode {
    /// Run as the Sender.
    Sender {
        /// port to bind to for sending data
        #[arg(short = 'p', long, default_value_t = DEFAULT_SENDER_PORT)]
        port: u16,

        /// target IP address to send data to
        #[arg(short = 'T', long, default_value_t = IpAddr::from_str(DEFAULT_TARGET_IP).unwrap())]
        target_ip: IpAddr,

        /// target port to send data to
        #[arg(short = 'P', long, default_value_t = DEFAULT_RECEIVER_PORT)]
        target_port: u16,

        /// data rate in mbps
        #[arg(short = 'b', long, default_value_t = DEFAULT_DATA_RATE)]
        data_rate: f64,

        /// packet size in bytes
        #[arg(short = 's', long, default_value_t = DEFAULT_PACKET_SIZE)]
        packet_size: usize,

        /// adjust the sleep by this factor
        #[arg(short = 'f', long, default_value_t = DEFAULT_SLEEP_ADJUST)]
        sleep_adjust: u32,

        /// file to write output to
        #[arg(short = 'o', long)]
        output_file: Option<PathBuf>,

        /// set the method by which the GPS information is provided
        #[arg(short, long, default_value_t = GpsMode::Phone)]
        gps_mode: GpsMode,

        /// Path to the GPS device
        #[arg(short = 'd', long, default_value = "/dev/USB0")]
        gps_device: String,
    },
    /// Run as the Receiver.
    Receiver {
        /// port to bind to for receiving data
        #[arg(short, long, default_value_t = DEFAULT_RECEIVER_PORT)]
        port: u16,

        /// file to write output to
        #[arg(short, long)]
        output_file: Option<PathBuf>,
    },
}

impl fmt::Display for RunMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            RunMode::Sender {
                port: _,
                target_ip,
                target_port,
                data_rate,
                packet_size,
                sleep_adjust,
                gps_mode,
                gps_device,
                output_file: _,
            } => write!(
                f,
                "Sender({}) -> {}:{}@{}/{}\nGPS: {} {}",
                sleep_adjust, target_ip, target_port, data_rate, packet_size, gps_mode, gps_device
            ),
            RunMode::Receiver {
                port,
                output_file: _,
            } => write!(f, "Receiver ({})", port),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum GpsMode {
    /// Run in a mode that takes GPS information from a GPS receiver attached to the host.
    Host,
    /// Run in a mode that takes GPS information from a phone that forwards the GPS information to the host.
    Phone,
}

impl fmt::Display for GpsMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            GpsMode::Host => write!(f, "host"),
            GpsMode::Phone => write!(f, "phone"),
        }
    }
}
