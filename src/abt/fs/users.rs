use std::fs;
use std::io;

use crate::abt::fs::dirs;

pub fn create(name : &str) -> io::Result<()> {
	if let Some(mut path) = dirs::savedata_dir() {
		path.push(name);

		match fs::exists(&path) {
			Ok(true) => {
				return Err(std::io::Error::other("User already exists"));
			},
			Ok(false) => { },
			Err(e) => {
				return Err(e);
			}
		}

		fs::create_dir_all(path)?;
	}

	Ok(())
}

pub fn rename(old : &str, new : &str) -> io::Result<()> {
	if let Some(mut old_path) = dirs::savedata_dir() {
		let mut new_path = old_path.clone();
		old_path.push(old);
		new_path.push(new);

		match fs::exists(&old_path) {
			Ok(true) => { },
			Ok(false) => {
				return Err(std::io::Error::other("User doesn't exist"));
			},
			Err(e) => {
				return Err(e);
			}
		};

		match fs::exists(&new_path) {
			Ok(true) => {
				return Err(std::io::Error::other("User already exists"));
			},
			Ok(false) => { },
			Err(e) => {
				return Err(e);
			}
		};

		fs::rename(old_path, new_path)?;
	}

	Ok(())
}

pub fn delete(name : &str) -> io::Result<()> {
	if let Some(mut path) = dirs::savedata_dir() {
		path.push(name);

		match fs::exists(&path) {
			Ok(true) => { },
			Ok(false) => {
				return Err(std::io::Error::other("User doesn't exist"));
			},
			Err(e) => {
				return Err(e);
			}
		}

		fs::remove_dir_all(path)?;
	}

	Ok(())
}
