#[cfg(target_os = "windows")]
pub fn setup_autostart() {
    use std::env;
    use std::process::Command;

    let exe_path = match env::current_exe() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to get executable path: {}", e);
            return;
        }
    };

    let reg_key = r#"HKCU\Software\Microsoft\Windows\CurrentVersion\Run"#;
    let name = "GambleGuard";

    if let Err(e) = Command::new("reg")
        .args(&["add", reg_key, "/v", name, "/d", &exe_path.display().to_string(), "/f"])
        .output()
    {
        eprintln!("Failed to write registry key: {}", e);
    } else {
        println!("Windows autostart registry key set.");
    }
}

#[cfg(target_os = "macos")]
pub fn setup_autostart() {
    use std::env;
    use std::fs::{create_dir_all, write};
    use std::path::PathBuf;

    let exe_path = match env::current_exe() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to get executable path: {}", e);
            return;
        }
    };

    let plist_path = match dirs::home_dir() {
        Some(home) => home.join("Library/LaunchAgents/com.gambleguard.agent.plist"),
        None => {
            eprintln!("Cannot determine home directory.");
            return;
        }
    };

    let plist_content = format!(
        r#"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.gambleguard.agent</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <false/>
</dict>
</plist>
"#,
        exe_path.display()
    );

    if let Err(e) = create_dir_all(plist_path.parent().unwrap()) {
        eprintln!("Failed to create LaunchAgents directory: {}", e);
        return;
    }

    if let Err(e) = write(&plist_path, plist_content) {
        eprintln!("Failed to write .plist: {}", e);
    } else {
        println!("MacOS autostart .plist written at:\n{}", plist_path.display());
    }
}

#[cfg(target_os = "linux")]
pub fn setup_autostart() {
    use std::env;
    use std::fs::{create_dir_all, write};

    let exe_path = match env::current_exe() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to get executable path: {}", e);
            return;
        }
    };

    let autostart_path = match dirs::home_dir() {
        Some(home) => home.join(".config/autostart/gambleguard.desktop"),
        None => {
            eprintln!("Cannot determine home directory.");
            return;
        }
    };

    let desktop_entry = format!(
        "[Desktop Entry]
Type=Application
Exec={}
Hidden=false
NoDisplay=true
X-GNOME-Autostart-enabled=true
Name=GambleGuard
Comment=Gambling site blocker",
        exe_path.display()
    );

    if let Err(e) = create_dir_all(autostart_path.parent().unwrap()) {
        eprintln!("Failed to create autostart directory: {}", e);
        return;
    }

    if let Err(e) = write(&autostart_path, desktop_entry) {
        eprintln!("Failed to write autostart file: {}", e);
    } else {
        println!("Linux autostart file written to:\n{}", autostart_path.display());
    }
}
