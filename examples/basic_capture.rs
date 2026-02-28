//! Basic packet capture example
//!
//! This example demonstrates how to use the protocol parsers
//! to analyze raw packet data.

use cap::{
    EthernetParser, IpParser, TcpParser,
    PacketParser,
};

fn main() {
    // Example: Parse Ethernet frame
    let ethernet_parser = EthernetParser::new();
    let eth_data = [
        0x00, 0x1a, 0x2b, 0x3c, 0x4d, 0x5e, // dst_mac
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55, // src_mac
        0x08, 0x00, // IPv4
    ];

    match ethernet_parser.parse(&eth_data) {
        Ok(header) => {
            println!("Ethernet: {} -> {}", header.src_mac, header.dst_mac);
            println!("  Type: 0x{:04x} (IPv4)", header.ether_type);
        }
        Err(e) => println!("Ethernet parse error: {}", e),
    }

    // Example: Parse IPv4 header
    let ip_parser = IpParser::new();
    let mut ip_data = [0u8; 20];
    ip_data[0] = 0x45; // Version 4, IHL 5
    ip_data[8] = 64;   // TTL
    ip_data[9] = 6;    // Protocol (TCP)
    ip_data[12] = 192; ip_data[13] = 168; ip_data[14] = 1; ip_data[15] = 100;
    ip_data[16] = 8; ip_data[17] = 8; ip_data[18] = 8; ip_data[19] = 8;

    match ip_parser.parse(&ip_data) {
        Ok(header) => {
            println!("\nIP: {} -> {}", header.src, header.dst);
            println!("  TTL: {}, Protocol: {}", header.ttl, header.protocol);
        }
        Err(e) => println!("IP parse error: {}", e),
    }

    // Example: Parse TCP header
    let tcp_parser = TcpParser::new();
    let mut tcp_data = [0u8; 20];
    tcp_data[0] = 0x1f; tcp_data[1] = 0x90; // src_port: 8080
    tcp_data[2] = 0x00; tcp_data[3] = 0x50; // dst_port: 80
    tcp_data[4] = 0x00; tcp_data[5] = 0x00; tcp_data[6] = 0x00; tcp_data[7] = 0x01;
    tcp_data[13] = 0x02; // SYN flag

    match tcp_parser.parse(&tcp_data) {
        Ok(header) => {
            println!("\nTCP: {} -> {}", header.src_port, header.dst_port);
            println!("  SEQ: {}, ACK: {}", header.seq, header.ack);
            println!("  Flags: SYN={}, ACK={}, FIN={}", header.flags.syn, header.flags.ack, header.flags.fin);
        }
        Err(e) => println!("TCP parse error: {}", e),
    }
}
