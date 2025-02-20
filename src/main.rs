#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::{Duration, Instant};

#[cfg(target_os = "windows")]
mod for_win;
#[cfg(target_os = "windows")]
use crate::for_win::platform;

#[cfg(target_os = "macos")]
mod for_mac;
#[cfg(target_os = "macos")]
use crate::for_mac::platform;

const MIN: u64 = 6; // 本当は60 sec
const HOUR: u64 = MIN * MIN;
const COUNTDOWN_MIN: u64 = 5;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let total_time = Duration::from_secs(6 * HOUR); // 仕様では72時間だが、デバッグのため短くしている。
    let alerts = [
        // total_time - Duration::from_secs(24 * HOUR),
        // total_time - Duration::from_secs(6 * HOUR),
        total_time - Duration::from_secs(3 * HOUR),
        total_time - Duration::from_secs(1 * HOUR),
        // 本当は30 MINとしたいが、デバッグ時の加速の関係で分数にする。
        total_time - Duration::from_secs(30 / 60 * HOUR),
        // total_time - Duration::from_secs(15 / 60 * HOUR),
        // total_time - Duration::from_secs(10 / 60 * HOUR),
        // total_time - Duration::from_secs(5 / 60 * HOUR),
    ];
    // println!("{:?}", alerts);

    let start_time = Instant::now() - platform::get_system_uptime();
    for alert_time in alerts.iter() {
        let elasped_time = start_time.elapsed();
        if *alert_time < (elasped_time + Duration::from_secs(1)) {
            continue;
        }

        let sleeped_time = *alert_time - elasped_time;
        std::thread::sleep(sleeped_time);

        let h = alert_time.as_secs() / HOUR;
        let m: u64 = (alert_time.as_secs() % HOUR) / MIN;
        platform::show_alert(&format!(
            "PC起動から{}時間{}分後にシャットダウンします。",
            h, m
        ));
    }

    for i in (1..COUNTDOWN_MIN + 1).rev() {
        platform::show_alert(&format!("強制シャットダウンまで {} 分", i));
        std::thread::sleep(Duration::from_secs(1 * MIN));
    }

    platform::shutdown()?;
    Ok(())
}
