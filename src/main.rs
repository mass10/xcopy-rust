extern crate clap;

mod application;
mod configuration;
mod myformatter;
mod prompt;

/// 使用方法を表示します。
fn usage() {
	println!("USAGE:");
	println!("    xcop-rust path/to/source path/to/destination --dry-run --verbose");
	println!();
	println!("    --help: Show usage.");
	println!("    --dry-run: Test run.");
	println!("    --verbose: Make operation verbose.");
	println!();
}

/// サマリーを表示します。
fn show_summary(affected: i32, dry_run: bool) {
	if dry_run {
		println!("{} file(s) deffer.", affected);
		return;
	}
	println!("{} file(s) copied.", affected);
}

/// アプリケーションのエントリーポイントです。
fn main() {
	// ========== CONFIGURATION ==========
	let result = configuration::configure();
	if result.is_none() {
		usage();
		return;
	}
	let conf = result.unwrap();

	// ========== XCOPY ==========
	let app = application::Application::new();
	let result = app.xcopy(
		conf.source.as_str(),  // 元のディレクトリ名
		conf.destination.as_str(), // 複製先
		conf.dry_run,        // テスト実行
		conf.verbose,        // 冗長モード
	);
	if result.is_err() {
		println!("[ERROR] <main()> {}", result.err().unwrap());
		return;
	}

	// 処理結果(コピーされたファイル数)
	let affected = result.ok().unwrap();

	// ========== SUMMARY ==========
	show_summary(affected, conf.dry_run);
}
