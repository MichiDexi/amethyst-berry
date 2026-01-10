use std::{
	io::Stdout,
	rc::Rc,
	cell::RefCell,
};
use crate::{
	helpers::input,
	abt::menus,
	abt::data,
};

pub trait Menu {
	fn init(menu_ref : Rc<RefCell<menus::Menu>>, data_ref : Rc<RefCell<data::Data>>) -> Self;
	fn redraw(&mut self, input : &input::InputHandler, out : &mut Stdout);
	fn tick(&mut self, input : &input::InputHandler, out : &mut Stdout);
}
