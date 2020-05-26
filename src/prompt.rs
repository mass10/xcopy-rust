use std::io::Write;

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
pub fn confirm() -> std::result::Result<bool, Box<dyn std::error::Error>> {
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
