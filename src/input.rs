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

pub struct Window {
	pub focused : bool,
	pub width : u16,
	pub height : u16,
}

pub fn update(mouse : &mut Mouse, window : &mut Window) -> io::Result<()> {
	if poll(Duration::from_millis(100))? {
		// println!("{:?}", read()?);
		match read()? {
			Event::FocusGained => window.focused = true,
			Event::FocusLost => window.focused = false,
			Event::Mouse(event) => println!("{:?}", event),
			Event::Resize(width, height) => {window.width = width; window.height = height},
			_ => {}
		}
	}
	Ok(())
}


pub fn init() {
	let mut stdout = io::stdout();
	execute!(stdout, crossterm::event::EnableBracketedPaste).unwrap();
	execute!(stdout, crossterm::event::EnableFocusChange).unwrap();
	execute!(stdout, crossterm::event::EnableMouseCapture).unwrap();
}

pub fn uninit() { // Initializes the end of all functions
	let mut stdout = io::stdout();
	execute!(stdout, crossterm::event::EnableBracketedPaste).unwrap();
	execute!(stdout, crossterm::event::EnableFocusChange).unwrap();
	execute!(stdout, crossterm::event::EnableMouseCapture).unwrap();
}
