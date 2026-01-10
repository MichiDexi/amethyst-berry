use std::io;
use std::io::stdout;
use std::io::Write;
use std::time::Duration;
use std::time::Instant;
use std::thread::sleep;
use std::cell::RefCell;
use std::rc::Rc;
use crossterm::execute;
use crossterm::terminal;
use crossterm::terminal::ClearType;

use crate::menus::menu_traits::Menu;

pub mod interface;
pub mod helpers;
pub mod menus;
pub mod abt;

const TARGET_FPS : f64 = 30.0;

fn main() -> io::Result<()> {
	// initialize everything needed, like:
	// InputHandler, menus
	helpers::input::init()?;
	let framerate : Duration = Duration::from_secs_f64(1.0 / TARGET_FPS);
	let mut input : helpers::input::InputHandler = helpers::input::InputHandler::new();

	// initialize variables that persist between menus
	let mut out = stdout();
	let menu : Rc<RefCell<abt::menus::Menu>> =
		Rc::new(RefCell::new(abt::menus::Menu::Main));
	let data : Rc<RefCell<abt::data::Data>> =
		Rc::new(RefCell::new(abt::data::Data {
			user : None, savefile : None, lobby : None, map : None }));
	let mut mainmenu =
		menus::mainmenu::MainMenu::init(Rc::clone(&menu), Rc::clone(&data));
	let mut userselect =
		menus::userselect::UserSelect::init(Rc::clone(&menu), Rc::clone(&data));
	let mut sfselect =
		menus::sfselect::SaveSelect::init(Rc::clone(&menu), Rc::clone(&data));
	mainmenu.redraw(&input, &mut out);
	let mut new_menu : abt::menus::Menu;

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
		let cmenu : abt::menus::Menu = *menu.borrow();
		match cmenu {
			abt::menus::Menu::Main => mainmenu.tick(&input, &mut out),
			abt::menus::Menu::UserSelect => userselect.tick(&input, &mut out),
			abt::menus::Menu::SavefileSelect => sfselect.tick(&input, &mut out),
			_ => break,
		}
		new_menu = *menu.borrow();
		if new_menu != cmenu {
			input.update()?; // Second update to disallow clicking in a menu that hasn't appeared yet
			match new_menu {
				abt::menus::Menu::Main => mainmenu.redraw(&input, &mut out),
				abt::menus::Menu::UserSelect => userselect.redraw(&input, &mut out),
				abt::menus::Menu::SavefileSelect => userselect.redraw(&input, &mut out),
				_ => break,
			}
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
