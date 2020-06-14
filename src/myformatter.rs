extern crate chrono;

pub trait MyFormatter {
	/// std::time::SystemTime の文字列表現を返します。
	fn to_string1(&self) -> String;
}

impl MyFormatter for std::time::SystemTime {
	fn to_string1(&self) -> String {
		let timestamp = chrono::DateTime::<chrono::Local>::from(*self);
		return format!("{}", timestamp.format("%Y-%m-%d %H:%M:%S%.3f"));
	}
}
