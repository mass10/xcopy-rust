extern crate clap;

mod application;
mod configuration;
mod myformatter;
mod prompt;

fn usage() {
	println!("[ERROR] invalid option.");
}

fn show_summary(affected: i32, dry_run: bool) {
	if dry_run {
		println!("{} file(s) deffer.", affected);
		return;
	}
	println!("{} file(s) copied.", affected);
}

/// エントリーポイントです。
fn main() {
	// ========== CONFIGURATION ==========
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
			usage();
			return;
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

	// ========== XCOPY ==========
	let app = application::Application::new();
	let result = app.xcopy(
		left.as_str(),  /*元のディレクトリ名*/
		right.as_str(), /*複製先*/
		dry_run,        /*テスト実行*/
		verbose,        /*冗長モード*/
	);
	if result.is_err() {
		println!("[ERROR] <main()> {}", result.err().unwrap());
		return;
	}

	// 処理結果(コピーされたファイル数)
	let affected = result.ok().unwrap();

	// ========== SUMMARY ==========
	show_summary(affected, dry_run);
}
