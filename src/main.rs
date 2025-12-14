use crossterm::{
	execute,
	cursor,
};
use std::{
	// env, <- Will be important later
	io::{
		stdout,
		Write,
	},
	thread::sleep,
	time::{
		Duration,
		Instant
	}
};

pub mod interface;
pub mod helpers;

const TARGET_FPS : f32 = 30.0;

fn main() {
	helpers::input::init().unwrap();
	let framerate : Duration = Duration::from_secs_f32(1.0 / TARGET_FPS);

	let mut mouse : helpers::input::Mouse = helpers::input::Mouse::new();
	let mut window : helpers::input::Window = helpers::input::Window::new();

	let mut textbox : interface::textbox::Box = interface::textbox::Box::new(20, 10, 21, 11, 0);

	let mut label : interface::label::Label =
	interface::label::Label::new(14, 9, 33, "This button will stop the program");
	
	let mut bar : interface::progressbar::ProgressBar =
	interface::progressbar::ProgressBar::new(10, 21, 41, 0, 100, ['<', '=', ' ', '>']);
		
	let mut table : interface::splitbox::SplitBox =
	interface::splitbox::SplitBox::new(40, 22, 20, 20, 1);
	
	let mut list : interface::list::List =
	interface::list::List::new(60, 2, 20, 10);

	list.items.push("a".to_string());
	list.items.push("b".to_string());
	list.items.push("c".to_string());
	list.items.push("d".to_string());
	list.items.push("e".to_string());
		list.items.push("a".to_string());
		list.items.push("b".to_string());
		list.items.push("c".to_string());
		list.items.push("d".to_string());
		list.items.push("e".to_string());
			list.items.push("a".to_string());
			list.items.push("b".to_string());
			list.items.push("c".to_string());
			list.items.push("d".to_string());
			list.items.push("e".to_string());
				list.items.push("a".to_string());
				list.items.push("b".to_string());
				list.items.push("c".to_string());
				list.items.push("d".to_string());
				list.items.push("e".to_string());
					list.items.push("a".to_string());
					list.items.push("b".to_string());
					list.items.push("c".to_string());
					list.items.push("d".to_string());
					list.items.push("e".to_string());
						list.items.push("a".to_string());
						list.items.push("b".to_string());
						list.items.push("c".to_string());
						list.items.push("d".to_string());
						list.items.push("e".to_string());
	
	let mut c : u16 = 0;

	loop {
		let now = Instant::now(); // Get frame time

		let mut out = stdout();

		helpers::input::update(&mut mouse, &mut window).unwrap();
		
		textbox.update(&mouse);
		table.update(&mouse);
		list.update(&mouse);
		
		if mouse.lclick {
			if textbox.hovered {
				break;
			}
		}

		

		c += 1;
		if c.is_multiple_of(10) {
			if c.is_multiple_of(20) {
				label.y += 1;
			}
			else {
				label.y -= 1;
			}
			bar.progress_full += 3;
		}
		write!(out, "\x1b[2J").unwrap();
		execute!(out, crossterm::cursor::MoveTo(0, 0)).unwrap();
		write!(out, "{}", table.hovered).unwrap();
		label.draw(&mut out);
		textbox.draw(&mut out);
		bar.draw(&mut out);
		table.draw(&mut out);
		list.draw(&mut out);
		stdout().flush().unwrap();
		
		// Frame time management for consistent framerate
		let frame_duration = Instant::now().duration_since(now);
		if frame_duration < framerate {
			sleep(framerate - frame_duration);
		}
	}
	
	helpers::input::uninit().unwrap();
}
