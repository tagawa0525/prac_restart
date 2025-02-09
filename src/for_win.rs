#[cfg(target_os = "windows")]
pub mod platform {
    use std::process::Command;
    use std::time::Duration;

    pub fn get_system_uptime() -> Duration {
        use winapi::um::sysinfoapi::GetTickCount64;

        let ticks = unsafe { GetTickCount64() };
        Duration::from_millis(ticks)
    }

    #[test]
    fn test_get_system_uptime() {
        let uptime = get_system_uptime();
        println!("{:?}", uptime);
        assert!(uptime.as_secs() > 0);
    }

    pub fn shutdown() {
        // デバッグ時ににシャットダウンすると面倒なので、コメントアウト
        // Command::new("shutdown")
        //     .arg("/s")
        //     .arg("/f")
        //     .arg("/t")
        //     .arg("0")
        //     .spawn()
        //     .unwrap();
        println!("shutdown!");
    }
    #[test]
    fn test_shutdown() {
        shutdown();
    }

    pub fn show_alert(message: &str) {
        Command::new("msg").arg("*").arg(message).spawn().unwrap();
    }
}
