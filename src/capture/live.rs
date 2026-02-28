use anyhow::{Context, Result};
use pcap::{Active, Capture as PcapCapture};
use std::sync::{Arc, Mutex};

/// 共享统计信息
#[derive(Default, Clone)]
pub struct CaptureStats {
    pub packet_count: u64,
    pub byte_count: u64,
}

/// 实时抓包配置
pub struct LiveCaptureConfig {
    /// 网卡名称
    pub interface: String,
    /// BPF 过滤器
    pub filter: Option<String>,
    /// 抓包超时（毫秒）
    pub timeout: i32,
    /// 快照长度
    pub snaplen: u32,
}

impl Default for LiveCaptureConfig {
    fn default() -> Self {
        Self {
            interface: String::new(),
            filter: None,
            timeout: 1000,
            snaplen: 65535,
        }
    }
}

/// 实时抓包器
pub struct LiveCapture {
    config: LiveCaptureConfig,
    handle: Option<PcapCapture<Active>>,
    running: bool,
    stats: Arc<Mutex<CaptureStats>>,
    last_packet: Arc<Mutex<Option<Vec<u8>>>>,
}

impl LiveCapture {
    pub fn new(interface: &str, filter: Option<&str>) -> Result<Self> {
        let config = LiveCaptureConfig {
            interface: interface.to_string(),
            filter: filter.map(String::from),
            ..Default::default()
        };

        Ok(Self {
            config,
            handle: None,
            running: false,
            stats: Arc::new(Mutex::new(CaptureStats::default())),
            last_packet: Arc::new(Mutex::new(None)),
        })
    }

    /// 打开网卡并设置过滤器
    pub fn open(&mut self) -> Result<()> {
        // 查找指定名称的网卡
        let device = pcap::Device::lookup()
            .context("Failed to lookup network devices")?
            .ok_or_else(|| anyhow::anyhow!("No network devices found"))?;

        // 如果指定了网卡，查找匹配的
        let target_device = if self.config.interface.is_empty() {
            device
        } else {
            pcap::Device::list()
                .context("Failed to list devices")?
                .into_iter()
                .find(|d| d.name == self.config.interface)
                .ok_or_else(|| {
                    anyhow::anyhow!("Device '{}' not found", self.config.interface)
                })?
        };

        // 打开网卡
        let mut capture = PcapCapture::from_device(target_device)?
            .promisc(false)
            .snaplen(self.config.snaplen as i32)
            .timeout(self.config.timeout)
            .open()
            .with_context(|| format!("Failed to open device '{}'", self.config.interface))?;

        // 设置 BPF 过滤器
        if let Some(ref filter) = self.config.filter {
            capture
                .filter(filter, true)
                .with_context(|| format!("Failed to set filter '{}'", filter))?;
        }

        self.handle = Some(capture);
        Ok(())
    }

    /// 开始循环抓包
    pub fn start(&mut self) -> Result<()> {
        self.open()?;
        self.running = true;
        Ok(())
    }

    /// 停止抓包
    pub fn stop(&mut self) {
        self.running = false;
        self.handle = None;
    }

    /// 是否正在运行
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// 抓取一个数据包
    pub fn capture_one(&mut self) -> Result<Option<PacketInfo>> {
        let capture = match &mut self.handle {
            Some(c) => c,
            None => return Ok(None),
        };

        match capture.next_packet() {
            Ok(packet) => {
                let data = (&*packet).to_vec();
                let bytes = data.len() as u64;

                // 更新统计
                {
                    let mut stats = self.stats.lock().unwrap();
                    stats.packet_count += 1;
                    stats.byte_count += bytes;
                }

                // 识别协议
                let protocol = Self::identify_protocol(&data);
                let packet_info = PacketInfo {
                    data,
                    bytes,
                    protocol,
                };

                Ok(Some(packet_info))
            }
            Err(pcap::Error::NoMorePackets) => Ok(None),
            Err(pcap::Error::TimeoutExpired) => Ok(None),
            Err(e) => Err(anyhow::anyhow!("Capture error: {}", e)),
        }
    }

    /// 识别数据包的主要协议
    fn identify_protocol(data: &[u8]) -> &'static str {
        if data.len() < 14 {
            return "unknown";
        }

        // 检查以太网类型
        let ethertype = u16::from_be_bytes([data[12], data[13]]);

        match ethertype {
            0x0800 => {
                // IPv4
                if data.len() > 23 {
                    let protocol = data[23];
                    match protocol {
                        6 => {
                            // TCP，检查端口识别 HTTP
                            let ip_header_len = ((data[14] & 0x0F) as usize) * 4;
                            let tcp_start = 14 + ip_header_len;
                            if data.len() > tcp_start + 3 {
                                let dst_port =
                                    u16::from_be_bytes([data[tcp_start + 2], data[tcp_start + 3]]);
                                if dst_port == 80 || dst_port == 8080 || dst_port == 8000 {
                                    return "http";
                                }
                            }
                            "tcp"
                        }
                        17 => {
                            // UDP，检查 DNS 端口
                            let ip_header_len = ((data[14] & 0x0F) as usize) * 4;
                            let udp_start = 14 + ip_header_len;
                            if data.len() > udp_start + 3 {
                                let dst_port =
                                    u16::from_be_bytes([data[udp_start + 2], data[udp_start + 3]]);
                                if dst_port == 53 || dst_port == 5353 {
                                    return "dns";
                                }
                            }
                            "udp"
                        }
                        _ => "ip",
                    }
                } else {
                    "ip"
                }
            }
            0x86DD => "ipv6",
            _ => "ethernet",
        }
    }

    /// 获取统计信息
    pub fn stats(&self) -> CaptureStats {
        self.stats.lock().unwrap().clone()
    }

    /// 重置统计
    pub fn reset_stats(&self) {
        let mut stats = self.stats.lock().unwrap();
        stats.packet_count = 0;
        stats.byte_count = 0;
    }
}

/// 数据包信息
pub struct PacketInfo {
    pub data: Vec<u8>,
    pub bytes: u64,
    pub protocol: &'static str,
}

/// 捕获实时数据包（简化版本，用于 CLI）
pub fn capture(interface: Option<&str>, filter: Option<&str>, count: u32) -> Result<()> {
    let iface = interface.unwrap_or("enp89s0");
    let mut capture = LiveCapture::new(iface, filter)?;
    capture.start()?;

    println!("Starting live capture on {}...", iface);
    if let Some(ref f) = filter {
        println!("Filter: {}", f);
    }
    if count > 0 {
        println!("Max packets: {}", count);
    }
    println!("Press Ctrl+C to stop\n");

    let mut captured = 0u64;
    while capture.is_running() {
        match capture.capture_one() {
            Ok(Some(packet)) => {
                captured += 1;
                println!("[{}] Captured {} bytes", packet.protocol, packet.bytes);
                if count > 0 && captured >= count as u64 {
                    break;
                }
            }
            Ok(None) => continue,
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }

    let stats = capture.stats();
    println!("\nCapture complete: {} packets, {} bytes", stats.packet_count, stats.byte_count);
    Ok(())
}
