use std::{time::Duration, io};
use std::io::{Write, stdout};
use crossterm::event::{
	EnableBracketedPaste,
	EnableFocusChange,
	EnableMouseCapture,
	Event,
	poll,
	read,
};
use crossterm::{execute, style::Print};
use crossterm::{
	terminal::size,
	cursor,
	event::KeyCode
};

pub struct Mouse {
	pub x : u16,
	pub y : u16,
	pub lclick : bool
}

impl Mouse {
	pub fn update(&mut self) -> io::Result<()> {
		if poll(Duration::from_millis(100))? {
			println!("{:?}", read()?);
			match read()? {
				Event::FocusGained => println!("FocusGained"),
				Event::FocusLost => println!("FocusLost"),
				Event::Key(event) => println!("{:?}", event),
				Event::Mouse(event) => println!("{:?}", event),
				Event::Paste(data) => println!("Pasted {:?}", data),
				Event::Resize(width, height) => println!("New size {}x{}", width, height),
			}
		}
		Ok(())
		// self.x = 
		// self.y = 
	}
}

fn init() {
	let mut stdout = io::stdout();
	execute!(stdout, crossterm::event::EnableBracketedPaste).unwrap();
	execute!(stdout, crossterm::event::EnableFocusChange).unwrap();
	execute!(stdout, crossterm::event::EnableMouseCapture).unwrap();
}
