use std::io::{
	Write,
	Stdout,
};
use crossterm::{
	execute,
};

use crate::helpers::utils;
use crate::helpers::input;
use crate::interface::traits;


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
	redraw_timer : u8,
}

impl traits::UserInterface for List {
	fn draw(&self, out : &mut Stdout) {

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
	}

	fn clear(&self, out : &mut Stdout) {

		for i in 0..self.height {
			execute!(out, crossterm::cursor::MoveTo(self.x, self.y+i)).unwrap();
			utils::repeat(out, ' ', self.width);
		}
	}
	
	fn update(&mut self, input : &input::InputHandler) {
	
		if utils::check_collision(
			self.x, self.y,
			self.width, self.height,
			input.mouse.x, input.mouse.y
		) { // This is true when the mouse is hovering something
			self.hovered = input.mouse.y + self.index - self.y; // +1 because 0 is nothing, 1 is the first etc.
		}
		else {
			return;
		}

		if input.mouse.scroll == 0 ||
			self.height as usize >= self.items.len()
		{
			return;
		}

		if input.mouse.scroll as i32 == 1 &&
			self.index == 0
		{
			return;
		}

		if input.mouse.scroll as i32 == -1 &&
			self.index == self.items.len() as u16 - self.height
		{
			return;
		}

		match input.mouse.scroll {
			1 => self.index -= 1,
			-1 => self.index += 1,
			_ => {}
		}
	}

	fn redraw_requested(&mut self) -> bool {
		if self.redraw_timer == 255 {
			self.redraw_timer = 0;
		}
		else {
			self.redraw_timer += 1;
		}
		self.redraw_timer.is_multiple_of(2)
	}

	fn set_position(&mut self, x : i16, y : i16, anchor : u8, size : (u16, u16)) {
		self.x = (x + if anchor == 1 || anchor == 3 { size.0 } else {0} as i16) as u16;
		self.y = (y + if anchor == 2 || anchor == 3 { size.1 } else {0} as i16) as u16;
	}

	fn set_color(&mut self, color : bool) {
		self.color.color_enabled = color;
	}
}

impl List {
	pub fn new(nx : u16, ny : u16, nwidth : u16, nheight : u16) -> Self {
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

			redraw_timer : 0
		}
	}
}
