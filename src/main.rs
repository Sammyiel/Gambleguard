mod logger;
mod updater;
mod blocklist;
mod hosts_modifier;
mod web;
mod paths;

#[cfg(target_os = "windows")]
mod platform;
#[cfg(target_os = "macos")]
mod platform;
#[cfg(target_os = "linux")]
mod platform;

use crate::logger::init_logger;
use crate::updater::fetch_blocklist;
use crate::blocklist::parse_blocklist;
use crate::hosts_modifier::apply_blocklist;
use crate::paths::platform_paths::*;
use std::{fs, thread, time::Duration};
use std::env;
use std::path::Path;

fn ensure_gambleguard_dir_exists() {
    let dir = Path::new(GAMBLEGUARD_DIR);
    if !dir.exists() {
        if let Err(e) = fs::create_dir_all(dir) {
            eprintln!("Failed to create log directory: {}", e);
        }
    }
}

fn apply_latest_blocklist() {
    match fetch_blocklist() {
        Ok(domains) => {
            log::info!("Fetched {} domains", domains.len());
            let parsed = parse_blocklist(&domains);
            log::info!("Parsed {} entries", parsed.len());

            #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
            apply_blocklist(&parsed);
        }
        Err(e) => {
            log::error!("Failed to fetch blocklist: {}", e);
        }
    }
}

#[cfg(target_os = "linux")]
fn uninstall_os() {
    use std::process::Command;

    println!("Uninstalling GambleGuard...");
    log::info!("Uninstalling GambleGuard on Linux...");

    let _ = Command::new("systemctl").args(["stop", "gambleguard.service"]).output();
    let _ = Command::new("systemctl").args(["disable", "gambleguard.service"]).output();
    let _ = fs::remove_file("/usr/local/bin/gambleguard");
    let _ = fs::remove_file("/etc/systemd/system/gambleguard.service");
    let _ = fs::remove_dir_all(GAMBLEGUARD_DIR);
    let _ = Command::new("systemctl").arg("daemon-reload").output();

    cleanup_hosts_file();

    println!("GambleGuard successfully uninstalled.");
    log::info!("GambleGuard successfully uninstalled.");
}

#[cfg(target_os = "macos")]
fn uninstall_os() {
    use std::process::Command;

    println!("Uninstalling GambleGuard...");
    log::info!("Uninstalling GambleGuard on macOS...");

    if let Some(plist) = dirs::home_dir().map(|p| p.join("Library/LaunchAgents/com.gambleguard.agent.plist")) {
        let _ = Command::new("launchctl")
            .args(["unload", plist.to_str().unwrap_or("")])
            .output();
        let _ = fs::remove_file(plist);
    }

    let _ = fs::remove_file("/usr/local/bin/gambleguard");
    let _ = fs::remove_dir_all(GAMBLEGUARD_DIR);

    cleanup_hosts_file();

    println!("GambleGuard successfully uninstalled.");
    log::info!("GambleGuard successfully uninstalled.");
}

#[cfg(target_os = "windows")]
fn uninstall_os() {
    use std::process::Command;

    println!("Uninstalling GambleGuard...");
    log::info!("Uninstalling GambleGuard on Windows...");

    let _ = Command::new("reg")
        .args([
            "delete",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Run",
            "/v",
            "GambleGuard",
            "/f",
        ])
        .output();

    let _ = fs::remove_file(r"C:\Program Files\GambleGuard\gambleguard.exe");
    let _ = fs::remove_dir_all(r"C:\Program Files\GambleGuard");

    cleanup_hosts_file();

    println!("GambleGuard successfully uninstalled.");
    log::info!("GambleGuard successfully uninstalled.");
}

fn cleanup_hosts_file() {
    let original = match fs::read_to_string(HOSTS_FILE) {
        Ok(data) => data,
        Err(_) => return,
    };

    let mut inside_block = false;
    let cleaned: Vec<String> = original
        .lines()
        .filter_map(|line| {
            if line.trim() == "# BEGIN GAMBLEGUARD" {
                inside_block = true;
                return None;
            }
            if line.trim() == "# END GAMBLEGUARD" {
                inside_block = false;
                return None;
            }
            if inside_block {
                return None;
            }
            Some(line.to_string())
        })
        .collect();

    let _ = fs::write(HOSTS_FILE, cleaned.join("\n"));
    log::info!("Cleaned hosts file from GambleGuard entries.");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    ensure_gambleguard_dir_exists(); // Ensure log and backup folder exists
    init_logger();
    log::info!("GambleGuard agent starting...");

    if args.contains(&"--uninstall".to_string()) {
        uninstall_os();
        return;
    }

    thread::spawn(|| {
        web::start_warning_server();
    });

    platform::setup_autostart();
    apply_latest_blocklist();

    thread::spawn(|| loop {
        thread::sleep(Duration::from_secs(60 * 60 * 24)); // Daily refresh
        log::info!("Auto-refreshing blocklist...");
        apply_latest_blocklist();
    });

    // Keep the agent alive forever
    loop {
        thread::sleep(Duration::from_secs(60 * 60));
    }
}

