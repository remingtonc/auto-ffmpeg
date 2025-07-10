use std::{
    fs,
    path::Path,
    process::Command,
    thread::sleep,
    time::Duration,
};

use walkdir::WalkDir;
use regex::Regex;
use pathdiff::diff_paths;

const SOURCE_PATH: &str = "/datalake/staging/transcode/source";
const TARGET_PATH: &str = "/datalake/staging/transcode/target";

fn main() {
    let source_dir = Path::new(SOURCE_PATH);
    let target_dir = Path::new(TARGET_PATH);
    let skip_regex = Regex::new(r"^title_[a-zA-Z0-9]+\.mkv$").unwrap();

    for entry in WalkDir::new(source_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "mkv"))
    {
        let full_path = entry.path();
        let filename = match full_path.file_name().and_then(|s| s.to_str()) {
            Some(name) => name,
            None => continue,
        };

        // Skip special titles
        if skip_regex.is_match(filename) {
            println!("Skipping special title file: {}", filename);
            continue;
        }

        // Compute relative path
        let relative_path = match diff_paths(full_path, source_dir) {
            Some(path) => path,
            None => continue,
        };

        let relative_dir = relative_path.parent().unwrap_or_else(|| Path::new(""));

        // Create directory in target if needed
        if !relative_dir.as_os_str().is_empty() && relative_dir != Path::new(".") {
            let target_subdir = target_dir.join(relative_dir);
            if let Err(e) = fs::create_dir_all(&target_subdir) {
                eprintln!("Failed to create directory {:?}: {}", target_subdir, e);
                continue;
            }
        }

        // Container-visible paths
        let container_source = Path::new("/data/source").join(&relative_path);
        let container_target = Path::new("/data/target").join(&relative_path);

        println!("SOURCE: {}", full_path.display());
        println!("TARGET: {}", target_dir.join(&relative_path).display());

        // Build podman command
        let status = Command::new("podman")
            .args([
                "run",
                "--rm",
                "--group-add", "keep-groups",
                "--device=/dev/dri:/dev/dri",
                "-v", &format!("{SOURCE_PATH}:/data/source"),
                "-v", &format!("{TARGET_PATH}:/data/target"),
                "auto-ffmpeg-ffmpeg-qs:latest",
                "-init_hw_device", "qsv=qsv:hw",
                "-hwaccel", "qsv",
                "-filter_hw_device", "qsv",
                "-hwaccel_output_format", "qsv",
                "-i", &container_source.to_string_lossy(),
                "-c:v", "hevc_qsv",
                "-vf", "hwupload=extra_hw_frames=64,format=qsv",
                "-preset", "slow",
                "-global_quality", "21",
                "-look_ahead", "1",
                "-low_power", "off",
                "-c:a", "libfdk_aac",
                "-b:a", "384k",
                "-c:s", "copy",
                &container_target.to_string_lossy(),
            ])
            .status();

        if let Err(e) = status {
            eprintln!("Podman execution failed: {}", e);
        }

        sleep(Duration::from_secs(10));
    }
}
