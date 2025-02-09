// Github Copilotを使用して作ったが未テスト
#[cfg(target_os = "macos")]
pub mod platform {
    use std::process::Command;
    use std::time::Duration;

    pub fn get_system_uptime() -> Duration {
        use std::time::{SystemTime, UNIX_EPOCH};

        let output = Command::new("sysctl")
            .arg("-n")
            .arg("kern.boottime")
            .output()
            .expect("Failed to execute sysctl command");

        let boot_time_str = String::from_utf8_lossy(&output.stdout);
        let boot_time_parts: Vec<&str> = boot_time_str.split_whitespace().collect();
        let boot_time_sec: u64 = boot_time_parts[3]
            .parse()
            .expect("Failed to parse boot time");

        let boot_duration = Duration::new(boot_time_sec, 0);
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        now - boot_duration
    }

    pub fn shutdown() {
        Command::new("sudo")
            .arg("shutdown")
            .arg("-h")
            .arg("now")
            .spawn()
            .unwrap();
    }

    pub fn show_alert(message: &str) {
        Command::new("osascript")
            .arg("-e")
            .arg(format!(
                "display notification \"{}\" with title \"警告\"",
                message
            ))
            .spawn()
            .unwrap();
    }
}
