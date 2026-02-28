use anyhow::Result;

/// List available network interfaces for packet capture
pub fn list_devices() -> Result<()> {
    // TODO: Implement device enumeration using pcap crate
    println!("Available network interfaces:");

    // Placeholder - will be implemented in next iteration
    println!("  [ ] Implementation pending");

    Ok(())
}
