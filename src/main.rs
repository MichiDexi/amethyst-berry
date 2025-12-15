use std::{
	time::{
		Duration,
		Instant,
	},
	thread::{
		sleep
	},
	io::{
		stdout,
		Write,
		Stdout,
	},
};


pub mod interface;
pub mod helpers;
pub mod menus;
pub mod abt;

const TARGET_FPS : f64 = 30.0;

fn main() {
	// Initialize everything, including:
	// InputHandler, Main menu
	helpers::input::init().unwrap();
	let framerate : Duration = Duration::from_secs_f64(1.0 / TARGET_FPS);
	let mut input : helpers::input::InputHandler = helpers::input::InputHandler::new();

	// Initialize variables that persist between menu switches
	let mut out = stdout();
	let mut menu = abt::menus::Menu::Main;
	let mut mainmenu = menus::mainmenu::MainMenu::init(&mut out);
	
	// Loop and use the program:
	// Switch between menus and interact with the filesystem
	loop {
		// Set frame up
		let now : Instant = Instant::now();
		input.update().unwrap();

		// The actual frame
		match menu {
			abt::menus::Menu::Main => mainmenu.tick(&input, &mut out, &mut menu),
			abt::menus::Menu::Out => break,
			_ => {}
		}
		if menu == abt::menus::Menu::Out {
			break;
		}

		// Frame time management for consistent framerate
		stdout().flush().unwrap();
		let frame_duration = Instant::now().duration_since(now);
		if frame_duration < framerate {
			sleep(framerate - frame_duration);
		}
	}

	// The loop has been exited, undo initialization of
	// InputHandler and screen extras
	helpers::input::uninit().unwrap();
}
