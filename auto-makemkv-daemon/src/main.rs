use std::{fs, process::Command, thread, time::Duration};
use std::path::{Path, PathBuf};
use serde::Deserialize;
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use std::fs::File;
use std::io::{self, Read};
use std::time::SystemTime;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use signal_hook::consts::TERM_SIGNALS;
use signal_hook::iterator::Signals;

#[derive(Debug, Deserialize)]
struct Config {
    target_path: String,
    optical_drive_path: String,
    check_interval_secs: u64,
    smtp_host: String,
    smtp_username: String,
    smtp_password: String,
    email_from: String,
    email_to: String,
}

fn load_config() -> Config {
    let mut file = File::open("config.toml").expect("Unable to open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read config file");
    toml::from_str(&contents).expect("Invalid config format")
}

fn find_sg_for_sr(sr_dev: &Path) -> Option<PathBuf> {
    let sr_name = sr_dev.file_name()?.to_str()?;

    for entry in fs::read_dir("/sys/class/scsi_generic/").ok()? {
        let path = entry.ok()?.path();
        let dev_dir = path.join("device/block");
        let block_devs = fs::read_dir(dev_dir).ok()?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>().ok()?;
        if block_devs.len() > 1 {
            // We cannot handle multiple entries today, fail out.
            eprintln!("Multiple entries found for SCSI generic device: {}", sr_name);
            return None;
        } else if block_devs.is_empty() {
            // No block devices found, continue to next entry.
            continue;
        }
        if block_devs[0].file_name()?.to_str()? == sr_name {
            return Some(PathBuf::from(format!("/dev/{}", path.file_name()?.to_string_lossy())));
        }
    }

    None
}

fn poll_for_disc_insertions(config: &Config, stop_flag: Arc<AtomicBool>) {
    let mut last_checked = SystemTime::now();
    let mut first_iteration = true;

    while !stop_flag.load(Ordering::SeqCst) {
        if let Ok(metadata) = fs::metadata(&config.optical_drive_path) {
            if metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH) > last_checked || first_iteration {
                println!("Disc appears to be inserted.");
                if !rip_already_exists(&config.target_path) {
                    run_makemkv(&config.target_path, &config.optical_drive_path);
                    send_email_notification(&config, "MakeMKV Finished", "Disc rip completed.");
                } else {
                    println!("Rip already exists, skipping.");
                }
                last_checked = SystemTime::now();
                first_iteration = false;
            }
        }
        thread::sleep(Duration::from_secs(config.check_interval_secs));
    }

    println!("Shutdown signal received. Exiting poll loop.");
}

fn rip_already_exists(dir: &str) -> bool {
    let patterns = vec!["title_00", "A1_01"];
    fs::read_dir(dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .any(|entry| {
            let fname = entry.file_name().to_string_lossy().to_string();
            patterns.iter().any(|p| fname.starts_with(p))
        })
}

fn run_makemkv(dest: &str, optical_drive: &str) {
    let sr_dev = fs::canonicalize(optical_drive).expect("Failed to resolve optical drive path");
    let sg_dev = find_sg_for_sr(&sr_dev).expect("No matching sg device found");
    println!("Using source device: {}", sr_dev.display());
    println!("Using SCSI generic device: {}", sg_dev.display());

    let status = Command::new("podman")
        .args(&["run", "-it", "--rm",
            "--group-add", "keep-groups",
            "--device=/dev/dri:/dev/dri",
            &format!("--device={}:{}", sr_dev.display(), sr_dev.display()),
            &format!("--device={}:{}", sg_dev.display(), sg_dev.display()),
            "-v", &format!("{}:/data/target", dest),
            "localhost/auto-ffmpeg-makemkv:1.18.1",
            "--robot", "--decrypt", "--cache=1024", "--minlength=600",
            "mkv", "disc:0", "all", "/data/target"])
        .status()
        .expect("failed to execute MakeMKV");

    println!("MakeMKV finished with status: {}", status);
}

fn send_email_notification(config: &Config, subject: &str, body: &str) {
    let email = Message::builder()
        .from(config.email_from.parse().unwrap())
        .to(config.email_to.parse().unwrap())
        .subject(subject)
        .body(body.to_string())
        .unwrap();

    let creds = Credentials::new(config.smtp_username.clone(), config.smtp_password.clone());

    let mailer = SmtpTransport::relay(&config.smtp_host)
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => eprintln!("Failed to send email: {:?}", e),
    }
}

fn main() {
    let config = load_config();
    println!("Daemon started. Polling for disc insertions...");

    let stop_flag = Arc::new(AtomicBool::new(false));
    let sf_clone = stop_flag.clone();

    let mut signals = Signals::new(TERM_SIGNALS).expect("Failed to register signals");
    let signals_thread = thread::spawn(move || {
        for _ in signals.forever() {
            sf_clone.store(true, Ordering::SeqCst);
            break;
        }
    });

    poll_for_disc_insertions(&config, stop_flag);
    signals_thread.join().expect("Signals thread panicked");
    println!("Daemon stopped.");
}
