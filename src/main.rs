use clap::{Parser, Subcommand};

mod capture;
mod filter;
mod output;
mod protocol;
mod stats;
mod tui;

#[derive(Parser)]
#[command(name = "cap")]
#[command(author = "Your Name")]
#[command(version = "0.1.0")]
#[command(about = "High-performance network packet capture and analysis tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Real-time packet capture from network interface
    Live {
        /// Network interface name (e.g., eth0, en0)
        #[arg(short, long)]
        interface: Option<String>,

        /// BPF filter expression (e.g., "port 80", "host 192.168.1.1")
        #[arg(short, long)]
        filter: Option<String>,

        /// Output format
        #[arg(long, default_value = "console")]
        format: OutputFormat,

        /// Maximum packets to capture (0 = unlimited)
        #[arg(short, long, default_value = "0")]
        count: u32,
    },

    /// Analyze offline pcap file
    Analyze {
        /// Path to pcap file
        #[arg(required = true)]
        file: String,

        /// Filter by protocol (ethernet, ip, tcp, http, dns)
        #[arg(short, long)]
        protocol: Option<String>,

        /// Output format
        #[arg(long, default_value = "console")]
        format: OutputFormat,
    },

    /// List available network interfaces
    Devices,

    /// Show traffic statistics from pcap file
    Stats {
        /// Path to pcap file
        #[arg(required = true)]
        file: String,
    },

    /// TUI visual interface (demo mode)
    Tui {
        /// Network interface for live capture
        #[arg(short, long)]
        interface: Option<String>,

        /// Replay mode: path to pcap file
        #[arg(long)]
        replay: Option<String>,
    },
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum OutputFormat {
    Console,
    Json,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Live { interface, filter, format: _, count } => {
            capture::live::capture(interface.as_deref(), filter.as_deref(), count)?;
        }
        Commands::Analyze { file, protocol, format: _ } => {
            capture::offline::analyze(&file, protocol.as_deref())?;
        }
        Commands::Devices => {
            capture::device::list_devices()?;
        }
        Commands::Stats { file } => {
            stats::show_stats(&file)?;
        }
        Commands::Tui { interface, replay } => {
            if let Some(file) = replay {
                println!("Replay mode: {}", file);
            }
            tui::run_tui(interface.as_deref())?;
        }
    }

    Ok(())
}
