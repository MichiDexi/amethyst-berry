use std::io::Stdout;
use crate::helpers::input;

pub trait UserInterface {
	fn draw(&self, out : &mut Stdout);
	fn clear(&self, out : &mut Stdout);
	fn update(&mut self, input : &input::InputHandler);
	fn redraw_requested(&self) -> bool;
	fn set_position(&mut self, x : i16, y : i16, anchor : u8, size : (u16, u16));
	fn set_color(&mut self, color : bool);
}
