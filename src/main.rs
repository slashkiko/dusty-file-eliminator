use std::{collections::HashSet, env, error::Error, path::PathBuf};
use trash;
mod prettify;
use prettify::print_file_detail;
mod confirm;
mod exclude_files;
mod select_dusty_file;
use confirm::{confirm_deletion, confirm_exclusion};
use exclude_files::{append_to_file, read_from_file};
use select_dusty_file::select_random_dusty_file;

// TODO 再帰やめる？
// TODO 最終アクセス日？最終更新日？
// TODO オプション追加する
// dry-run
// verbose
// force

fn main() -> Result<(), Box<dyn Error>> {
    let home = env::var("HOME")?;
    let default_excluded_dirs = HashSet::from([
        PathBuf::from(format!("{}/Library", home)),
        PathBuf::from(format!("{}/.Trash", home)),
        PathBuf::from(format!("{}/Pictures/Photos Library.photoslibrary", home)),
    ]);
    let user_excluded_dirs =
        read_from_file(&format!("{}/.dfe/excluded_files", home)).unwrap_or(HashSet::new());
    let excluded_dirs = default_excluded_dirs
        .union(&user_excluded_dirs)
        .cloned()
        .collect();

    let file_path = PathBuf::from(&home);
    let target_file = loop {
        match select_random_dusty_file(&file_path, &excluded_dirs) {
            Ok(Some(file)) => {
                break Ok(file);
            }
            Ok(None) => {
                println!("return root");
            }
            Err(e) => {
                eprintln!("エラーが発生しました: {}", e);
                break Err(e);
            }
        }
    };

    match target_file {
        Ok(file) => {
            print_file_detail(&file)?;
            if confirm_deletion()? {
                // match fs::remove_file(&file) {
                trash::delete(&file)
                    .map_err(|e| format!("ファイルの削除中にエラーが発生しました: {}", e))?;
                println!("{:?} が削除されました。", file)
            } else {
                println!("削除はキャンセルされました。");
                if confirm_exclusion()? {
                    // match fs::remove_file(&file) {
                    append_to_file(
                        &format!("{}/.dfe/excluded_files", home),
                        &file.to_string_lossy(),
                    )
                    .map_err(|e| {
                        format!("ファイルを除外リストに追加中にエラーが発生しました: {}", e)
                    })?;
                    println!("{:?} が除外されました。", file);
                }
            }
        }
        Err(_) => {}
    }
    Ok(())
}
