/// ファイルのタイムスタンプを文字列で返します。(未使用)
#[allow(unused)]
fn get_filetime(s: &str) -> std::result::Result<String, std::io::Error> {
	let right_attribute = std::fs::metadata(s)?;
	let file_time = right_attribute.modified()?;
	let timestamp = format!("{:?}", file_time);
	return Ok(timestamp);
}

/// 二つのファイルが同一かどうかを調べます。
pub fn seems_to_be_same(source_path: &std::path::Path, destination_path: &std::path::Path) -> std::result::Result<bool, Box<dyn std::error::Error>> {
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
