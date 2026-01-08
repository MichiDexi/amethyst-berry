use std::{
	io::Stdout,
	rc::Rc,
	cell::RefCell,
};
use crate::{
	helpers::input,
	abt::menus,
};

pub trait Menu {
	fn init(menu_ref : Rc<RefCell<menus::Menu>>) -> Self;
	fn redraw(&mut self, input : &input::InputHandler, out : &mut Stdout);
	fn tick(&mut self, input : &input::InputHandler, out : &mut Stdout);
}
