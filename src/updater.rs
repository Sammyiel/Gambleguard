use std::fs;
use std::path::PathBuf;
use std::error::Error;
use std::env;

fn get_fallback_path() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        let appdata = env::var("APPDATA").unwrap_or_else(|_| "C:\\ProgramData".into());
        return PathBuf::from(format!(
            "{}\\GambleGuard\\gambleguard_domain_blocklist.txt",
            appdata
        ));
    }

    #[cfg(target_os = "macos")]
    {
        return PathBuf::from("/usr/local/etc/gambleguard_domain_blocklist.txt");
    }

    #[cfg(target_os = "linux")]
    {
        return PathBuf::from("/etc/gambleguard/gambleguard_domain_blocklist.txt");
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        panic!("Unsupported OS for fallback blocklist path");
    }
}

pub fn fetch_blocklist() -> Result<Vec<String>, Box<dyn Error>> {
    let remote_url = "https://raw.githubusercontent.com/Zacwat7/gambleguard-block-list/refs/heads/main/blocks.txt";  // Replace with your real URL
    let fallback_path = get_fallback_path();

    let contents = match reqwest::blocking::get(remote_url) {
        Ok(resp) if resp.status().is_success() => {
            log::info!("Fetched blocklist from remote");
            resp.text()?
        }
        Err(e) => {
            log::warn!("Failed to fetch remote blocklist: {}. Falling back.", e);
            fs::read_to_string(&fallback_path)?
        }
        Ok(resp) => {
            log::warn!(
                "Received non-success status {} from remote. Falling back.",
                resp.status()
            );
            fs::read_to_string(&fallback_path)?
        }
    };

    let lines: Vec<String> = contents
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty() && !s.starts_with('#'))
        .collect();

    Ok(lines)
}
