use crate::env::current_dir;

pub fn get_absolute_path(path: &str) -> String {
	let mut path = path.to_string();
	if !path.starts_with("/") {
		path = current_dir().unwrap().join(path).to_str().unwrap().to_string();
	}
	path
}
