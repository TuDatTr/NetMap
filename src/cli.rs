pub mod modes;

use clap::Parser;
use modes::RunMode;

#[derive(Parser, Debug)]
#[command(
    author = "Tuan-Dat Tran <tuan-dat.tran@tudattr.dev>",
    version = "0.1.0",
    about = "NetMap is a network performance testing tool that allows users to record bandwidths at different physical locations of their wireless network using customized UDP traffic and GPS tracking.",
    long_about
)]

pub struct Cli {
    /// set whether to run as a sender or receiver
    #[command(subcommand)]
    pub mode: RunMode,
}
