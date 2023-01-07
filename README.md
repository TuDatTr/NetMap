# NetMap

NetMap is a Rust library for testing the performance of a wireless network at different physical locations using GPS tracking and custom UDP traffic.

# Installation

To install NetMap, you will need the latest version of Rust and Cargo. Follow the instructions on the [Rust website](https://www.rust-lang.org/tools/install) to install these tools.

Once you have Rust and Cargo installed, clone the NetMap repository and build the project with the following commands:

```sh
git clone https://gitlab.com/TuDatTr/netmap
cd netmap
cargo build --release
```

This will build the NetMap binary and place it in the `target/release` directory. You can then move the binary to a location in your `PATH` so that you can run it from any directory.

```sh
mv target/release/netmap /usr/local/bin/
```

Alternatively, you can install the NetMap binary with Cargo by running:

```sh
cargo install --path .
```

This will install the NetMap binary to `~/.cargo/bin`, which should be in your PATH if you followed the instructions on the Rust website.

# Usage

NetMap is a command-line tool that allows users to test the network performance of their wireless network at specific locations. The tool generates UDP traffic and tracks the location of the network using GPS. The tool can be run in two modes: `sender` and `receiver`.

To use NetMap, first install the tool using the instructions in the "Installation" section.

To run NetMap in `sender` mode, use the following command:

```sh
netmap sender -p [port] -T [target_ip] -P [target_port] -d [data_rate] -s [packet_size] -f [sleep_adjust] [-o output_file]
```

To run NetMap in `receiver` mode, use the following command:

```sh
netmap receiver -p [port] [-o output_file]
```

For more information on the available options for each mode, use the `--help` flag.

```sh
netmap sender --help
```

```sh
netmap receiver --help
```

# Example Output
Example Output

The following is an example of NetMap output when run in Sender mode:

```
Data rate: 1 Mbit/s, Packet size: 1458 bytes, Target iteration duration: 11.664ms
Average pass time: 8.776975ms, Average sleep duration: 8.748ms, Average runtime: 28.975µs
Average pass time: 8.778581ms, Average sleep duration: 8.748ms, Average runtime: 30.581µs
Average pass time: 8.787669ms, Average sleep duration: 8.748ms, Average runtime: 39.669µs
Average pass time: 8.795984ms, Average sleep duration: 8.773356ms, Average runtime: 22.628µs
Average pass time: 8.744477ms, Average sleep duration: 8.722643ms, Average runtime: 21.834µs
```

The output shows the data rate, packet size, and target iteration duration at the beginning of the run. The following lines show the time elapsed, sleep timer, number of iterations, and number of adjustments made to the sleep duration for each second of the run.

The following is an example of NetMap output when run in Receiver mode:

```sh
Data Rate: 1.33 Mbps
Data Rate: 1.33 Mbps
Data Rate: 1.33 Mbps
Data Rate: 1.33 Mbps
Data Rate: 1.33 Mbps
Data Rate: 1.33 Mbps
Data Rate: 1.00 Mbps
```

The output shows the IP and port that the Receiver is listening on, as well as the IP, port, and number of bytes received for each packet received.
