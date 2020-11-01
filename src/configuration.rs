extern crate clap;

/// コンフィギュレーション構造体
#[derive(Debug, Clone)]
pub struct Configuration {
	pub left: String,
	pub right: String,
	pub dry_run: bool,
	pub verbose: bool,
}

impl Configuration {
	/// インスタンスを初期化します。
	pub fn new(left: String, right: String, dry_run: bool, verbose: bool) -> Configuration {
		let instance = Configuration {
			left: left,
			right: right,
			dry_run: dry_run,
			verbose: verbose,
		};
		return instance;
	}
}

/// コンフィギュレーションを行います。
pub fn configure() -> Option<Configuration> {
	let mut left = String::new();
	let mut right = String::new();
	let mut dry_run = false;
	let mut verbose = false;

	let args: Vec<String> = std::env::args().skip(1).collect();
	for e in args {
		if e == "--dry-run" {
			dry_run = true;
			continue;
		}
		if e == "--verbose" {
			verbose = true;
			continue;
		}
		if e.starts_with("--") {
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
