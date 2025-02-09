use std::thread;
use std::time::{Duration, Instant};

#[cfg(target_os = "windows")]
mod for_win;
#[cfg(target_os = "windows")]
use crate::for_win::platform;

#[cfg(target_os = "macos")]
mod for_mac;
#[cfg(target_os = "macos")]
use crate::for_mac::platform;

const MIN: u64 = 2; // 本当は60 sec
const HOUR: u64 = MIN * MIN;
fn main() {
    let start_time = Instant::now();
    let total_time = Duration::from_secs(6 * HOUR); // 仕様では72時間だが、デバッグのため短くしている。
    let alerts = [
        // total_time - Duration::from_secs(24 * HOUR),
        // total_time - Duration::from_secs(6 * HOUR),
        total_time - Duration::from_secs(3 * HOUR),
        total_time - Duration::from_secs(HOUR),
        // 本当は30 MINとしたいが、デバッグ時の加速の関係で分数にする。
        total_time - Duration::from_secs(30 / 60 * HOUR),
        // total_time - Duration::from_secs(15 / 60 * HOUR),
        // total_time - Duration::from_secs(10 / 60 * HOUR),
        // total_time - Duration::from_secs(5 / 60 * HOUR),
    ];
    println!("{:?}", alerts);

    for alert_time in alerts.iter() {
        let elasped_time = start_time.elapsed();
        if *alert_time < elasped_time {
            continue;
        }
        thread::sleep(*alert_time - elasped_time);

        let h = alert_time.as_secs() / HOUR;
        let m: u64 = (alert_time.as_secs() % HOUR) / MIN;
        platform::show_alert(&format!(
            "PC起動から{}時間{}分後にシャットダウンします。",
            h, m
        ));
    }

    let countdown_time = Duration::from_secs(5);
    for i in (1..=countdown_time.as_secs()).rev() {
        platform::show_alert(&format!("強制シャットダウンまで {} 分", i));
        thread::sleep(Duration::from_secs(1 * MIN));
    }

    platform::shutdown();
}
