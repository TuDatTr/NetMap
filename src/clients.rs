use std::net::{SocketAddr, UdpSocket};
use std::thread;
use std::time::{Duration, Instant};

use tracing::{debug, info, trace};

pub struct Sender {
    recv_socket: UdpSocket,
    target_address: SocketAddr,
}

impl Sender {
    pub fn new(recv_socket: UdpSocket, target_address: SocketAddr) -> Self {
        Self {
            recv_socket,
            target_address,
        }
    }

    pub fn run(self, pkt_count: u32, packet_size: u16) -> thread::JoinHandle<()> {
        // const BUF_SIZE: usize =  65536;
        const BUF_SIZE: usize = 9000;
        let mut buf = [0; BUF_SIZE]; // Increase the buffer size to hold at least 9000 bytes

        const UDP_HEADER_SIZE: u16 = 42;
        let payload_size: usize = (packet_size - UDP_HEADER_SIZE).into(); // size of packets to send in bytes

        let payload: Vec<u8> = vec![0; payload_size]; // create a vector of 0s with the specified packet size

        thread::spawn(move || {
            for _ in 0..pkt_count {
                let _ = &self
                    .recv_socket
                    .send_to(&payload, self.target_address)
                    .unwrap(); // send the data
            }

            match &self.recv_socket.recv_from(&mut buf) {
                Ok((size, src)) => {
                    trace!("received {} bytes from {:?}", size, src); // Use logging with tracing
                }
                Err(_e) => {}
            }
        })
    }
}

pub struct Receiver {
    socket: UdpSocket,
}

impl Receiver {
    pub fn new(port: u16) -> Self {
        let socket = UdpSocket::bind(format!("127.0.0.1:{}", port)).unwrap();
        socket.set_nonblocking(true).unwrap();
        Self { socket }
    }

    pub fn run(self) -> thread::JoinHandle<()> {
        let mut bytes_received = 0; // counter variable to keep track of bytes received
        let mut start_time = Instant::now(); // start time for measuring elapsed time

        let _payload = "ACK";

        thread::spawn(move || {
            loop {
                let mut buf = [0; 65536]; // Increase the buffer size to hold at least 9000 bytes
                match self.socket.recv_from(&mut buf) {
                    Ok((size, src)) => {
                        let payload: Vec<u8> = vec![0; size];
                        self.send(payload, src);
                        bytes_received += size; // update the counter variable
                        debug!("received {} bytes from {:?}", size, src); // Use logging with tracing
                                                                          // trace!("data: {:?}", &buf[..size]);  // Use logging with tracing
                    }
                    Err(_e) => {
                        // error!("Error receiving data");  // Use logging with tracing
                    }
                }

                let elapsed_time = start_time.elapsed(); // measure elapsed time since start
                if elapsed_time > Duration::from_secs(1) {
                    // if elapsed time is greater than 1 second
                    let bytes_per_second = bytes_received as f64 / elapsed_time.as_secs_f64(); // calculate bytes per second rate
                    if bytes_per_second > 0.0 {
                        info!(
                            "Data Rate: {:.2} Mbps",
                            bytes_per_second * 8.0 / 1_000_000.0
                        ); // print the rate with logging
                    }
                    bytes_received = 0; // reset the counter variable
                    start_time = Instant::now(); // reset the start time
                }
            }
        })
    }

    fn send(&self, payload: Vec<u8>, src: SocketAddr) {
        self.socket.send_to(&payload, src).unwrap(); // send the data
    }
}
