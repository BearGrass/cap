use crate::protocol::Packet;
use comfy_table::Table;

/// Print packet to console in human-readable format
pub fn print_packet(packet: &Packet) {
    let mut table = Table::new();
    table.set_header(vec!["Layer", "Info"]);

    for layer in &packet.layers {
        match layer {
            crate::protocol::PacketLayer::Ethernet(eth) => {
                table.add_row(vec!["Ethernet", &format!("{} -> {}", eth.src_mac, eth.dst_mac)]);
            }
            crate::protocol::PacketLayer::Ip(ip) => {
                table.add_row(vec!["IP", &format!("{} -> {} (TTL: {})", ip.src, ip.dst, ip.ttl)]);
            }
            crate::protocol::PacketLayer::Tcp(tcp) => {
                table.add_row(vec!["TCP", &format!("{} -> {}", tcp.src_port, tcp.dst_port)]);
            }
            crate::protocol::PacketLayer::Udp(udp) => {
                table.add_row(vec!["UDP", &format!("{} -> {}", udp.src_port, udp.dst_port)]);
            }
            crate::protocol::PacketLayer::Http(http) => {
                if let Some(method) = &http.method {
                    table.add_row(vec!["HTTP", &format!("{} {}", method, http.path.as_deref().unwrap_or(""))]);
                } else if let Some(status) = http.status {
                    table.add_row(vec!["HTTP", &format!("Response: {}", status)]);
                }
            }
            crate::protocol::PacketLayer::Dns(dns) => {
                for query in &dns.queries {
                    table.add_row(vec!["DNS", &format!("Query: {} ({})", query.name, query.query_type)]);
                }
            }
            crate::protocol::PacketLayer::Unknown(_) => {
                table.add_row(vec!["Unknown", "Raw data"]);
            }
        }
    }

    println!("{table}");
}
