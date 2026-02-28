use serde::Serialize;

/// Common trait for all packet parsers
pub trait PacketParser {
    type Output;
    type Error;

    /// Parse raw packet data into structured format
    fn parse(&self, data: &[u8]) -> Result<Self::Output, Self::Error>;

    /// Get the protocol name
    fn protocol_name(&self) -> &'static str;
}

/// Unified packet representation
#[derive(Debug, Clone, Serialize)]
pub struct Packet {
    pub timestamp: u64,
    pub length: usize,
    pub layers: Vec<PacketLayer>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum PacketLayer {
    Ethernet(EthernetHeader),
    Ip(IpHeader),
    Tcp(TcpHeader),
    Udp(UdpHeader),
    Http(HttpData),
    Dns(DnsData),
    Unknown(Vec<u8>),
}

// Ethernet Layer
#[derive(Debug, Clone, Serialize)]
pub struct EthernetHeader {
    pub src_mac: String,
    pub dst_mac: String,
    pub ether_type: u16,
}

// IP Layer
#[derive(Debug, Clone, Serialize)]
pub struct IpHeader {
    pub version: u8,
    pub src: String,
    pub dst: String,
    pub protocol: u8,
    pub ttl: u8,
}

// TCP Layer
#[derive(Debug, Clone, Serialize)]
pub struct TcpHeader {
    pub src_port: u16,
    pub dst_port: u16,
    pub seq: u32,
    pub ack: u32,
    pub flags: TcpFlags,
}

#[derive(Debug, Clone, Serialize)]
pub struct TcpFlags {
    pub fin: bool,
    pub syn: bool,
    pub rst: bool,
    pub psh: bool,
    pub ack: bool,
    pub urg: bool,
}

// UDP Layer
#[derive(Debug, Clone, Serialize)]
pub struct UdpHeader {
    pub src_port: u16,
    pub dst_port: u16,
    pub length: u16,
}

// HTTP Layer
#[derive(Debug, Clone, Serialize)]
pub struct HttpData {
    pub method: Option<String>,
    pub path: Option<String>,
    pub status: Option<u16>,
    pub host: Option<String>,
    pub content_type: Option<String>,
}

// DNS Layer
#[derive(Debug, Clone, Serialize)]
pub struct DnsData {
    pub transaction_id: u16,
    pub is_response: bool,
    pub queries: Vec<DnsQuery>,
    pub answers: Vec<DnsRecord>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DnsQuery {
    pub name: String,
    pub query_type: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DnsRecord {
    pub name: String,
    pub record_type: String,
    pub data: String,
    pub ttl: u32,
}
