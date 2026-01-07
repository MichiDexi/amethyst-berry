use std::fs;
use std::io;

use crate::abt::fs::dirs;
use crate::abt::fs::sf;

pub fn create(user : &str, savefile : &str, name : &str) -> io::Result<()> {
	if let Some(mut path) = dirs::savedata_dir() {

		path.push(user);
		if let Ok(v) = fs::exists(&path) && !v {
			return Err(std::io::Error::other("User doesn't exist"));
		}
		path.push(savefile);
		if let Ok(exists) = fs::exists(&path) && !exists {
			return Err(std::io::Error::other("Savefile doesn't exist"));
		}

		if let Ok(false) = sf::is_lobby_based(user, savefile) {
			return Err(std::io::Error::other("Savefile isn't lobby based"));
		}

		
		path.push(name);

		match fs::exists(&path) {
			Ok(true) => {
				return Err(std::io::Error::other("Lobby already exists"));
			},
			Ok(false) => { },
			Err(e) => {
				return Err(e);
			}
		}

		fs::create_dir(path)?;
	}

	Ok(())
}

pub fn rename(user : &str, savefile : &str, old : &str, new : &str) -> io::Result<()> {
	if let Some(mut old_path) = dirs::savedata_dir() {
		old_path.push(user);
		if let Ok(v) = fs::exists(&old_path) && !v {
			return Err(std::io::Error::other("User doesn't exist"));
		}
		old_path.push(savefile);
		if let Ok(v) = fs::exists(&old_path) && !v {
			return Err(std::io::Error::other("Savefile doesn't exist"));
		}
		
		let mut new_path = old_path.clone();
		
		old_path.push(old);
		new_path.push(new);

		match fs::exists(&old_path) {
			Ok(true) => { },
			Ok(false) => {
				return Err(std::io::Error::other("Lobby doesn't exist"));
			},
			Err(e) => {
				return Err(e);
			}
		};

		match fs::exists(&new_path) {
			Ok(true) => {
				return Err(std::io::Error::other("Lobby already exists"));
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

pub fn delete(user : &str, savefile : &str, name : &str) -> io::Result<()> {
	if let Some(mut path) = dirs::savedata_dir() {
		path.push(user);
		if let Ok(v) = fs::exists(&path) && !v {
			return Err(std::io::Error::other("User doesn't exist"));
		}
		path.push(savefile);
		if let Ok(v) = fs::exists(&path) && !v {
			return Err(std::io::Error::other("Savefile doesn't exist"));
		}
		
		path.push(name);

		match fs::exists(&path) {
			Ok(true) => { },
			Ok(false) => {
				return Err(std::io::Error::other("Lobby doesn't exist"));
			},
			Err(e) => {
				return Err(e);
			}
		}

		fs::remove_dir_all(path)?;
	}

	Ok(())
}
