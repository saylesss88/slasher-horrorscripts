// fetch.rs
use ansi_width::ansi_width;
use colored::Colorize;
use sysinfo::System;

fn username() -> String {
    std::env::var("USER").unwrap_or_else(|_| "victim".to_string())
}

fn hostname() -> String {
    System::host_name().unwrap_or_else(|| "slasher".to_string())
} // sysinfo exposes System::host_name() [web:65]

#[must_use]
pub fn fetch_lines(script_count: usize) -> Vec<String> {
    let os = System::name().unwrap_or_else(|| "Unknown OS".to_string());
    let kernel = System::kernel_version().unwrap_or_else(|| "?.?.?".to_string());
    let uptime = System::uptime();

    let days = uptime / 86400;
    let hours = (uptime % 86400) / 3600;
    let mins = (uptime % 3600) / 60;

    let uptime_str = if days > 0 {
        format!("{days}d {hours}h {mins}m")
    } else {
        format!("{hours}h {mins}m")
    };

    let key = |k: &str| format!("{k:<10}").red().bold();

    vec![
        format!("{}@{}", username().red().bold(), hostname().white()),
        "-----------------------".white().to_string(),
        format!("{}: {}", key("OS"), os.white()),
        format!("{}: {}", key("Kernel"), kernel.white()),
        format!("{}: {}", key("Uptime"), uptime_str.white()),
        format!("{}: {}", key("Victims"), script_count.to_string().yellow()),
        format!("{}: {}", key("Status"), "Haunted".purple()),
    ]
}

pub fn print_fetch(script_count: usize) {
    let logo = [
        "   _______  ",
        "  /      /| ",
        " /______/ | ",
        " |  XX  | | ",
        " |      |/  ",
        " |______|   ",
    ];

    let info = fetch_lines(script_count);
    let max_lines = logo.len().max(info.len());

    println!();
    for i in 0..max_lines {
        let l = *logo.get(i).unwrap_or(&"");
        let r = info.get(i).map_or("", String::as_str);
        println!("  {:<18}   {}", l.bright_red(), r);
    }
    println!();
}

pub fn print_with_left_block(left: &str, script_count: usize) {
    let left_lines: Vec<&str> = left.lines().collect();
    let right_lines = fetch_lines(script_count);

    let left_width = left_lines.iter().map(|l| ansi_width(l)).max().unwrap_or(0);
    // let pad: i32 = 60;
    let pad = left_width + 3;

    let max_lines = left_lines.len().max(right_lines.len());
    println!();

    for i in 0..max_lines {
        let l = *left_lines.get(i).unwrap_or(&"");
        let r = right_lines.get(i).map_or("", String::as_str);

        let l_w = ansi_width(l);
        let spaces = pad.saturating_sub(l_w);

        print!("{l}");
        print!("{:width$}", "", width = spaces);
        println!("{r}");
    }

    println!();
}
