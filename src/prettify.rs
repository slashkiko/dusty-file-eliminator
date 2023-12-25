use std::{error::Error, fs, path::PathBuf, time::SystemTime};

use colored::{ColoredString, Colorize};
const SMALL: u64 = 1024 * 1024;
const SMALL_NEXT: u64 = SMALL + 1;
const MEDIUM: u64 = 1024 * 1024 * 500;

pub fn pretty_print_bytes(bytes: u64) -> ColoredString {
    let units = ["B", "KB", "MB", "GB", "TB"];
    if bytes == 0 {
        return format!("0 {}", units[0]).green();
    }
    let color = match bytes {
        0..=SMALL => "green",
        SMALL_NEXT..=MEDIUM => "yellow",
        _ => "red",
    };
    let digit_groups = (bytes as f64).log10() as i32 / 3;
    format!(
        "{:.1}{}",
        (bytes as f64) / 10f64.powi(digit_groups * 3),
        units[digit_groups as usize]
    )
    .color(color)
}

pub fn pretty_print_time(system_time: SystemTime) -> String {
    let datetime = chrono::DateTime::<chrono::Utc>::from(system_time);
    let formatted = datetime.format("%Y-%m-%d %H:%M").to_string();
    formatted
}

pub fn print_file_detail(file_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let metadata = fs::metadata(&file_path)?;
    println!(
        "{} {} {}",
        file_path.to_str().ok_or("Invalid file path")?,
        pretty_print_bytes(metadata.len()),
        pretty_print_time(metadata.accessed()?)
    );
    Ok(())
}
