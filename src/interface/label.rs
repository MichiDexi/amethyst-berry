use crossterm::{
	execute,
	cursor::MoveTo,
};
use std::io::{stdout, Write};

use crate::utils;

pub struct Label {
	// Size and position
	pub x : u16, pub y : u16,
	pub size : u16,
	pub text : String,

	// Extra options
	pub color : utils::Color,
	pub bg_color : utils::Color,
}


impl Label {
	pub fn draw(&self) {
		let mut out = stdout();

		execute!(out, crossterm::cursor::MoveTo(self.x, self.y)).unwrap();
		write!(out, "{}", utils::shorten_text(&self.text, self.size)).unwrap();

		stdout().flush().unwrap();
	}
}
