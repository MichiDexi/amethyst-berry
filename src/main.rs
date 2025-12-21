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
		Write,
		Stdout,
	},
};


pub mod interface;
pub mod helpers;
pub mod menus;
pub mod abt;

const TARGET_FPS : f64 = 30.0;

fn main() -> io::Result<()> {

	let mapstring : &str = "05TAGID01020304050607080910000";
	let map : abt::mapdata::Map = abt::mapdata::parse_map(&mapstring.to_string());
	println!("{}", map.tag);
	println!("{}", map.difficulty);
	return Ok(());
	

	// Initialize everything, including:
	// InputHandler, Main menu
	helpers::input::init()?;
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
		input.update()?;

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
	helpers::input::uninit()?;
	Ok(())
}
