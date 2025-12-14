use crossterm::{
	execute,
	cursor::MoveTo,
};
use std::io::{stdout, Write, Stdout};

use crate::helpers::utils;
use crate::helpers::input;

pub struct List {
	// Size and position
	pub x : u16, pub y : u16,
	pub width : u16, pub height : u16,

	// Contents
	pub items : Vec<String>,

	// Extra options
	pub color : utils::Color,
	pub color_hovered : utils::Color,

	// Event polling
	pub hovered : u16,

	// Internal
	index : u16,
}


impl List {
	pub fn new(
		nx : u16, ny : u16,
		nwidth : u16, nheight : u16
	) -> Self {
		List {
			x : nx, y : ny,
			width : nwidth, height : nheight,

			items : vec!(),
			
			color : utils::Color {
				color_enabled : true,
				color : 7,
				bright : false,

				truecolor : false,
				red : 0,
				green : 0,
				blue : 0,
			},

			color_hovered : utils::Color {
				color_enabled : true,
				color : 6,
				bright : false,

				truecolor : false,
				red : 0,
				green : 0,
				blue : 0,
			},

			hovered : 0,
			index : 0,
		}
	}
	
	pub fn draw(&self, out : &mut Stdout) {

		for i in 0..self.height {
			if self.index+i >= self.items.len() as u16 {
				break;
			}
			
			if self.hovered == self.index+i {
				self.color_hovered.write_color(out, false);
			}
			else {
				self.color.write_color(out, false);
			}
			
			execute!(out, crossterm::cursor::MoveTo(self.x, self.y+i)).unwrap();
			write!(out, "{}", utils::shorten_text(&self.items[(self.index+i) as usize], self.width)).unwrap();
		}

		write!(out, "\x1b[0m").unwrap();

		stdout().flush().unwrap();
	}

	pub fn update(&mut self, mouse : &input::Mouse) {
	
		if utils::check_collision(
			self.x, self.y,
			self.width, self.height,
			mouse.x, mouse.y
		) { // This is true when the mouse is hovering something
			self.hovered = mouse.y + self.index - self.y + 1; // +1 because 0 is nothing, 1 is the first etc.
		}

		if mouse.scroll == 0 {
			return;
		}

		if (self.index as i32 - mouse.scroll as i32) < 0 {
			self.index = 0;
		}
		else if self.index as i32 - mouse.scroll as i32 >= self.items.len() as i32 - self.height as i32 {
			self.index = self.items.len() as u16 - self.height;
		}
		else {
			match mouse.scroll {
				1 => self.index -= 1,
				-1 => self.index += 1,
				_ => {}
			}
		}
	}
}
