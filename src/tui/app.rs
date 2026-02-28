use crate::stats::Analyzer;

/// TUI 应用状态
pub struct App {
    /// 是否正在运行
    pub running: bool,
    /// 是否暂停
    pub paused: bool,
    /// 数据包分析器
    pub analyzer: Analyzer,
    /// 实时流量历史（秒级）- 包数
    pub traffic_history: Vec<u64>,
    /// 实时字节流量历史（秒级）
    pub bytes_history: Vec<u64>,
    /// 协议统计
    pub protocol_stats: ProtocolStats,
    /// 当前秒的包计数
    pub current_packets: u64,
    /// 当前秒的字节计数
    pub current_bytes: u64,
    /// 总包数
    pub total_packets: u64,
    /// 总字节数
    pub total_bytes: u64,
    /// 运行时长（秒）
    pub elapsed_secs: u64,
    /// 网卡名称
    pub interface: Option<String>,
    /// 错误信息
    pub error_message: Option<String>,
}

#[derive(Default)]
pub struct ProtocolStats {
    pub tcp: u64,
    pub udp: u64,
    pub http: u64,
    pub dns: u64,
    pub other: u64,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            paused: false,
            analyzer: Analyzer::new(),
            traffic_history: Vec::with_capacity(60),
            bytes_history: Vec::with_capacity(60),
            protocol_stats: ProtocolStats::default(),
            current_packets: 0,
            current_bytes: 0,
            total_packets: 0,
            total_bytes: 0,
            elapsed_secs: 0,
            interface: None,
            error_message: None,
        }
    }

    pub fn with_interface(interface: &str) -> Self {
        let mut app = Self::new();
        app.interface = Some(interface.to_string());
        app
    }

    /// 每秒 tick 时调用，更新统计数据
    pub fn tick(&mut self) {
        // 将当前秒的数据加入历史
        if self.traffic_history.len() >= 60 {
            self.traffic_history.remove(0);
            self.bytes_history.remove(0);
        }
        self.traffic_history.push(self.current_packets);
        self.bytes_history.push(self.current_bytes);

        // 累加总数
        self.total_packets += self.current_packets;
        self.total_bytes += self.current_bytes;
        self.elapsed_secs += 1;

        // 重置当前秒计数
        self.current_packets = 0;
        self.current_bytes = 0;
    }

    /// 添加一个数据包的统计
    pub fn add_packet(&mut self, bytes: u64, protocol: &str) {
        self.current_packets += 1;
        self.current_bytes += bytes;

        match protocol.to_lowercase().as_str() {
            "tcp" => self.protocol_stats.tcp += 1,
            "udp" => self.protocol_stats.udp += 1,
            "http" => self.protocol_stats.http += 1,
            "dns" => self.protocol_stats.dns += 1,
            _ => self.protocol_stats.other += 1,
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    pub fn reset(&mut self) {
        self.traffic_history.clear();
        self.bytes_history.clear();
        self.protocol_stats = ProtocolStats::default();
        self.total_packets = 0;
        self.total_bytes = 0;
        self.elapsed_secs = 0;
        self.current_packets = 0;
        self.current_bytes = 0;
    }

    /// 获取平均包速率
    pub fn avg_packets_per_sec(&self) -> f64 {
        if self.elapsed_secs == 0 {
            return 0.0;
        }
        self.total_packets as f64 / self.elapsed_secs as f64
    }

    /// 获取平均字节速率
    pub fn avg_bytes_per_sec(&self) -> f64 {
        if self.bytes_history.is_empty() {
            return 0.0;
        }
        self.bytes_history.iter().sum::<u64>() as f64 / self.bytes_history.len() as f64
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
