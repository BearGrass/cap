use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::io;

use super::app::App;

/// 渲染 TUI 界面
pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  // 标题
            Constraint::Percentage(30), // 流量图
            Constraint::Percentage(30), // 协议分布
            Constraint::Percentage(30), // 连接列表
            Constraint::Length(3),  // 状态栏
        ])
        .split(frame.area());

    render_title(frame, chunks[0]);
    render_traffic_chart(frame, app, chunks[1]);
    render_protocol_stats(frame, app, chunks[2]);
    render_connection_list(frame, chunks[3]);
    render_status_bar(frame, chunks[4], app);
}

fn render_title(frame: &mut Frame, area: ratatui::layout::Rect) {
    let title = Paragraph::new("cap - Network Packet Analyzer")
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, area);
}

fn render_traffic_chart(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    // 构建带数值的流量图
    let mut lines = Vec::new();

    // 流量波形图
    let data: Vec<Span> = app
        .traffic_history
        .iter()
        .map(|&v| {
            let bar = match v {
                0 => " ",
                1..=10 => "░",
                11..=50 => "▒",
                51..=100 => "▓",
                _ => "█",
            };
            Span::raw(bar)
        })
        .collect();

    lines.push(Line::from(data));

    // 当前速率数值
    let current_packets = app.traffic_history.last().copied().unwrap_or(0);
    let current_bytes = app.bytes_history.last().copied().unwrap_or(0);
    lines.push(Line::from(vec![
        Span::styled(
            format!("  当前：{} 包/s", current_packets),
            Style::default().fg(Color::Green),
        ),
        Span::raw("  |  "),
        Span::styled(
            format!("{} 字节/s", format_bytes(current_bytes)),
            Style::default().fg(Color::Yellow),
        ),
    ]));

    // 平均速率
    lines.push(Line::from(vec![
        Span::styled(
            format!("  平均：{:.1} 包/s", app.avg_packets_per_sec()),
            Style::default().fg(Color::Cyan),
        ),
        Span::raw("  |  "),
        Span::styled(
            format!("{} 字节/s", format_bytes(app.avg_bytes_per_sec() as u64)),
            Style::default().fg(Color::Blue),
        ),
    ]));

    let traffic = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" 实时流量 ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green)),
        )
        .style(Style::default().fg(Color::Yellow));

    frame.render_widget(traffic, area);
}

/// 格式化字节数为易读格式
fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

fn render_protocol_stats(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let stats = &app.protocol_stats;
    let total = stats.tcp + stats.udp + stats.http + stats.dns + stats.other;

    let mut lines = Vec::new();

    if total > 0 {
        let tcp_pct = (stats.tcp as f64 / total as f64) * 100.0;
        let udp_pct = (stats.udp as f64 / total as f64) * 100.0;
        let http_pct = (stats.http as f64 / total as f64) * 100.0;
        let dns_pct = (stats.dns as f64 / total as f64) * 100.0;
        let other_pct = (stats.other as f64 / total as f64) * 100.0;

        lines.push(Line::from(vec![
            Span::styled("TCP:  ", Style::default().fg(Color::Green)),
            Span::raw(format!(
                "{} {} ({:.1}%)",
                "█".repeat((tcp_pct / 5.0) as usize),
                " ".repeat((20 - (tcp_pct / 5.0) as usize).max(0)),
                tcp_pct
            )),
            Span::raw("  "),
            Span::styled(format!("{}", stats.tcp), Style::default().fg(Color::White)),
        ]));

        lines.push(Line::from(vec![
            Span::styled("UDP:  ", Style::default().fg(Color::Blue)),
            Span::raw(format!(
                "{} {} ({:.1}%)",
                "█".repeat((udp_pct / 5.0) as usize),
                " ".repeat((20 - (udp_pct / 5.0) as usize).max(0)),
                udp_pct
            )),
            Span::raw("  "),
            Span::styled(format!("{}", stats.udp), Style::default().fg(Color::White)),
        ]));

        lines.push(Line::from(vec![
            Span::styled("HTTP: ", Style::default().fg(Color::Magenta)),
            Span::raw(format!(
                "{} {} ({:.1}%)",
                "█".repeat((http_pct / 5.0) as usize),
                " ".repeat((20 - (http_pct / 5.0) as usize).max(0)),
                http_pct
            )),
            Span::raw("  "),
            Span::styled(format!("{}", stats.http), Style::default().fg(Color::White)),
        ]));

        lines.push(Line::from(vec![
            Span::styled("DNS:  ", Style::default().fg(Color::Cyan)),
            Span::raw(format!(
                "{} {} ({:.1}%)",
                "█".repeat((dns_pct / 5.0) as usize),
                " ".repeat((20 - (dns_pct / 5.0) as usize).max(0)),
                dns_pct
            )),
            Span::raw("  "),
            Span::styled(format!("{}", stats.dns), Style::default().fg(Color::White)),
        ]));

        lines.push(Line::from(vec![
            Span::styled("其他：", Style::default().fg(Color::DarkGray)),
            Span::raw(format!(
                "{} {} ({:.1}%)",
                "█".repeat((other_pct / 5.0) as usize),
                " ".repeat((20 - (other_pct / 5.0) as usize).max(0)),
                other_pct
            )),
            Span::raw("  "),
            Span::styled(format!("{}", stats.other), Style::default().fg(Color::White)),
        ]));

        lines.push(Line::from(vec![Span::raw("─".repeat(40))]));
        lines.push(Line::from(vec![
            Span::styled("总计：", Style::default().fg(Color::Yellow)),
            Span::raw(format!("{} 包", total)),
        ]));
    } else {
        lines.push(Line::from("等待数据包..."));
    }

    let protocol = Paragraph::new(lines)
        .block(
            Block::default()
                .title(" 协议分布 ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue)),
        );

    frame.render_widget(protocol, area);
}

fn render_connection_list(frame: &mut Frame, area: ratatui::layout::Rect) {
    let items = vec![
        ListItem::new("192.168.1.100:8080 -> 8.8.8.8:53 (DNS)"),
        ListItem::new("192.168.1.100:44321 -> 172.217.14.110:443 (TCP)"),
        ListItem::new("192.168.1.100:55123 -> 93.184.216.34:80 (HTTP)"),
    ];

    let list = List::new(items)
        .block(
            Block::default()
                .title(" 活动连接 ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Magenta)),
        );

    frame.render_widget(list, area);
}

fn render_status_bar(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let mut status_parts = Vec::new();

    // 左侧：操作提示
    let pause_status = if app.paused { "p: 继续" } else { "p: 暂停" };
    status_parts.push(format!("q: 退出 | {} | r: 重置", pause_status));

    // 右侧：网卡信息
    if let Some(ref iface) = app.interface {
        status_parts.push(format!("| {}", iface));
    }

    // 错误信息
    if let Some(ref err) = app.error_message {
        status_parts.push(format!("| ERROR: {}", err));
    }

    let status = Paragraph::new(status_parts.join("  "))
        .style(Style::default().fg(Color::White).bg(Color::DarkGray));
    frame.render_widget(status, area);
}

/// 运行 TUI
pub fn run_tui(interface: Option<&str>) -> anyhow::Result<()> {
    use crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };

    // 设置终端
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = if let Some(iface) = interface {
        App::with_interface(iface)
    } else {
        App::new()
    };
    let res = run_app(&mut terminal, &mut app);

    // 恢复终端
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> anyhow::Result<()> {
    use crossterm::event::{self, Event, KeyCode};
    use std::time::Instant;

    // 如果指定了网卡，尝试打开实时抓包
    let mut live_capture = if let Some(ref iface) = app.interface {
        match crate::capture::live::LiveCapture::new(iface, None) {
            Ok(mut cap) => {
                match cap.start() {
                    Ok(()) => Some(cap),
                    Err(e) => {
                        app.error_message = Some(format!(
                            "无法在 {} 上抓包：{} (需要 root 权限？尝试 sudo cargo run -- tui -i {})",
                            iface, e, iface
                        ));
                        None
                    }
                }
            }
            Err(e) => {
                app.error_message = Some(format!("Failed to create capture: {}", e));
                None
            }
        }
    } else {
        None
    };

    let mut last_tick = Instant::now();
    let tick_rate = std::time::Duration::from_millis(500); // 0.5 秒更新一次

    loop {
        terminal.draw(|f| render(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| std::time::Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => app.quit(),
                    KeyCode::Char('p') => app.toggle_pause(),
                    KeyCode::Char('r') => app.reset(),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate && !app.paused {
            if let Some(ref mut capture) = live_capture {
                // 实时抓包模式
                let mut packets_this_second = 0u64;
                let mut bytes_this_second = 0u64;
                let mut protocols: Vec<&str> = Vec::new();

                // 非阻塞抓取最多 200 个包
                for _ in 0..200 {
                    match capture.capture_one() {
                        Ok(Some(packet)) => {
                            packets_this_second += 1;
                            bytes_this_second += packet.bytes;
                            protocols.push(packet.protocol);
                        }
                        Ok(None) => break,
                        Err(_) => break,
                    }
                }

                app.current_packets = packets_this_second;
                app.current_bytes = bytes_this_second;

                // 更新协议统计
                for proto in protocols {
                    match proto {
                        "tcp" => app.protocol_stats.tcp += 1,
                        "udp" => app.protocol_stats.udp += 1,
                        "http" => app.protocol_stats.http += 1,
                        "dns" => app.protocol_stats.dns += 1,
                        _ => app.protocol_stats.other += 1,
                    }
                }
            } else {
                // 演示模式：模拟数据
                app.current_packets = 20 + (app.elapsed_secs % 10);
                app.current_bytes = app.current_packets * 100;

                app.protocol_stats.tcp += app.current_packets / 3;
                app.protocol_stats.udp += app.current_packets / 4;
                app.protocol_stats.http += app.current_packets / 6;
                app.protocol_stats.dns += app.current_packets / 8;
                app.protocol_stats.other += app.current_packets / 8;
            }

            app.tick();
            last_tick = Instant::now();
        }

        if !app.running {
            break;
        }
    }

    Ok(())
}
