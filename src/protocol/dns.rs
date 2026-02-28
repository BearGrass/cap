use super::parser::{DnsData, DnsQuery, DnsRecord, PacketParser};

#[derive(Debug)]
pub struct DnsParser;

impl DnsParser {
    pub fn new() -> Self {
        Self
    }

    fn parse_name(&self, data: &[u8], mut offset: usize) -> Option<(String, usize)> {
        let mut name = String::new();
        let mut jumped = false;
        let max_jumps = 10;
        let mut jumps = 0;

        loop {
            if offset >= data.len() {
                break;
            }

            let len = data[offset];

            if len == 0 {
                if !jumped {
                    offset += 1;
                }
                break;
            }

            // Check for compression pointer
            if (len & 0xC0) == 0xC0 {
                if offset + 1 >= data.len() {
                    return None;
                }
                if !jumped {
                    offset += 1;
                }
                let _pointer = ((len & 0x3F) as usize) << 8 | (data[offset] as usize);
                offset += 1;
                jumped = true;
                jumps += 1;
                if jumps > max_jumps {
                    return None; // Prevent infinite loops
                }
                continue;
            }

            offset += 1;
            if offset + len as usize > data.len() {
                return None;
            }

            if let Ok(label) = str::from_utf8(&data[offset..offset + len as usize]) {
                if !name.is_empty() {
                    name.push('.');
                }
                name.push_str(label);
            }
            offset += len as usize;
        }

        if !jumped {
            return Some((name, offset));
        }
        Some((name, offset))
    }

    fn parse_query(&self, data: &[u8], mut offset: usize) -> Option<(DnsQuery, usize)> {
        let (name, new_offset) = self.parse_name(data, offset)?;
        offset = new_offset;

        if offset + 4 > data.len() {
            return None;
        }

        let query_type = u16::from_be_bytes([data[offset], data[offset + 1]]);
        offset += 4; // Skip type and class

        let query_type_str = match query_type {
            1 => "A",
            2 => "NS",
            5 => "CNAME",
            15 => "MX",
            16 => "TXT",
            28 => "AAAA",
            _ => "UNKNOWN",
        };

        Some((
            DnsQuery {
                name,
                query_type: query_type_str.to_string(),
            },
            offset,
        ))
    }

    fn parse_record(&self, data: &[u8], mut offset: usize) -> Option<(DnsRecord, usize)> {
        let (name, new_offset) = self.parse_name(data, offset)?;
        offset = new_offset;

        if offset + 10 > data.len() {
            return None;
        }

        let record_type = u16::from_be_bytes([data[offset], data[offset + 1]]);
        offset += 4; // Skip type and class
        let ttl = u32::from_be_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]]);
        offset += 4;
        let rd_length = u16::from_be_bytes([data[offset], data[offset + 1]]) as usize;
        offset += 2;

        if offset + rd_length > data.len() {
            return None;
        }

        let rd_data = &data[offset..offset + rd_length];
        let data_str = match record_type {
            1 if rd_length == 4 => {
                // A record
                format!("{}.{}.{}.{}", rd_data[0], rd_data[1], rd_data[2], rd_data[3])
            }
            _ => format!("{:02x?}", rd_data),
        };

        let record_type_str = match record_type {
            1 => "A",
            2 => "NS",
            5 => "CNAME",
            15 => "MX",
            16 => "TXT",
            28 => "AAAA",
            _ => "UNKNOWN",
        };

        offset += rd_length;

        Some((
            DnsRecord {
                name,
                record_type: record_type_str.to_string(),
                data: data_str,
                ttl,
            },
            offset,
        ))
    }
}

impl PacketParser for DnsParser {
    type Output = DnsData;
    type Error = DnsError;

    fn parse(&self, data: &[u8]) -> Result<Self::Output, Self::Error> {
        if data.len() < 12 {
            return Err(DnsError::TooShort);
        }

        let transaction_id = u16::from_be_bytes([data[0], data[1]]);
        let flags = u16::from_be_bytes([data[2], data[3]]);
        let is_response = (flags & 0x8000) != 0;

        let qd_count = u16::from_be_bytes([data[4], data[5]]) as usize;
        let an_count = u16::from_be_bytes([data[6], data[7]]) as usize;

        let mut offset = 12;
        let mut queries = Vec::new();
        let mut answers = Vec::new();

        // Parse queries
        for _ in 0..qd_count {
            if let Some((query, new_offset)) = self.parse_query(data, offset) {
                queries.push(query);
                offset = new_offset;
            } else {
                break;
            }
        }

        // Parse answers
        for _ in 0..an_count {
            if let Some((record, new_offset)) = self.parse_record(data, offset) {
                answers.push(record);
                offset = new_offset;
            } else {
                break;
            }
        }

        Ok(DnsData {
            transaction_id,
            is_response,
            queries,
            answers,
        })
    }

    fn protocol_name(&self) -> &'static str {
        "DNS"
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DnsError {
    #[error("Packet too short for DNS header (minimum 12 bytes)")]
    TooShort,
    #[error("Invalid DNS format")]
    InvalidFormat,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dns_parser() {
        let parser = DnsParser::new();
        // Simplified DNS query for "example.com" type A
        let data = [
            0x12, 0x34, // Transaction ID
            0x01, 0x00, // Flags: standard query
            0x00, 0x01, // Questions: 1
            0x00, 0x00, // Answer RRs: 0
            0x00, 0x00, // Authority RRs: 0
            0x00, 0x00, // Additional RRs: 0
            0x07, b'e', b'x', b'a', b'm', b'p', b'l', b'e',
            0x03, b'c', b'o', b'm',
            0x00,       // Null terminator
            0x00, 0x01, // Type: A
            0x00, 0x01, // Class: IN
        ];

        let result = parser.parse(&data).unwrap();
        assert_eq!(result.transaction_id, 0x1234);
        assert!(!result.is_response);
        assert_eq!(result.queries.len(), 1);
        assert_eq!(result.queries[0].name, "example.com");
        assert_eq!(result.queries[0].query_type, "A");
    }
}
