use colored::Colorize;
use sysinfo::System; // Keep SystemExt if you need other traits, but not for these calls

pub fn print_fetch(script_count: usize) {
    // The Horror Theme ASCII Art
    let logo = [
        "   _______  ",
        "  /      /| ",
        " /______/ | ",
        " |  XX  | | ",
        " |      |/  ",
        " |______|   ",
    ];

    // System Info (Using Static Functions)
    let username = std::env::var("USER").unwrap_or_else(|_| "victim".to_string());

    // FIX: Use System::Function() instead of sys.function()
    let os = System::name().unwrap_or_else(|| "Unknown OS".to_string());
    let kernel = System::kernel_version().unwrap_or_else(|| "?.?.?".to_string());
    let uptime = System::uptime(); // Returns u64 seconds directly

    let days = uptime / 86400;
    let hours = (uptime % 86400) / 3600;
    let mins = (uptime % 3600) / 60;

    let uptime_str = if days > 0 {
        format!("{days}d {hours}h {mins}m")
    } else {
        format!("{hours}h {mins}m")
    };
    // Helper to color and pad keys (e.g. "OS" -> "OS      ")
    // Adjust the '10' to be as wide as your longest label
    let key = |k: &str| format!("{k:<10}").red().bold();

    // The Data Columns
    let info = [
        format!("{}@{}", username.red().bold(), "slasher".white()),
        format!("{}", "---------------------------".white()), // Longer separator
        format!("{}: {}", key("OS"), os.white()),
        format!("{}: {}", key("Kernel"), kernel.white()),
        format!("{}: {}", key("Uptime"), uptime_str.white()),
        format!("{}: {}", key("Victims"), script_count.to_string().yellow()),
        format!("{}: {}", key("Status"), "Haunted 👻".purple()),
    ];
    let max_lines = std::cmp::max(logo.len(), info.len());
    println!();

    for i in 0..max_lines {
        let logo_str = logo.get(i).unwrap_or(&"");
        let info_str = info.get(i).map_or("", String::as_str);

        // "{:<18}" means "Left align, fill with spaces up to 18 chars"
        let logo_padded = format!("{logo_str:<18}");

        // NOW apply color to the padded block
        println!("  {}   {}", logo_padded.bright_red(), info_str);
    }
    println!();

    // Print Side-by-Side
    // let max_lines = std::cmp::max(logo.len(), info.len());
    // println!();
    // for i in 0..max_lines {
    //     let logo_line = logo.get(i).unwrap_or(&"            ");
    //     let info_line = info.get(i).map(|s| s.as_str()).unwrap_or("");

    //     println!("  {}   {}", logo_line.bright_red(), info_line);
    // }
    // println!();
}
