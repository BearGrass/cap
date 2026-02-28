mod analyzer;

pub use analyzer::Analyzer;

/// Show traffic statistics from pcap file
pub fn show_stats(file: &str) -> anyhow::Result<()> {
    // TODO: Implement statistics display
    println!("Statistics for: {}", file);
    println!("[INFO] Implementation in progress...");
    Ok(())
}
