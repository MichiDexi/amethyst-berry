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
	execute,
	terminal::size,
	cursor,
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
			Event::Mouse(event) => {
				let mouse_input : (u16, u16, bool) = handle_mouse(event);
				mouse.x = mouse_input.0;
				mouse.y = mouse_input.1;
				mouse.lclick = mouse_input.2;
			},
			Event::Resize(width, height) => {window.width = width; window.height = height},
			_ => {}
		}
	}
	Ok(())
}

fn handle_mouse(event : MouseEvent) -> (u16, u16, bool) {
	let pressed = matches!(
		event.kind,
		MouseEventKind::Down(MouseButton::Left)
		| MouseEventKind::Drag(MouseButton::Left)
	);

	(event.column, event.row, pressed)
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
