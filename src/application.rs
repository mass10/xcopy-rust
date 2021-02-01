use super::io;

/// ファイルごとに呼びだされるハンドラーです。
fn file_handler(source_path: &str, destination_path: &str, dry_run: bool, verbose: bool) -> std::result::Result<i32, Box<dyn std::error::Error>> {
	// 差分チェック
	if io::seems_to_be_same(std::path::Path::new(source_path), std::path::Path::new(destination_path))? {
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
/// * source_path
/// * destination_path
/// * handler エントリーを受け取るハンドラー
/// * dry_run テスト実行
/// * verbose 冗長モード
fn find_file(source_path: &str, destination_path: &str, handler: &FileHandler, dry_run: bool, verbose: bool) -> std::result::Result<i32, Box<dyn std::error::Error>> {
	// 元
	let source_path = std::path::Path::new(source_path);
	// 先
	let destination_path = std::path::Path::new(destination_path);

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
	///
	/// ### Arguments
	/// * source_path
	/// * destination_path
	/// * dry_run テスト実行
	/// * verbose 冗長モード
	pub fn xcopy(self, source_path: &str, destination_path: &str, dry_run: bool, verbose: bool) -> std::result::Result<i32, Box<dyn std::error::Error>> {
		return find_file(source_path, destination_path, &file_handler, dry_run, verbose);
	}
}
