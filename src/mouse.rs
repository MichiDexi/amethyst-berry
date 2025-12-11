use std::{time::Duration, io};
use crossterm::event::{poll, read};

pub struct Mouse {
	pub x : u16,
	pub y : u16,
	pub lclick : bool
}

impl Mouse {
	pub fn update(&mut self) -> io::Result<()> {
		if poll(Duration::from_millis(0))? {
			println!("{:?}", read()?);
		}
		Ok(())
		// self.x = 
		// self.y = 
	}
}
