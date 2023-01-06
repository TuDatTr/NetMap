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
    pub fn run(self, data_rate: f64, packet_size: usize, sleep_adjust_factor: u32) -> thread::JoinHandle<()> {
        // const BUF_SIZE: usize =  65536;
        const BUF_SIZE: usize = 9000;
        let mut buf = [0; BUF_SIZE]; // Increase the buffer size to hold at least 9000 bytes

        let payload_size: usize = packet_size - UDP_HEADER_SIZE; // size of packets to send in bytes
        const UDP_HEADER_SIZE: usize = 42;

        let payload: Vec<u8> = vec![0; payload_size]; // create a vector of 0s with the specified packet size
        let packet_size_bits = payload_size * 8; // size of packets in bits

        // Initial interval at which to sleep
        let total_iteration_duration = packet_size_bits as f64 / (data_rate * 1000000.0); // interval in seconds to pause the loop
        let total_iteration_duration = total_iteration_duration * 1_000_000_000.0; // convert to nanoseconds
        let target_iteration_duration = Duration::from_nanos(total_iteration_duration as u64);

        // Add a counter variable to keep track of the number of iterations.
        let mut iteration_counter = 0;
        let mut sleep_timer = Duration::from_micros(0);

        // Start a timer to measure the elapsed time.
        let mut iteration_start = Instant::now();
        let mut second_timer = Instant::now();
        let mut sleep_duration = target_iteration_duration;

        // Log the data rate, packet size, and interval.
        info!(
            "Data rate: {} Mbit/s, Packet size: {} bytes, Target iteration duration: {:?}",
            data_rate, payload_size, target_iteration_duration
        );

        let mut adjustments = 0i64;
        thread::spawn(move || {
            loop {
                // Send the data.
                let _ = &self
                    .recv_socket
                    .send_to(&payload, self.target_address)
                    .unwrap(); // send the data
                trace!("Sent {} bytes to {}", payload_size, &self.target_address);

                // Pause the loop for the specified interval
                Self::busy_sleep(sleep_duration);

                // Increment iteration counter and iteration_timer
                iteration_counter += 1;
                sleep_timer += sleep_duration;

                // Check if one second has passed.
                let second_elapsed = second_timer.elapsed();
                if second_elapsed >= Duration::from_secs(1) {
                    let _ = &self.loop_log(second_elapsed, sleep_timer, iteration_counter,adjustments);

                    // Reset the counters and timers.
                    adjustments = 0;
                    iteration_counter = 0;
                    sleep_timer = Duration::from_micros(0);
                    second_timer = Instant::now();
                }

                match &self.recv_socket.recv_from(&mut buf) {
                    Ok((size, src)) => {
                        trace!("received {} bytes from {:?}", size, src); // Use logging with tracing

                        // trace!("data: {:?}", &buf[..size]); // Use logging with tracing
                    }
                    Err(_e) => {}
                }

                buf = unsafe { std::mem::zeroed() };

                // Calculate the elapsed time and recalculate necessary sleep_duration.
                let real_iteration_duration = iteration_start.elapsed();
                let former_sleep_duration = sleep_duration;
                sleep_duration = Self::adjust_sleep(
                    sleep_duration,
                    real_iteration_duration,
                    target_iteration_duration,
                    sleep_adjust_factor,
                );

                if former_sleep_duration < sleep_duration {
                    adjustments += 1;
                } else {
                    adjustments -= 1;
                }
                iteration_start = Instant::now();
            }
        })
    }

    fn busy_sleep(duration: Duration) {
        let start = Instant::now();

        loop {
            let elapsed = start.elapsed();
            if elapsed >= duration {
                break;
            }
        }
    }

    fn adjust_sleep(current: Duration, actual: Duration, target: Duration, factor: u32) -> Duration {
        Self::aimd(current, actual, target, factor)
    }

    fn aimd(current: Duration, actual: Duration, target: Duration, factor: u32) -> Duration {
        debug!("{:?}, {:?}\n", actual, current);
        if actual >= target { // Multiplicative Decrease 
            current / 2 // Half sleeptime
        } else { // Addative Increase
            current + (target / factor) // Add 20% of target time
        }
    }

    fn loop_log(&self, second_elapsed: Duration, sleep_timer: Duration, iteration_counter: u32, adjustments: i64) {
        // The average time that each iteration of the loop took to run.
        let avg_pass_time = second_elapsed / iteration_counter;
        // The average time that the loop slept between iterations.
        let avg_sleep_duration = sleep_timer / iteration_counter;

        // The average time that each iteration of the loop took to run, minus the average time that the loop slept between iterations.
        let avg_runtime = match avg_pass_time > avg_sleep_duration {
            true => avg_pass_time - avg_sleep_duration,
            false => Duration::from_micros(0),
        };

        // Display the average pass time.
        info!(
            "Average pass time: {:?}, Average sleep duration: {:?}, Average runtime: {:?}, Adjustments(optimal: 0): {}",
            avg_pass_time, avg_sleep_duration, avg_runtime, adjustments
        );
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
