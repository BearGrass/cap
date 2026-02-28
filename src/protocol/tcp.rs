use super::parser::{TcpFlags, TcpHeader, PacketParser};

#[derive(Debug)]
pub struct TcpParser;

impl TcpParser {
    pub fn new() -> Self {
        Self
    }
}

impl PacketParser for TcpParser {
    type Output = TcpHeader;
    type Error = TcpError;

    fn parse(&self, data: &[u8]) -> Result<Self::Output, Self::Error> {
        if data.len() < 20 {
            return Err(TcpError::TooShort);
        }

        let src_port = u16::from_be_bytes([data[0], data[1]]);
        let dst_port = u16::from_be_bytes([data[2], data[3]]);
        let seq = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
        let ack = u32::from_be_bytes([data[8], data[9], data[10], data[11]]);

        let flags_byte = data[13];
        let flags = TcpFlags {
            fin: (flags_byte & 0x01) != 0,
            syn: (flags_byte & 0x02) != 0,
            rst: (flags_byte & 0x04) != 0,
            psh: (flags_byte & 0x08) != 0,
            ack: (flags_byte & 0x10) != 0,
            urg: (flags_byte & 0x20) != 0,
        };

        Ok(TcpHeader {
            src_port,
            dst_port,
            seq,
            ack,
            flags,
        })
    }

    fn protocol_name(&self) -> &'static str {
        "TCP"
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TcpError {
    #[error("Packet too short for TCP header (minimum 20 bytes)")]
    TooShort,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tcp_parser() {
        let parser = TcpParser::new();
        // Minimal TCP header (20 bytes)
        let mut data = [0u8; 20];
        data[0] = 0x1f; data[1] = 0x90; // src_port: 8080
        data[2] = 0x00; data[3] = 0x50; // dst_port: 80
        data[4] = 0x00; data[5] = 0x00; data[6] = 0x00; data[7] = 0x01; // seq: 1
        data[8] = 0x00; data[9] = 0x00; data[10] = 0x00; data[11] = 0x00; // ack: 0
        data[13] = 0x02; // SYN flag

        let result = parser.parse(&data).unwrap();
        assert_eq!(result.src_port, 8080);
        assert_eq!(result.dst_port, 80);
        assert_eq!(result.seq, 1);
        assert!(result.flags.syn);
        assert!(!result.flags.fin);
    }
}
