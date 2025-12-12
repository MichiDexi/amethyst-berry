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
	pub rclick : bool,
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
			Event::Mouse(event) => {
				let mouse_input : (u16, u16, bool, bool) = handle_mouse(event);
				mouse.x = mouse_input.0;
				mouse.y = mouse_input.1;
				mouse.lclick = mouse_input.2;
				mouse.rclick = mouse_input.3;
			},
			Event::Resize(width, height) => {window.width = width; window.height = height},
			_ => {}
		}
	}
	Ok(())
}

fn handle_mouse(event : MouseEvent) -> (u16, u16, bool, bool) {
	let lpressed = matches!(
		event.kind,
		MouseEventKind::Down(MouseButton::Left)
		| MouseEventKind::Drag(MouseButton::Left)
	);

	let rpressed = matches!(
		event.kind,
		MouseEventKind::Down(MouseButton::Right)
		| MouseEventKind::Drag(MouseButton::Right)
	);

	(event.column, event.row, lpressed, rpressed)
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
