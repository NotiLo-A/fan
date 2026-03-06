use std::fs;
use std::process::{Command, exit};
use std::env;
use std::thread;
use std::time::Duration;

fn check_thinkpad() {
    if !std::path::Path::new("/proc/acpi/ibm/fan").exists() {
        eprintln!("Are you using a Thinkpad?");
        exit(1);
    }
}

fn is_root() -> bool {
    env::var("USER").map(|u| u == "root").unwrap_or(false)
        || fs::read_to_string("/proc/self/status")
            .unwrap_or_default()
            .lines()
            .find(|l| l.starts_with("Uid:"))
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|uid| uid.parse::<u32>().ok())
            .map(|uid| uid == 0)
            .unwrap_or(false)
}

fn fan_cmd(cmd: &str) {
    if !is_root() {
        let args: Vec<String> = env::args().collect();
        let status = Command::new("sudo")
            .args(&args)
            .status()
            .expect("Failed to run sudo");
        exit(status.code().unwrap_or(1));
    }

    let content = match cmd {
        "enable" | "disable" => cmd.to_string(),
        _ => format!("level {}", cmd),
    };

    fs::write("/proc/acpi/ibm/fan", content)
        .expect("Failed to write to /proc/acpi/ibm/fan");
}

fn print_help() {
    println!("fan is a tool for controlling fan speed.\n");
    println!("Usage:\n");
    println!("        fan <value>\n");
    println!("The values are:\n");
    println!("        help            show this help message");
    println!("        stat / info     show fan stat");
    println!("        w               watch fan stat (updates every 1s)");
    println!("        on              turn the fan on");
    println!("        off             turn the fan off");
    println!("        auto            set fan to auto");
    println!("        max             set fan to maximum speed");
    println!("        min             set fan to minimum speed");
    println!("        1-7             set fan speed to level 1-7");
}

fn fan_status() {
    let content = fs::read_to_string("/proc/acpi/ibm/fan")
        .expect("Failed to read /proc/acpi/ibm/fan");

    for line in content.lines() {
        if line.starts_with("status:")
            || line.starts_with("speed:")
            || line.starts_with("level:")
        {
            println!("{}", line);
        }
    }
}

fn fan_watch() {
    loop {
        print!("\x1B[2J\x1B[H");
        fan_status();
        thread::sleep(Duration::from_secs(1));
    }
}

fn run(value: &str) {
    match value {
        "" | "help" => print_help(),
        "stat" | "info" => fan_status(),
        "w"    => fan_watch(),
        "on"   => fan_cmd("enable"),
        "off"  => fan_cmd("disable"),
        "max"  => fan_cmd("full-speed"),
        "auto" => fan_cmd("auto"),
        "min"  => fan_cmd("1"),
        "1"    => fan_cmd("1"),
        "2"    => fan_cmd("2"),
        "3"    => fan_cmd("3"),
        "4"    => fan_cmd("4"),
        "5"    => fan_cmd("5"),
        "6"    => fan_cmd("6"),
        "7"    => fan_cmd("7"),
        _      => {
            eprintln!("fan: {}: unknown command", value);
            eprintln!("Run 'fan help' for usage.");
        }
    }
}

fn main() {
    check_thinkpad();
    let args: Vec<String> = env::args().collect();
    let value = args.get(1).map(|s| s.as_str()).unwrap_or("");
    run(value);
}