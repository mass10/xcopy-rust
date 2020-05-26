/// ファイルごとに呼びだされるハンドラーです。
fn file_handler(source_path: &str, destination_path: &str) -> std::result::Result<i32, Box<dyn std::error::Error>> {
	// コンフィギュレーション
	let conf = super::configuration::Configuration::get_instance();

	// 差分チェック
	if seems_to_be_same(std::path::Path::new(source_path), std::path::Path::new(destination_path))? {
		if conf.verbose {
			println!("will be ignored: {}", destination_path);
		}
		return Ok(0);
	}

	// ========== DRY-RUN ==========
	if conf.dry_run {
		println!("will be updated: {}", destination_path);
		return Ok(1);
	}

	// 上書き確認
	println!("ファイル {} を上書きしますか？", destination_path);
	if !super::prompt::confirm()? {
		return Ok(0);
	}

	// コピー
	std::fs::copy(source_path, destination_path)?;
	std::thread::sleep(std::time::Duration::from_millis(1));

	return Ok(1);
}

/// ディレクトリを走査します。
fn find_file(source_path: &str, destination_path: &str, handler: &dyn Fn(&str, &str) -> std::result::Result<i32, Box<dyn std::error::Error>>) -> std::result::Result<i32, Box<dyn std::error::Error>> {
	// コンフィギュレーション
	let conf = super::configuration::Configuration::get_instance();
	// 元
	let source_path = std::path::Path::new(source_path);
	// 先
	let destination_path = std::path::Path::new(destination_path);
	if !source_path.exists() {
		println!("[TRACE] invalid path {}", source_path.to_str().unwrap());
		return Ok(0);
	}

	// ディレクトリ
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
		if !conf.dry_run {
			std::fs::create_dir_all(destination_path)?;
		}
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

	// ファイル
	if source_path.is_file() {
		return handler(source_path.to_str().unwrap(), destination_path.to_str().unwrap());
	}

	// 不明なファイルシステム
	println!("[WARN] 不明なファイルです。[{}]", source_path.to_str().unwrap());
	return Ok(0);
}

/// ファイルのタイムスタンプを文字列で返します。(未使用)
#[allow(unused)]
fn get_filetime(s: &str) -> std::result::Result<String, std::io::Error> {
	let right_attribute = std::fs::metadata(s)?;
	let file_time = right_attribute.modified()?;
	let timestamp = format!("{:?}", file_time);
	return Ok(timestamp);
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

pub struct Application;

/// アプリケーション
impl Application {
	/// 新しいインスタンスを返します。
	pub fn new() -> Application {
		return Application {};
	}

	/// ディレクトリ全体をコピーします。
	pub fn xcopy(self, source_path: &str, destination_path: &str) -> std::result::Result<i32, Box<dyn std::error::Error>> {
		return find_file(source_path, destination_path, &file_handler);
	}
}
