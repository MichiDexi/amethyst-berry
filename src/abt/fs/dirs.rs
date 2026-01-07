use std::path::PathBuf;
use std::env;
use std::io;
use std::fs;
use std::fs::File;

pub const CONFIG : &str = ".config/abt/";
pub const SAVEDATA : &str = ".local/share/abt/";

pub fn config_dir() -> Option<PathBuf> {
	let home_dir = env::home_dir();
	if let Some(v) = home_dir {
		let mut config : PathBuf = v.clone();
		config.push(CONFIG);
		return Some(config);
	}
	None
}

pub fn savedata_dir() -> Option<PathBuf> {
	let home_dir = env::home_dir();
	if let Some(v) = home_dir {
		let mut savedata : PathBuf = v.clone();
		savedata.push(SAVEDATA);
		return Some(savedata);
	}
	None
}

pub fn config_file() -> Option<PathBuf> {
	if let Some(mut v) = config_dir() {
		v.push("abt.cfg");
		return Some(v);
	}
	None
}

pub fn init_directories() -> io::Result<()> {
	// Config dir
	if let Some(v) = config_dir() {
		match fs::exists(&v) {
			Ok(true) => { },
			Ok(false) => {
				fs::create_dir_all(v)?;
			},
			Err(e) => {
				return Err(e);
			}
		}
	}
	// Config file
	if let Some(v) = config_file() {
		match fs::exists(&v) {
			Ok(true) => { },
			Ok(false) => {
				File::create(v)?;
			},
			Err(e) => {
				return Err(e);
			}
		}
	}
	// Savedata dir
	if let Some(v) = savedata_dir() {
		match fs::exists(&v) {
			Ok(true) => { },
			Ok(false) => {
				fs::create_dir_all(v)?;
			},
			Err(e) => {
				return Err(e);
			}
		};
	}
	Ok(())
}

pub fn path_exists(name : &PathBuf) -> bool {
	match fs::exists(name) {
		Ok(true) => true,
		Ok(false) => false,
		Err(_) => false
	}
}
