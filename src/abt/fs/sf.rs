use std::fs;
use std::fs::File;
use std::io;
use std::io::Error;
use std::io::prelude::*;

use crate::abt::fs::dirs;

pub fn create(user : &str, name : &str, lobby_based : bool) -> io::Result<()> {
	if let Some(mut path) = dirs::savedata_dir() {
		path.push(user);

		if let Ok(v) = fs::exists(&path) && !v {
			return Err(std::io::Error::other("User doesn't exist"));
		}
		
		path.push(name);

		match fs::exists(&path) {
			Ok(true) => {
				return Err(std::io::Error::other("Savefile already exists"));
			},
			Ok(false) => { },
			Err(e) => {
				return Err(e);
			}
		}

		fs::create_dir(&path)?;
		path.push("save.cfg");
		let mut file = File::create(&path)?;
		match lobby_based {
			true => {
				file.write_all(b"t")?;
			},
			false => {
				file.write_all(b"f")?;
				path.pop();
				path.push("Maps");
				fs::create_dir(&path)?;
			},
		}
	}

	Ok(())
}

pub fn rename(user : &str, old : &str, new : &str) -> io::Result<()> {
	if let Some(mut old_path) = dirs::savedata_dir() {
		old_path.push(user);
		let mut new_path = old_path.clone();

		if let Ok(v) = fs::exists(&old_path) && !v {
			return Err(std::io::Error::other("User doesn't exist"));
		}
		
		old_path.push(old);
		new_path.push(new);

		match fs::exists(&old_path) {
			Ok(true) => { },
			Ok(false) => {
				return Err(std::io::Error::other("Savefile doesn't exist"));
			},
			Err(e) => {
				return Err(e);
			}
		};

		match fs::exists(&new_path) {
			Ok(true) => {
				return Err(std::io::Error::other("Savefile already exists"));
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

pub fn delete(user : &str, name : &str) -> io::Result<()> {
	if let Some(mut path) = dirs::savedata_dir() {
		path.push(user);

		if let Ok(v) = fs::exists(&path) && !v {
			return Err(std::io::Error::other("User doesn't exist"));
		}
		
		path.push(name);

		match fs::exists(&path) {
			Ok(true) => { },
			Ok(false) => {
				return Err(std::io::Error::other("Savefile doesn't exist"));
			},
			Err(e) => {
				return Err(e);
			}
		}

		fs::remove_dir_all(path)?;
	}

	Ok(())
}

pub fn is_lobby_based(user : &str, name : &str) -> Result<bool, Error> {
	if let Some(mut path) = dirs::savedata_dir() {
		path.push(user);
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

		path.push("save.cfg");
		let mut file = File::open(path)?;
		let mut contents = String::new();
		file.read_to_string(&mut contents)?;
		return match &contents[0..1] {
			"t" => Ok(true),
			"f" => Ok(false),
			_ => Ok(false)
		};
	}

	Ok(false)
}
