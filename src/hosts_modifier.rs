use std::fs::{read_to_string, write};
use std::path::Path;
use std::process::Command;

use crate::web::append_log;
use crate::paths::platform_paths::*;

const START_TAG: &str = "# BEGIN GAMBLEGUARD";
const END_TAG: &str = "# END GAMBLEGUARD";

pub fn apply_blocklist(domains: &[String]) {
    if !is_root() {
        eprintln!("GambleGuard must be run as root or with sufficient privileges.");
        return;
    }

    // Backup the hosts file once
    if !Path::new(BACKUP_FILE).exists() {
        if let Err(e) = std::fs::copy(HOSTS_FILE, BACKUP_FILE) {
            eprintln!("Failed to backup hosts file: {}", e);
        } else {
            println!("Backed up original hosts file to {}", BACKUP_FILE);
        }
    }

    let mut base_hosts = match read_to_string(HOSTS_FILE) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to read hosts file: {}", e);
            return;
        }
    };

    // Remove old GambleGuard block if it exists
    if let Some(start) = base_hosts.find(START_TAG) {
        if let Some(end) = base_hosts.find(END_TAG) {
            base_hosts.replace_range(start..end + END_TAG.len(), "");
        }
    }

    let mut block_section = String::new();
    block_section.push_str(&format!("\n{}\n", START_TAG));

    for domain in domains {
        block_section.push_str(&format!("127.0.0.1 {}\n", domain));
        block_section.push_str(&format!("127.0.0.1 www.{}\n", domain));
        block_section.push_str(&format!("::1 {}\n", domain));
        block_section.push_str(&format!("::1 www.{}\n", domain));

        let _ = append_log(&format!("Blocked domain: {}", domain));
    }

    block_section.push_str(&format!("{}\n", END_TAG));

    let new_hosts = format!("{}{}", base_hosts.trim_end(), block_section);
    if let Err(e) = write(HOSTS_FILE, new_hosts) {
        eprintln!("Failed to write to hosts file: {}", e);
    } else {
        println!("Blocklist successfully applied to {}", HOSTS_FILE);
    }
}

fn is_root() -> bool {
    #[cfg(target_os = "windows")]
    {
        true
    }

    #[cfg(not(target_os = "windows"))]
    {
        match Command::new("id").arg("-u").output() {
            Ok(output) => {
                let uid = String::from_utf8_lossy(&output.stdout);
                uid.trim() == "0"
            }
            Err(_) => false,
        }
    }
}
