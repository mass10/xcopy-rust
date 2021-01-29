/// 二つのファイルが同一かどうかを調べます。
fn seems_to_be_same(source_path: &std::path::Path, destination_path: &std::path::Path) -> std::result::Result<bool, Box<dyn std::error::Error>> {
	// 元
	let source_attributes = std::fs::metadata(source_path)?;

	// 先
	let result = std::fs::metadata(destination_path);
	if result.is_err() {
		// 先のファイルがみつからないようです。
		return Ok(false);
	}
	let destination_attributes = result?;

	// サイズとタイムスタンプが同じなら同じとみなします。
	if source_attributes.len() == destination_attributes.len() {
		if source_attributes.modified()? == destination_attributes.modified()? {
			return Ok(true);
		}
	}

	// 中身を比較
	let result = file_diff::diff(source_path.to_str().unwrap(), destination_path.to_str().unwrap());
	return Ok(result);
}

/// ファイルごとに呼びだされるハンドラーです。
fn file_handler(source_path: &str, destination_path: &str, dry_run: bool, verbose: bool) -> std::result::Result<i32, Box<dyn std::error::Error>> {
	// 差分チェック
	if seems_to_be_same(std::path::Path::new(source_path), std::path::Path::new(destination_path))? {
		if verbose {
			println!("will be ignored: {}", destination_path);
		}
		return Ok(0);
	}

	// ========== DRY-RUN ==========
	if dry_run {
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

	{
		// ファイルの属性
		let left = std::fs::metadata(destination_path)?;
		// ファイルサイズ
		let len = left.len();
		// ファイル更新日時
		use super::myformatter::MyFormatter;
		let timestamp = format!("{}", left.modified()?.to_string1());
		println!("> {} ({}, {} bytes)", destination_path, timestamp, len);
	}

	std::thread::sleep(std::time::Duration::from_millis(1));

	return Ok(1);
}

type FileHandler = dyn Fn(&str, &str, bool, bool) -> std::result::Result<i32, Box<dyn std::error::Error>>;

/// ディレクトリを走査します。
///
/// ### Arguments
/// * source
/// * destination
/// * handler
/// * dry_run テスト実行
/// * verbose 冗長モード
fn find_file(source: &str, destination: &str, handler: &FileHandler, dry_run: bool, verbose: bool) -> std::result::Result<i32, Box<dyn std::error::Error>> {
	// 元
	let source_path = std::path::Path::new(source);
	// 先
	let destination_path = std::path::Path::new(destination);

	// 元の存在確認
	if !source_path.exists() {
		println!("[TRACE] invalid path {}", source_path.to_str().unwrap());
		return Ok(0);
	}

	// ディレクトリのコピー
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
		if !dry_run {
			std::fs::create_dir_all(destination_path)?;
		}
		// ディレクトリ内エントリーを走査
		let it = std::fs::read_dir(source_path)?;
		let mut affected = 0;
		for e in it {
			let entry = e?;
			let name = entry.file_name();
			let path = entry.path();
			affected = affected + find_file(&path.to_str().unwrap(), destination_path.join(name).as_path().to_str().unwrap(), handler, dry_run, verbose)?;
		}
		return Ok(affected);
	}

	// ファイルのコピー
	if source_path.is_file() {
		return handler(source_path.to_str().unwrap(), destination_path.to_str().unwrap(), dry_run, verbose);
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

pub struct Application;

///
/// アプリケーション
///
impl Application {
	/// 新しいインスタンスを返します。
	///
	/// ### Returns
	/// `Application` の新しいインスタンス
	pub fn new() -> Application {
		return Application {};
	}

	/// ディレクトリ全体をコピーします。
	pub fn xcopy(self, source_path: &str, destination_path: &str, dry_run: bool, verbose: bool) -> std::result::Result<i32, Box<dyn std::error::Error>> {
		return find_file(source_path, destination_path, &file_handler, dry_run, verbose);
	}
}
