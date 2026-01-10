use crossterm::{
	execute,
	event::KeyCode,
	event::KeyEvent,
	event::KeyEventKind
};
use std::io::{
	Stdout,
	Write,
	stdout,
};

use crate::helpers::utils;
use crate::helpers::input;
use crate::interface::traits;

pub struct InputField {
	// Size and position
	pub x : u16, pub y : u16,
	pub size : u16,
	pub output : String,
	pub output_prev : String,

	// Extra options
	pub color : utils::Color,
	pub bgcolor : utils::Color,

	// Event polling
	pub hovered : bool,
	pub cursor : u16,
}


impl traits::UserInterface for InputField {
	fn draw(&self, out : &mut Stdout) {
		self.color.write_color(out, false);
		self.bgcolor.write_color(out, true);

		execute!(out, crossterm::cursor::MoveTo(self.x, self.y)).unwrap();
		utils::repeat(out, ' ', self.size);
		execute!(out, crossterm::cursor::MoveTo(self.x, self.y)).unwrap();
		write!(out, "{}", utils::shorten_text(&self.output, self.size)).unwrap();

		write!(out, "\x1b[0m").unwrap();

		stdout().flush().unwrap();
	}
	
	fn clear(&self, out : &mut Stdout) {
		execute!(out, crossterm::cursor::MoveTo(self.x, self.y)).unwrap();
		utils::repeat(out, ' ', self.size);

		stdout().flush().unwrap();
	}
	
	fn update(&mut self, input : &input::InputHandler) {
		self.hovered = utils::check_collision(
			self.x, self.y,
			self.size, 1,
			input.mouse.x, input.mouse.y
		);

		if self.hovered {
			read_line_from_keys(&input.keyboard.key_events, &mut self.output);
			self.output = self.output.trim().to_string();
		}
	}

	fn redraw_requested(&mut self) -> bool {
		if self.output != self.output_prev {
			self.output_prev = self.output.clone();
			return true;
		}
		false
	}

	fn set_position(&mut self, x : i16, y : i16, anchor : u8, size : (u16, u16)) {
		self.x = (x + if anchor == 1 || anchor == 3 { size.0 } else {0} as i16) as u16;
		self.y = (y + if anchor == 2 || anchor == 3 { size.1 } else {0} as i16) as u16;

		if anchor == 4 {
			self.x = (x + size.0 as i16 /2) as u16 -(self.size/2);
			self.y = (y + size.1 as i16 /2) as u16;
		}
	}

	fn set_color(&mut self, color : bool) {
		self.color.color_enabled = color;
	}
}

impl InputField {
	pub fn new(
		nx : u16, ny : u16,
		nsize : u16) -> Self
	{
		InputField {
			x : nx,
			y : ny,
			size : nsize,
			output : "".to_string(),
			output_prev : "".to_string(),

			color : utils::Color {
				color_enabled : true,
			
				color : 0,
				bright : false,
				
				truecolor : false,
				red : 0,
				green : 0,
				blue : 0,
			},

			bgcolor : utils::Color {
				color_enabled : true,
						
				color : 7,
				bright : false,
							
				truecolor : false,
				red : 0,
				green : 0,
				blue : 0,
			},
			
			hovered : false,
			cursor : 0
		}
	}
}

fn read_line_from_keys(
	events : &[KeyEvent],
	buffer : &mut String,
) -> Option<String> {
    for event in events {
        if event.kind != KeyEventKind::Press {
            continue;
        }

        match event.code {
            KeyCode::Char(c) => buffer.push(c),
            KeyCode::Backspace => { buffer.pop(); }
            KeyCode::Enter => {
                let line = buffer.clone();
                buffer.clear();
                return Some(line);
            }
            _ => {}
        }
    }
    None
}
