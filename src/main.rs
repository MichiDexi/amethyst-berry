use std::{
	io,
	time::{
		Duration,
		Instant,
	},
	thread::{
		sleep
	},
	io::{
		stdout,
		Write
	},
};
use crossterm::{
	execute,
	terminal::{self, ClearType},
};
// use std::fs;
use std::rc::Rc;
use std::cell::RefCell;
// use std::path::PathBuf;


pub mod interface;
pub mod helpers;
pub mod menus;
pub mod abt;

use crate::menus::menu_traits::Menu;

const TARGET_FPS : f64 = 30.0;

fn main() -> io::Result<()> {
	// initialize everything needed, like:
	// InputHandler, menus
	helpers::input::init()?;
	let framerate : Duration = Duration::from_secs_f64(1.0 / TARGET_FPS);
	let mut input : helpers::input::InputHandler = helpers::input::InputHandler::new();

	// initialize variables that persist between menus
	let mut out = stdout();
	let menu = Rc::new(RefCell::new(abt::menus::Menu::Main));
	let mut mainmenu = menus::mainmenu::MainMenu::init(Rc::clone(&menu));
	mainmenu.redraw(&input, &mut out);
	
	// the actual program
	loop {
		// setup for the frame
		// includes frame timing, input polling and redraw request accepting
		let now : Instant = Instant::now();
		input.update()?;
		if input.window.request_full_redraw {
			execute!(
			    out,
			    terminal::Clear(ClearType::All)
			)?;
		}

		// menu checking and executing their tick() behavior
		let cmenu = *menu.borrow();
		match cmenu {
			abt::menus::Menu::Main => mainmenu.tick(&input, &mut out),
			_ => break,
		}

		// flushing buffer and wait for the rest of
		// the frame to keep framerate stable
		stdout().flush().unwrap();
		let frame_duration = Instant::now().duration_since(now);
		if frame_duration < framerate {
			sleep(framerate - frame_duration);
		}
	}

	// the loop has exited, undo initialization of
	// InputHandler and screen extras because
	// they harm the terminal for no reason
	helpers::input::uninit()?;
	Ok(())
}
