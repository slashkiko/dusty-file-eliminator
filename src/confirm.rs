use std::io::{self, Write};

fn confirm(message: &str) -> io::Result<bool> {
    let mut input = String::new();
    print!("{} (y/N): ", message);
    io::stdout().flush()?; // 標準出力バッファをフラッシュして、プロンプトが表示されるようにします
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().eq_ignore_ascii_case("y"))
}

pub fn confirm_deletion() -> io::Result<bool> {
    confirm("選択されたファイルを削除しますか？ (y/N): ")
}

pub fn confirm_exclusion() -> io::Result<bool> {
    confirm("選択されたファイルを除外しますか？ (y/N): ")
}
