use std::io::Stdout;
use crate::helpers::input;

pub trait UserInterface {
	fn draw(&self, out : &mut Stdout);
	fn clear(&self, out : &mut Stdout);
	fn update(&mut self, input: &input::InputHandler);
}
