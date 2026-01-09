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
		list,
		textbox,
		traits
	},
	helpers::input,
	helpers::utils,
	abt::menus,
	menus::menu_traits,
};


pub struct UserSelect {
	users : list::List,
	decoration : textbox::Box,
	tier : textbox::Box,
	menu : Rc<RefCell<menus::Menu>>
}

impl menu_traits::Menu for UserSelect {
	fn init(menu_ref : Rc<RefCell<menus::Menu>>) -> Self {
		let mut user_list : list::List = list::List::new(4, 2, 5, 5);
		let deco_box : textbox::Box = textbox::Box::new(3, 1, 6, 6);
		let tier_label : textbox::Box = textbox::Box::new(15, 3, 5, 3);

		for _ in 0..10 {
			user_list.items.push("a".to_string());
			user_list.items.push("b".to_string());
		}
		
		Self {
			users : user_list,
			decoration : deco_box,
			tier : tier_label,
			menu : menu_ref
		}
	}

	fn redraw(&mut self, input : &input::InputHandler, out : &mut Stdout) {
		write!(out, "\x1b[2J").unwrap();

		self.update(input, out);

		traits::UserInterface::draw(&self.users, out);
		traits::UserInterface::draw(&self.decoration, out);
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
		utils::object(&mut self.users, input, &self.menu, menus::Menu::UserSelect,
		(15, 3), (15, 3), 0, out);

		utils::object(&mut self.tier, input, &self.menu, menus::Menu::Main,
		(-7, 3), (-7, 3), 1, out);
	}
}
