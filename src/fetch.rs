use ansi_width::ansi_width;
use anyhow::Result;
use colored::Colorize;
use image::{imageops::FilterType, DynamicImage};
use px2ansi::{CharsetMode, RenderOptions};
use std::{env, io::Write};

use sysinfo::{
    CpuRefreshKind, Disks, MemoryRefreshKind, Networks, ProcessRefreshKind, RefreshKind, System,
};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Minimum terminal columns required for the right-hand info text to display.
const MIN_RIGHT_BUDGET: usize = 12;
/// Horizontal space (in columns) between the image and the system info.
const GAP: usize = 1;

// ---------------------------------------------------------------------------
// Terminal size
// ---------------------------------------------------------------------------

/// Returns the current terminal width in columns.
/// Defaults to 80 if it cannot be determined via environment or ioctl.
fn term_cols() -> usize {
    if let Some(n) = env::var("COLUMNS")
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .filter(|&n: &usize| n > 0)
    {
        return n;
    }
    #[cfg(unix)]
    {
        use std::os::unix::io::AsRawFd;
        for fd in [std::io::stderr().as_raw_fd(), std::io::stdout().as_raw_fd()] {
            let mut ws: libc::winsize = unsafe { std::mem::zeroed() };
            if unsafe { libc::ioctl(fd, libc::TIOCGWINSZ, &mut ws) } == 0 && ws.ws_col > 0 {
                return ws.ws_col as usize;
            }
        }
    }
    80
}

/// Calculates the approximate width of a single terminal character cell in pixels.
fn term_cell_px_w() -> u32 {
    #[cfg(unix)]
    {
        use std::os::unix::io::AsRawFd;
        for fd in [std::io::stderr().as_raw_fd(), std::io::stdout().as_raw_fd()] {
            let mut ws: libc::winsize = unsafe { std::mem::zeroed() };
            if unsafe { libc::ioctl(fd, libc::TIOCGWINSZ, &mut ws) } == 0
                && ws.ws_col > 0
                && ws.ws_xpixel > 0
            {
                return (u32::from(ws.ws_xpixel) / u32::from(ws.ws_col)).max(1);
            }
        }
    }
    10
}

/// Calculates the approximate height of a single terminal character cell in pixels.
fn term_cell_px_h() -> u32 {
    #[cfg(unix)]
    {
        use std::os::unix::io::AsRawFd;
        for fd in [std::io::stderr().as_raw_fd(), std::io::stdout().as_raw_fd()] {
            let mut ws: libc::winsize = unsafe { std::mem::zeroed() };
            if unsafe { libc::ioctl(fd, libc::TIOCGWINSZ, &mut ws) } == 0
                && ws.ws_row > 0
                && ws.ws_ypixel > 0
            {
                return (u32::from(ws.ws_ypixel) / u32::from(ws.ws_row)).max(1);
            }
        }
    }
    20
}

// ---------------------------------------------------------------------------
// Info helpers
// ---------------------------------------------------------------------------

/// Returns the current OS user, or "victim" as a horror-themed fallback.
fn username() -> String {
    env::var("USER").unwrap_or_else(|_| "victim".to_string())
}

/// Returns the system locale from env variables.
#[must_use]
pub fn linux_locale() -> String {
    env::var("LC_ALL")
        .or_else(|_| env::var("LANG"))
        .unwrap_or_else(|_| "C".to_string())
}

/// Identifies the current shell name (e.g., "zsh", "bash").
fn current_shell() -> String {
    env::var("SHELL").map_or_else(
        |_| "unknown".to_string(),
        |s| s.rsplit('/').next().unwrap_or(&s).to_string(),
    )
}

/// Returns the brand name of the first CPU found.
fn cpu_model(sys: &System) -> String {
    sys.cpus()
        .first()
        .map_or_else(|| "Unknown CPU".to_string(), |c| c.brand().to_string())
}

/// Finds the first non-loopback IPv4 address.
fn local_ip() -> String {
    let networks = Networks::new_with_refreshed_list();
    for (iface, data) in &networks {
        if iface == "lo" || iface.starts_with("lo") {
            continue;
        }
        for addr in data.ip_networks() {
            let ip = addr.addr;
            if ip.is_ipv4() && !ip.to_string().starts_with("127.") {
                return ip.to_string();
            }
        }
    }
    "N/A".to_string()
}

/// Calculates used vs total space for the root (/) partition.
fn disk_usage() -> String {
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        if disk.mount_point().to_str() == Some("/") {
            let total = disk.total_space() / 1024 / 1024 / 1024;
            let free = disk.available_space() / 1024 / 1024 / 1024;
            let used = total.saturating_sub(free);
            return format!("{used} / {total} GiB");
        }
    }
    "N/A".to_string()
}

/// Returns the machine architecture (e.g., `x86_64`).
fn arch() -> String {
    std::env::consts::ARCH.to_string()
}

/// Formats system uptime into a human-readable "Dd Hh Mm" format.
fn uptime_string(uptime: u64) -> String {
    let days = uptime / 86400;
    let hours = (uptime % 86400) / 3600;
    let mins = (uptime % 3600) / 60;
    if days > 0 {
        format!("{days}d {hours}h {mins}m")
    } else {
        format!("{hours}h {mins}m")
    }
}

/// Aggregates all system information into a vector of formatted, colored strings.
#[must_use]
pub fn fetch_lines() -> Vec<String> {
    let refresh = RefreshKind::nothing()
        .with_cpu(CpuRefreshKind::nothing().with_cpu_usage())
        .with_memory(MemoryRefreshKind::nothing().with_ram())
        .with_processes(ProcessRefreshKind::nothing());
    let sys = System::new_with_specifics(refresh);
    let key = |k: &str| format!("{k:<10}").red().bold();

    vec![
        format!(
            "{}@{}",
            username().red().bold(),
            System::host_name().unwrap_or_default().white()
        ),
        "─".repeat(24).white().to_string(),
        format!(
            "{}: {}",
            key("OS"),
            System::name().unwrap_or_default().white()
        ),
        format!(
            "{}: {}",
            key("Kernel"),
            System::kernel_version().unwrap_or_default().white()
        ),
        format!("{}: {}", key("Arch"), arch().white()),
        format!(
            "{}: {}",
            key("Uptime"),
            uptime_string(System::uptime()).white()
        ),
        format!("{}: {}", key("Shell"), current_shell().white()),
        format!("{}: {}", key("CPU"), cpu_model(&sys).white()),
        format!(
            "{}: {:.1}%",
            key("CPU Usage"),
            sys.global_cpu_usage().to_string().yellow()
        ),
        format!(
            "{}: {} / {} MiB",
            key("Memory"),
            (sys.used_memory() >> 20).to_string().yellow(),
            (sys.total_memory() >> 20).to_string().white()
        ),
        format!("{}: {}", key("Disk (/)"), disk_usage().yellow()),
        format!(
            "{}: {}",
            key("Processes"),
            sys.processes().len().to_string().purple()
        ),
        format!("{}: {}", key("Locale"), linux_locale().purple()),
        format!("{}: {}", key("Local IP"), local_ip().cyan()),
    ]
}

// ---------------------------------------------------------------------------
// ANSI-safe truncation
// ---------------------------------------------------------------------------

/// Truncates a string to a specific column width without breaking ANSI escape codes.
/// This prevents "bleeding" colors and ensuring the layout remains intact.
fn truncate_ansi(s: &str, max_cols: usize) -> String {
    if max_cols == 0 {
        return String::new();
    }
    if ansi_width(s) <= max_cols {
        return s.to_string();
    }
    let target = max_cols.saturating_sub(1);
    let mut out = String::new();
    let mut col = 0usize;
    let mut chars = s.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            out.push(ch);
            if chars.peek() == Some(&'[') {
                if let Some(bracket) = chars.next() {
                    out.push(bracket);
                }
                for inner in chars.by_ref() {
                    out.push(inner);
                    if inner.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
            continue;
        }
        let w = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0);
        if col + w > target {
            break;
        }
        out.push(ch);
        col += w;
    }
    out.push_str("\x1b[0m");
    out.push('…');
    out
}

// ---------------------------------------------------------------------------
// Print functions
// ---------------------------------------------------------------------------

/// Prints a simple ASCII horror logo alongside system information.
pub fn print_fetch() {
    let logo = [
        "   _______  ",
        "  /      /| ",
        " /______/ | ",
        " |  XX  | | ",
        " |      |/  ",
        " |______|   ",
    ];
    let info = fetch_lines();
    let max_lines = logo.len().max(info.len());
    println!();
    for i in 0..max_lines {
        let l = *logo.get(i).unwrap_or(&"");
        let r = info.get(i).map_or("", String::as_str);
        println!("  {:<18}   {}", l.bright_red(), r);
    }
    println!();
}

/// Prints two blocks of text side-by-side.
/// If the terminal is too narrow, it stacks them vertically instead.
///
/// # Errors
/// Returns an error if writing to the provided `writer` fails.
pub fn print_with_left_block_writer(
    image_block: &str,
    left_width: usize,
    writer: &mut dyn Write,
    cols: usize,
) -> Result<()> {
    let left_lines: Vec<&str> = image_block.lines().collect();
    let pad = left_width + GAP;
    let right_budget = cols.saturating_sub(pad);
    let info_lines = fetch_lines();
    if right_budget < MIN_RIGHT_BUDGET {
        writeln!(writer)?;
        for line in &left_lines {
            writeln!(writer, "{line}")?;
        }
        writeln!(writer)?;
        for line in &info_lines {
            writeln!(writer, "{line}")?;
        }
        writeln!(writer)?;
        return Ok(());
    }
    let max_lines = left_lines.len().max(info_lines.len());
    writeln!(writer)?;
    for i in 0..max_lines {
        let l = *left_lines.get(i).unwrap_or(&"");
        let r = info_lines.get(i).map_or("", String::as_str);
        write!(writer, "{l:<pad$}")?;
        writeln!(writer, "{}", truncate_ansi(r, right_budget))?;
    }
    writeln!(writer)?;
    Ok(())
}

/// Renders a dynamic image and system info side-by-side.
/// Handles specific scaling logic for Sixel (high-res) vs. Text-based charsets.
///
/// # Errors
/// Returns an error if image rendering or writing fails.
pub fn print_fetch_with_image(
    img: &DynamicImage,
    render: &RenderOptions,
    writer: &mut dyn Write,
) -> Result<()> {
    let cols = term_cols();
    // Ensure the image doesn't take up the whole screen, leaving room for text
    let max_img_cols = u32::try_from(cols.saturating_sub(38).max(20)).unwrap_or(20);
    let (orig_w, orig_h) = (img.width(), img.height());

    if matches!(render.charset(), CharsetMode::Sixel) {
        let target_char_rows: u32 = 30;
        let cell_px_h = term_cell_px_h();
        let cell_px_w = term_cell_px_w();
        let target_px_h = target_char_rows * cell_px_h;
        let scale_h = f64::from(target_px_h) / f64::from(orig_h);
        let scale_w = f64::from(max_img_cols * cell_px_w) / f64::from(orig_w);
        let scale = scale_h.min(scale_w);
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let tw = (f64::from(orig_w) * scale).max(1.0) as u32;
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let th = (f64::from(orig_h) * scale).max(1.0) as u32;
        let img_to_render = if tw != orig_w || th != orig_h {
            img.resize(tw, th, FilterType::Nearest)
        } else {
            img.clone()
        };
        let img_cols = (tw as usize).div_ceil(cell_px_w as usize);
        let text_col = img_cols + GAP + 1;
        let right_budget = cols.saturating_sub(text_col.saturating_sub(1));
        let info_lines = fetch_lines();
        let total_rows = (target_char_rows as usize).max(info_lines.len());
        for _ in 0..total_rows {
            writeln!(writer)?;
        }
        write!(writer, "\x1b[{total_rows}A\x1b[1G")?;
        write!(writer, "\x1b[s")?;
        if right_budget >= MIN_RIGHT_BUDGET {
            for line in &info_lines {
                write!(writer, "\x1b[{text_col}G")?;
                write!(writer, "{}", truncate_ansi(line, right_budget))?;
                write!(writer, "\x1b[1B\x1b[1G")?;
            }
        }
        write!(writer, "\x1b[u")?;
        let mut buf = Vec::new();
        render.render(&img_to_render, &mut buf)?;
        writer.write_all(&buf)?;
        write!(writer, "\x1b[u")?;
        write!(writer, "\x1b[{total_rows}B\x1b[1G")?;
        writeln!(writer)?;
        writer.flush()?;
        return Ok(());
    }

    let mut img_buf = Vec::new();
    let img_cols = match render.charset() {
        CharsetMode::Ascii | CharsetMode::Chinese | CharsetMode::Kanji => {
            let target_cols = 50_u32.min(max_img_cols);
            let aspect = f64::from(orig_h) / f64::from(orig_w);
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let target_rows = ((f64::from(target_cols) * aspect) * 0.5).max(1.0) as u32;
            let ascii_img = img.resize_exact(target_cols, target_rows, FilterType::Nearest);
            render
                .with_width(target_cols)
                .render(&ascii_img, &mut img_buf)?;
            target_cols as usize
        }
        _ => {
            let max_px_w = max_img_cols * 2;
            let scale = (90.0 / f64::from(orig_h)).min(f64::from(max_px_w) / f64::from(orig_w));
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let tw = (f64::from(orig_w) * scale * 0.5).max(1.0) as u32;
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let th = (f64::from(orig_h) * scale).max(1.0) as u32;
            let img_to_render = if tw != orig_w || th != orig_h {
                img.resize(tw, th, FilterType::Nearest)
            } else {
                img.clone()
            };
            render.render(&img_to_render, &mut img_buf)?;
            tw as usize
        }
    };

    let img_str = String::from_utf8_lossy(&img_buf);
    print_with_left_block_writer(&img_str, img_cols, writer, cols)
}
