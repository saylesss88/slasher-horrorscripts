use ansi_width::ansi_width;
use colored::Colorize;
use std::env;
use sysinfo::System;

fn username() -> String {
    std::env::var("USER").unwrap_or_else(|_| "victim".to_string())
}

#[must_use]
pub fn linux_locale() -> String {
    env::var("LC_ALL")
        .or_else(|_| env::var("LANG"))
        .unwrap_or_else(|_| "C".to_string())
}

#[must_use]
pub fn fetch_lines() -> Vec<String> {
    // new_all ensures we get CPU and process lists immediately
    let mut sys = System::new_all();
    sys.refresh_all();

    let os = System::name().unwrap_or_else(|| "Unknown OS".to_string());
    let kernel = System::kernel_version().unwrap_or_else(|| "?.?.?".to_string());
    let uptime = System::uptime();
    let hostname = System::host_name().unwrap_or_else(|| "slasher".to_string());

    // RAM: Convert bytes to MiB for readability
    let total_mem = sys.total_memory() / 1024 / 1024;
    let used_mem = sys.used_memory() / 1024 / 1024;

    // CPU: Get the global usage percentage
    let cpu_usage = sys.global_cpu_usage();

    // Process count: How many "souls" are running on the system
    let process_count = sys.processes().len();

    let days = uptime / 86400;
    let hours = (uptime % 86400) / 3600;
    let mins = (uptime % 3600) / 60;

    let uptime_str = if days > 0 {
        format!("{days}d {hours}h {mins}m")
    } else {
        format!("{hours}h {mins}m")
    };

    let key = |k: &str| format!("{k:<10}").red().bold();

    let locale = linux_locale();

    vec![
        format!("{}@{}", username().red().bold(), hostname.white()),
        "-----------------------".white().to_string(),
        format!("{}: {}", key("OS"), os.white()),
        format!("{}: {}", key("Kernel"), kernel.white()),
        format!("{}: {}", key("Uptime"), uptime_str.white()),
        format!(
            "{}: {} / {} MiB",
            key("Memory"),
            used_mem.to_string().yellow(),
            total_mem.to_string().white()
        ),
        format!("{}: {:.1}%", key("CPU"), cpu_usage.to_string().yellow()),
        format!(
            "{}: {}",
            key("Processes"),
            process_count.to_string().purple()
        ),
        format!("{}: {}", key("User Locale"), locale.purple()),
        // For "Slashers 13" pass script_count:usize to this func
        // format!("{}: {}", key("Slashers"), script_count.to_string().cyan()),
    ]
}
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

pub fn print_with_left_block(left: &str) {
    let left_lines: Vec<&str> = left.lines().collect();
    let right_lines = fetch_lines();

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
