#[cfg(target_os = "linux")]
pub mod platform_paths {
    pub const HOSTS_FILE: &str = "/etc/hosts";
    pub const BACKUP_FILE: &str = "/etc/hosts.gambleguard.bak";
    pub const GAMBLEGUARD_DIR: &str = "/etc/gambleguard";
    pub const LOG_PATH: &str = "/etc/gambleguard/logs.txt";
}

#[cfg(target_os = "macos")]
pub mod platform_paths {
    pub const HOSTS_FILE: &str = "/etc/hosts";
    pub const BACKUP_FILE: &str = "/etc/hosts.gambleguard.bak";
    pub const GAMBLEGUARD_DIR: &str = "/Library/Application Support/GambleGuard";
    pub const LOG_PATH: &str = "/Library/Application Support/GambleGuard/logs.txt";
}

#[cfg(target_os = "windows")]
pub mod platform_paths {
    pub const HOSTS_FILE: &str = "C:\\Windows\\System32\\drivers\\etc\\hosts";
    pub const BACKUP_FILE: &str = "C:\\ProgramData\\GambleGuard\\hosts.gambleguard.bak";
    pub const GAMBLEGUARD_DIR: &str = "C:\\ProgramData\\GambleGuard";
    pub const LOG_PATH: &str = "C:\\ProgramData\\GambleGuard\\logs.txt";
}
