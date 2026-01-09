use std::{
	io::{
		Write,
		Stdout,
	},
	rc::Rc,
	cell::RefCell,
};
use crate::{
	interface::{
		textbox,
		traits
	},
	helpers::input,
	helpers::utils,
	abt::menus,
	menus::menu_traits,
};


pub struct UserSelect {
	tier : textbox::Box,
	menu : Rc<RefCell<menus::Menu>>
}

impl menu_traits::Menu for UserSelect {
	fn init(menu_ref : Rc<RefCell<menus::Menu>>) -> Self {
		let tier_label : textbox::Box = textbox::Box::new(15, 3, 5, 3);
		
		Self {
			tier : tier_label,
			menu : menu_ref
		}
	}

	fn redraw(&mut self, input : &input::InputHandler, out : &mut Stdout) {
		write!(out, "\x1b[2J").unwrap();

		self.update(input, out);

		traits::UserInterface::draw(&self.tier, out);
	}

	fn tick(&mut self, input : &input::InputHandler, out : &mut Stdout) {

		if input.window.request_full_redraw {
			self.redraw(input, out);
		}

		self.update(input, out);
	}
}

impl UserSelect {
	fn update(&mut self, input : &input::InputHandler, out : &mut Stdout) {

		utils::object(&mut self.tier, input, &self.menu, menus::Menu::Main,
		(-7, 3), (-7, 3), 1, out);
	}
}
