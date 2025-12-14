use std::{
	time::Duration,
	io::{
		Write,
		stdout,
		self
	},
};
use crossterm::{
	event::{
		EnableBracketedPaste,
		EnableFocusChange,
		EnableMouseCapture,
		Event,
		KeyCode,
		MouseEvent,
		MouseEventKind,
		MouseButton,
		poll,
		read,
	},
	terminal::{
		size,
		enable_raw_mode,
		disable_raw_mode,
		SetTitle,
	},
	cursor::{
		Hide,
		Show,
	},
	execute,
};


pub struct Mouse {
	pub x : u16,
	pub y : u16,
	pub lclick : bool,
	pub lclickheld : bool,
	pub rclick : bool,
	pub rclickheld : bool,
	pub scroll : i8,
}

impl Mouse {
	pub fn new() -> Self {
		Mouse {
			x : 0, y : 0,
			lclick : false, lclickheld : false,
			rclick : false, rclickheld : false,
			scroll : 0
		}
	}
}

pub struct Window {
	pub focused : bool,
	pub width : u16,
	pub height : u16,
}

impl Window {
	pub fn new() -> Self {
		let termsize = crossterm::terminal::size().unwrap();
		Window {
			focused : false, 
			width : termsize.0,
			height : termsize.1,
		}
	}
}

pub fn update(mouse : &mut Mouse, window : &mut Window) -> io::Result<()> {
	mouse.scroll = 0;
	while poll(Duration::from_millis(0))? {
		match read()? {
			Event::FocusGained => window.focused = true,
			Event::FocusLost => window.focused = false,
			Event::Mouse(event) => { handle_mouse(event, mouse); },
			Event::Resize(width, height) => {window.width = width; window.height = height},
			_ => {}
		}
	}
	Ok(())
}

fn handle_mouse(event : MouseEvent, mouse : &mut Mouse) {

	mouse.lclick = false;
	mouse.rclick = false;
	

	mouse.x = event.column;
	mouse.y = event.row;

	match event.kind {
		MouseEventKind::Down(MouseButton::Left)  => mouse.lclick     = true,
		MouseEventKind::Drag(MouseButton::Left)  => mouse.lclickheld = true,
		MouseEventKind::Up(MouseButton::Left)    => mouse.lclickheld = false,
		MouseEventKind::Down(MouseButton::Right) => mouse.rclick     = true,
		MouseEventKind::Drag(MouseButton::Right) => mouse.rclickheld = true,
		MouseEventKind::Up(MouseButton::Right)   => mouse.rclickheld = false,
		MouseEventKind::ScrollUp => mouse.scroll = 1,
		MouseEventKind::ScrollDown => mouse.scroll = -1,
		_ => {}
	}
}


pub fn init() -> io::Result<()> {
	let mut stdout = io::stdout();
	execute!(stdout, crossterm::terminal::EnterAlternateScreen).unwrap();
	execute!(stdout, crossterm::cursor::Hide).unwrap();
	execute!(stdout, crossterm::event::EnableBracketedPaste).unwrap();
	execute!(stdout, crossterm::event::EnableFocusChange).unwrap();
	execute!(stdout, crossterm::event::EnableMouseCapture).unwrap();
	execute!(stdout, SetTitle("Amethyst berry tool"))?;
	enable_raw_mode()?;
	Ok(())
}

pub fn uninit() -> io::Result<()> { // Initializes the end of all functions
	let mut stdout = io::stdout();
	execute!(stdout, crossterm::terminal::LeaveAlternateScreen).unwrap();
	execute!(stdout, crossterm::cursor::Show).unwrap();
	execute!(stdout, crossterm::event::DisableBracketedPaste).unwrap();
	execute!(stdout, crossterm::event::DisableFocusChange).unwrap();
	execute!(stdout, crossterm::event::DisableMouseCapture).unwrap();
	disable_raw_mode()?;
	Ok(())
}
