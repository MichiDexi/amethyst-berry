use std::fs;
use std::path::PathBuf;
use std::io;
use std::collections::HashMap;

use crate::abt::fs::dirs;
use crate::abt::fs::sf;

pub fn create(
	user : &str, savefile : &str,
	lobby : &str, name : &str
) -> io::Result<()>
{
	if let Some(mut path) = dirs::savedata_dir() {
		path.push(user);
		if let Ok(v) = fs::exists(&path) && !v {
			return Err(std::io::Error::other("User doesn't exist"));
		}
		path.push(savefile);
		if let Ok(exists) = fs::exists(&path) && !exists {
			return Err(std::io::Error::other("Savefile doesn't exist"));
		}
		if !lobby.is_empty() {
			path.push(lobby);
			if let Ok(exists) = fs::exists(&path) && !exists {
				return Err(std::io::Error::other("Lobby doesn't exist"));
			}
			if let Ok(false) = sf::is_lobby_based(user, savefile) {
				return Err(std::io::Error::other("Savefile isn't lobby based"));
			}
		}
		else {
			path.push("Maps");
			if let Ok(true) = sf::is_lobby_based(user, savefile) {
				return Err(std::io::Error::other("Savefile is lobby based"));
			}
		}
		path.push(name);

		match fs::exists(&path) {
			Ok(true) => {
				return Err(std::io::Error::other("Map file already exists"));
			},
			Ok(false) => { },
			Err(e) => {
				return Err(e);
			}
		}

		fs::File::create(path)?;
	}

	Ok(())
}

pub fn rename(
	user : &str, savefile : &str,
	lobby : &str, old : &str, new : &str
) -> io::Result<()>
{
	if let Some(mut old_path) = dirs::savedata_dir() {
		old_path.push(user);
		if let Ok(v) = fs::exists(&old_path) && !v {
			return Err(std::io::Error::other("User doesn't exist"));
		}
		old_path.push(savefile);
		if let Ok(exists) = fs::exists(&old_path) && !exists {
			return Err(std::io::Error::other("Savefile doesn't exist"));
		}
		if !lobby.is_empty() {
			old_path.push(lobby);
			if let Ok(exists) = fs::exists(&old_path) && !exists {
				return Err(std::io::Error::other("Lobby doesn't exist"));
			}
		}
		else {
			old_path.push("Maps");
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

pub fn delete(
	user : &str, savefile : &str,
	lobby : &str, name : &str)
-> io::Result<()>
{
	if let Some(mut path) = dirs::savedata_dir() {
		path.push(user);
		if let Ok(v) = fs::exists(&path) && !v {
			return Err(std::io::Error::other("User doesn't exist"));
		}
		path.push(savefile);
		if let Ok(exists) = fs::exists(&path) && !exists {
			return Err(std::io::Error::other("Savefile doesn't exist"));
		}
		if !lobby.is_empty() {
			path.push(lobby);
			if let Ok(exists) = fs::exists(&path) && !exists {
				return Err(std::io::Error::other("Lobby doesn't exist"));
			}
		}
		else {
			path.push("Maps");
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

		fs::remove_file(path)?;
	}

	Ok(())
}

pub fn next_num(user : &str, savefile : &str, lobby : &str) -> io::Result<u64> {
	if let Some(mut path) = dirs::savedata_dir() {
		path.push(user);
		if let Ok(v) = fs::exists(&path) && !v {
			return Err(std::io::Error::other("User doesn't exist"));
		}
		path.push(savefile);
		if let Ok(exists) = fs::exists(&path) && !exists {
			return Err(std::io::Error::other("Savefile doesn't exist"));
		}
		if !lobby.is_empty() {
			path.push(lobby);
			if let Ok(exists) = fs::exists(&path) && !exists {
				return Err(std::io::Error::other("Lobby doesn't exist"));
			}
		}
		else {
			path.push("Maps");
		}
		
		normalize_files(&path)?;
		let last_number : Option<u64> = fs::read_dir(path)?
			.filter_map(|entry| entry.ok())
			.filter_map(|entry| {
				entry.file_name()
				.to_str()?
				.parse::<u64>()
				.ok()
			})
			.max();

		return Ok(1+last_number.unwrap_or(0));
	}
	Err(std::io::Error::other("Couldn't get savedata directory (~/.local/share/abt/)"))
}

pub fn normalize_files(path : &PathBuf) -> io::Result<()> {
	let mut nums : Vec<u64> = fs::read_dir(path)?
		.filter_map(|entry| entry.ok())
		.filter_map(|entry| entry.file_name().to_str()?.parse::<u64>().ok())
		.collect();

	let mut sorted : Vec<u64> = nums.clone();
	sorted.sort_unstable();
	sorted.dedup();

	let mapping : HashMap<u64, u64> = sorted
		.iter()
		.enumerate()
		.map(|(rank, &num)| (num, rank as u64))
		.collect();

	nums.sort_unstable_by(|a, b| b.cmp(a));

	for original in nums {
		let new_num = mapping[&original];
		let original_path = path.join(original.to_string());
		let new_path = path.join(new_num.to_string());

		if original_path != new_path {
			fs::rename(&original_path, &new_path)?;
		}
	}

	Ok(())
}
