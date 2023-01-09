mod position;

use position::Position;
use serde_json::Value;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::process::{Command, Output, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use tracing::{debug, info};

pub struct Host {
    position: Arc<Mutex<Position>>,
}

impl Host {
    pub fn new(device_path: String) -> Self {
        let position = Arc::new(Mutex::new(Position::default()));
        let _gpsd_output = Self::run_gpsd(device_path);
        Host { position }
    }

    pub fn run_gpsd(device: String) -> Output {
        Command::new("sudo")
            .arg("pkill")
            .arg("gpsd")
            .output()
            .expect("couldn't kill running gpsd-service");
        Command::new("sudo")
            .arg("systemctl")
            .arg("stop")
            .arg("gpsd.socket")
            .output()
            .expect("couldn't kill running gpsd-service");
        Command::new("sudo")
            .arg("gpsd")
            .arg("-n")
            // .arg("-D")
            // .arg("5")
            .arg(device)
            .stdin(Stdio::piped())
            // .stdout(Stdio::piped())
            // .stderr(Stdio::piped())
            .output()
            .expect("couldn't start gpsd")
    }

    pub fn run(self) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            // binary exponential backoff algorithm base for error case
            let bebo_base = 2_u64;
            let mut retries = 0_u32;
            let mut stream = loop {
                break match TcpStream::connect("127.0.0.1:2947") {
                    Ok(s) => s,
                    Err(_) => {
                        thread::sleep(Duration::from_millis(bebo_base.pow(retries)));
                        retries += 1;
                        debug!("Failed to connect to gpsd on port 2947. Retrying...");
                        continue;
                    }
                };
            };
            let mut reader: BufReader<TcpStream> = BufReader::new(
                stream
                    .try_clone()
                    .expect("Failed to create reader for gpsd on port 2947."),
            );
            info!("Successfully connected to server in port 2947");
            let msg = b"?WATCH={\"enable\":true,\"json\":true}";

            stream
                .write_all(msg)
                .expect("Couldn't write message to gpsd.");
            info!("Sent msg, awaiting reply...");
            loop {
                let mut buffer = String::new();
                let _ = reader.read_line(&mut buffer);
                let data =
                    serde_json::from_str::<Value>(&buffer).expect("failed parsing gps to json");
                if &data["class"] == "TPV" {
                    let mode = data["mode"].as_u64().unwrap();
                    if mode == 2_u64 || mode == 3_u64 {
                        let current_position = Position::new(
                            String::from(data["time"].as_str().unwrap_or("")),
                            data["lat"].as_f64().unwrap_or(0.0),
                            data["lon"].as_f64().unwrap_or(0.0),
                        );
                        debug!("Locking position.");
                        *self.position.lock().unwrap() = current_position;
                    }
                };
                // debug!("{}{}","Invalid position: ".green(), buffer);
            }
        })
    }
}

pub struct Phone {
    position: Arc<Mutex<Position>>,
}

impl Phone {
    pub fn new(device_path: String) -> Self {
        let position = Arc::new(Mutex::new(Position::default()));
        let _gpsd_output = Self::run_gpsd(device_path);
        Phone { position }
    }

    pub fn run_gpsd(device: String) -> Output {
        Command::new("sudo")
            .arg("pkill")
            .arg("gpsd")
            .output()
            .expect("couldn't kill running gpsd-service");
        Command::new("sudo")
            .arg("systemctl")
            .arg("stop")
            .arg("gpsd.socket")
            .output()
            .expect("couldn't kill running gpsd-service");
        Command::new("sudo")
            .arg("gpsd")
            // .arg("-D")
            // .arg("5")
            .arg(device)
            .stdin(Stdio::piped())
            // .stdout(Stdio::piped())
            // .stderr(Stdio::piped())
            .output()
            .expect("couldn't start gpsd")
    }

    pub fn run(self) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            // binary exponential backoff algorithm base for error case
            let bebo_base = 2_u64;
            let mut retries = 0_u32;
            let mut stream = loop {
                break match TcpStream::connect("127.0.0.1:2947") {
                    Ok(s) => s,
                    Err(_) => {
                        thread::sleep(Duration::from_millis(bebo_base.pow(retries)));
                        retries += 1;
                        debug!("Failed to connect to gpsd on port 2947. Retrying...");
                        continue;
                    }
                };
            };
            let mut reader: BufReader<TcpStream> = BufReader::new(
                stream
                    .try_clone()
                    .expect("Failed to create reader for gpsd on port 2947."),
            );
            info!("Successfully connected to server in port 2947");
            let msg = b"?WATCH={\"enable\":true,\"json\":true}";

            stream
                .write_all(msg)
                .expect("Couldn't write message to gpsd.");
            info!("Sent msg, awaiting reply...");
            loop {
                let mut buffer = String::new();
                let _ = reader.read_line(&mut buffer);
                let data =
                    serde_json::from_str::<Value>(&buffer).expect("failed parsing gps to json");
                if &data["class"] == "TPV" {
                    let mode = data["mode"].as_u64().unwrap();
                    if mode == 2_u64 || mode == 3_u64 {
                        let current_position = Position::new(
                            String::from(data["time"].as_str().unwrap_or("")),
                            data["lat"].as_f64().unwrap_or(0.0),
                            data["lon"].as_f64().unwrap_or(0.0),
                        );
                        debug!("Locking position.");
                        *self.position.lock().unwrap() = current_position;
                    }
                };
                debug!("Invalid position: {}", buffer);
            }
        })
    }
}
