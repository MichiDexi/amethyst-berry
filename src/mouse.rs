use std::{time::Duration, io};
use crossterm::{event::poll};

struct Mouse {
	x : u16,
	y : u16,
	lclick : bool
}

impl Mouse {
	pub fn update(&mut self) -> io::Result<()> {
		if poll(Duration::from_millis(0))? {
			println!("{:?}", crossterm::event::read()?);
		}
		Ok(())
		// self.x = 
		// self.y = 
	}
}
