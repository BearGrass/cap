use crate::protocol::Packet;
use serde_json;

/// Print packet as JSON
pub fn print_packet_json(packet: &Packet) -> serde_json::Result<()> {
    let json = serde_json::to_string_pretty(packet)?;
    println!("{json}");
    Ok(())
}
