use std::io::Write;
///
///
/// 日本語パス名への対応が未確認です。
///

/// 標準入力から一行の入力を得ます。
fn input_text() -> String {
	let mut line = String::new();
	let ret = std::io::stdin().read_line(&mut line);
	if ret.is_err() {
		println!("[ERROR] {}", ret.err().unwrap());
		return String::new();
	}
	if ret.unwrap() == 0 {
		return String::new();
	}
	return (*line.trim()).to_string();
}

/// プロンプトを表示し、YES/NO の応答を読み取ります。
fn confirm() -> std::result::Result<bool, Box<dyn std::error::Error>> {
	print!("(y/N)> ");
	std::io::stdout().flush().unwrap();
	let line = input_text().to_uppercase();
	if line == "Y" {
		return Ok(true);
	}
	if line == "YES" {
		return Ok(true);
	}
	return Ok(false);
}

/// 二つのファイルが同一かどうかを調べます。
fn seems_to_be_same(source_path: &std::path::Path, destination_path: &std::path::Path) -> std::result::Result<bool, Box<dyn std::error::Error>> {
	// 元
	let left = std::fs::metadata(source_path)?;
	// 先
	let right_attribute = std::fs::metadata(destination_path);
	if right_attribute.is_err() {
		// 先のファイルがみつからないようです。
		return Ok(false);
	}
	let right = right_attribute?;
	// サイズとタイムスタンプが同じなら同じとみなします。
	if left.len() == right.len() {
		if left.modified()? == right.modified()? {
			return Ok(true);
		}
	}
	// 中身を比較
	let result = file_diff::diff(source_path.to_str().unwrap(), destination_path.to_str().unwrap());
	return Ok(result);
}

/// ファイルごとに呼びだされるハンドラーです。
fn file_handler(source_path: &str, destination_path: &str) -> std::result::Result<i32, Box<dyn std::error::Error>> {
	if seems_to_be_same(std::path::Path::new(source_path), std::path::Path::new(destination_path))? {
		return Ok(0);
	}
	println!("ファイル {} を上書きしますか？", destination_path);
	if !confirm()? {
		return Ok(0);
	}
	std::fs::copy(source_path, destination_path)?;
	std::thread::sleep(std::time::Duration::from_millis(1));
	return Ok(1);
}

/// ディレクトリをコピーします。
fn find_file(source_path: &str, destination_path: &str, handler: &dyn Fn(&str, &str) -> std::result::Result<i32, Box<dyn std::error::Error>>) -> std::result::Result<i32, Box<dyn std::error::Error>> {
	let source_path = std::path::Path::new(source_path);
	let destination_path = std::path::Path::new(destination_path);
	if !source_path.exists() {
		println!("[TRACE] invalid path {}", source_path.to_str().unwrap());
		return Ok(0);
	}
	if source_path.is_dir() {
		let dir_name = source_path.file_name().unwrap().to_str().unwrap();
		if dir_name == "node_modules" {
			return Ok(0);
		}
		if dir_name == ".git" {
			return Ok(0);
		}
		if dir_name == "dist" {
			return Ok(0);
		}
		if dir_name == "target" {
			return Ok(0);
		}
		if dir_name == ".svn" {
			return Ok(0);
		}
		// コピー先にディレクトリを作成します。
		std::fs::create_dir_all(destination_path)?;
		// ディレクトリ内エントリーを走査
		let it = std::fs::read_dir(source_path)?;
		let mut affected = 0;
		for e in it {
			let entry = e?;
			let name = entry.file_name();
			let path = entry.path();
			affected = affected + find_file(&path.to_str().unwrap(), destination_path.join(name).as_path().to_str().unwrap(), handler)?;
		}
		return Ok(affected);
	}
	if source_path.is_file() {
		return handler(source_path.to_str().unwrap(), destination_path.to_str().unwrap());
	}
	println!("[WARN] 不明なファイルです。[{}]", source_path.to_str().unwrap());
	return Ok(0);
}

fn xcopy(source_path: &str, destination_path: &str) -> std::result::Result<i32, Box<dyn std::error::Error>> {
	return find_file(source_path, destination_path, &file_handler);
}

/// エントリーポイントです。
fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 3 {
		println!("path?");
		return;
	}
	let left = &args[1];
	let right = &args[2];
	let result = xcopy(left, right);
	if result.is_err() {
		println!("[ERROR] <main()> {}", result.err().unwrap());
		return;
	}
	let affected = result.unwrap();
	println!("{} file(s) copied.", affected);
}
