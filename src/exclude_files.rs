use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufWriter};
use std::io::{BufReader, Write};
use std::path::PathBuf;

pub fn append_to_file(file_path: &str, data: &str) -> io::Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    let mut buffered_writer = BufWriter::new(file);
    writeln!(buffered_writer, "{}", data)?;
    buffered_writer.flush()?; // バッファリングされたデータをフラッシュする
    Ok(())
}

pub fn read_from_file(file_path: &str) -> io::Result<HashSet<PathBuf>> {
    let file = OpenOptions::new().read(true).open(file_path)?;
    let buffered_reader = BufReader::new(file);

    let mut lines = HashSet::new();
    for line in buffered_reader.lines() {
        match line {
            Ok(l) => {
                lines.insert(PathBuf::from(l));
            }
            Err(e) => return Err(e),
        }
    }
    Ok(lines)
}
