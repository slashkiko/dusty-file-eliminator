use rand::seq::SliceRandom;
use std::{
    collections::HashSet,
    fs, io,
    path::{Path, PathBuf},
    time::Duration,
};

fn scan_dir(dir: &Path, excluded_dirs: &HashSet<PathBuf>) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let entries = fs::read_dir(dir)?;
    for entry in entries {
        let entry = entry?;
        // フォルダ/ファイルのパスを取得
        let path = entry.path();
        if excluded_dirs.contains(&path) {
            continue;
        }
        files.push(path);
    }

    Ok(files)
}

pub fn select_random_dusty_file(
    dir: &Path,
    excluded_dirs: &HashSet<PathBuf>,
) -> io::Result<Option<PathBuf>> {
    match scan_dir(dir, excluded_dirs) {
        Ok(files) => {
            if files.is_empty() {
                return Ok(None);
                // // ディレクトリが空だった場合は一つ上に戻る
                // if let Some(parent_dir) = dir.parent() {
                //     return select_random_file(parent_dir);
                // } else {
                //     return Err(io::Error::new(
                //         io::ErrorKind::Other,
                //         "Reached the root directory, no files found",
                //     ));
                // }
            }

            // ランダムに1つ選ぶ
            match files.choose(&mut rand::thread_rng()) {
                Some(file) => {
                    if file.is_dir() {
                        println!("{:?}", file);
                        // ディレクトリだった場合は再帰的に呼び出す
                        return select_random_dusty_file(file, excluded_dirs);
                    } else {
                        // ファイルだった場合はそのまま返す
                        let metadata = fs::metadata(file)?;
                        let last_accessed = metadata.accessed()?;
                        let elapsed = last_accessed.elapsed().unwrap_or(Duration::new(0, 0));
                        if elapsed.as_secs() > 365 * 24 * 60 * 60 {
                            return Ok(Some(file.clone()));
                        } else {
                            return Ok(None);
                        }
                    }
                }
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "No files in the directory",
                    ));
                }
            }
        }
        Err(e) => {
            eprintln!("{:?}", e);
            return Err(e);
        }
    }
}
