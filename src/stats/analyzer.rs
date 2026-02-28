use crate::protocol::Packet;

/// Traffic analyzer for statistics
pub struct Analyzer {
    packet_count: u64,
    #[allow(dead_code)]
    total_bytes: u64,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            packet_count: 0,
            total_bytes: 0,
        }
    }

    pub fn analyze(&mut self, _packet: &Packet) {
        self.packet_count += 1;
        // TODO: Implement detailed analysis
    }

    pub fn packet_count(&self) -> u64 {
        self.packet_count
    }
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}
