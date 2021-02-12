extern crate clap;

///
/// コンフィギュレーション構造体
///
#[derive(Debug, Clone)]
pub struct Configuration {
	/// コピー元
	pub source_path: String,
	/// コピー先
	pub destination_path: String,
	/// テスト実行
	pub dry_run: bool,
	/// 冗長実行
	pub verbose: bool,
}

impl Configuration {
	/// インスタンスを初期化します。
	///
	/// ### Arguments
	/// * source コピー元
	/// * destination コピー先
	/// * dry_run テスト実行
	/// * verbose 冗長実行
	pub fn new(source: String, destination: String, dry_run: bool, verbose: bool) -> Configuration {
		let instance = Configuration {
			source_path: source,
			destination_path: destination,
			dry_run: dry_run,
			verbose: verbose,
		};
		return instance;
	}
}

/// コンフィギュレーションを行います。
///
/// ### Returns
/// Configuration の新しいインスタンス
pub fn configure() -> Option<Configuration> {
	// コピー元
	let mut left = String::new();
	// コピー先
	let mut right = String::new();
	// テスト実行
	let mut dry_run = false;
	// 冗長モード
	let mut verbose = false;

	// コマンドライン引数
	let args: Vec<String> = std::env::args().skip(1).collect();

	for e in args {
		if e == "--help" {
			// usage
			return None;
		}
		if e == "--dry-run" {
			dry_run = true;
			continue;
		}
		if e == "--verbose" {
			verbose = true;
			continue;
		}
		if e.starts_with("--") {
			println!("[ERROR] Unknown option flag {}.", e);
			println!();
			// usage
			return None;
		}
		if e.starts_with("-") {
			println!("[ERROR] Short option flags are not supported.");
			println!();
			// usage
			return None;
		}
		if left == "" {
			left = e;
			continue;
		}
		if right == "" {
			right = e;
			continue;
		}
	}

	if left == "" {
		return None;
	}
	if right == "" {
		return None;
	}

	return Some(Configuration::new(left, right, dry_run, verbose));
}
