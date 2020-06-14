extern crate clap;

mod application;
mod configuration;
mod myformatter;
mod prompt;

fn usage() {
	println!("[ERROR] invalid option.");
}

/// エントリーポイントです。
fn main() {
	// ========== CONFIGURATION(BAK) ==========
	if false {
		let _ = configuration::Configuration::commandline_arguments();
	}

	// ========== CONFIGURATION ==========
	let args: Vec<String> = std::env::args().skip(1).collect();
	let mut conf = configuration::Configuration::get_instance();
	let mut left = "".to_string();
	let mut right = "".to_string();
	for e in args {
		if e == "--dry-run" {
			conf.dry_run = true;
			continue;
		}
		if e == "--verbose" {
			conf.verbose = true;
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
	let result = app.xcopy(left.as_str(), right.as_str());
	if result.is_err() {
		println!("[ERROR] <main()> {}", result.err().unwrap());
		return;
	}

	// ========== SUMMARY ==========
	let affected = result.ok().unwrap();
	if conf.dry_run {
		println!("{} file(s) deffer.", affected);
	} else {
		println!("{} file(s) copied.", affected);
	}
}
