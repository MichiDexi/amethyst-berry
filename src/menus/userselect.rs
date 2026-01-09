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
		traits,
		inputfield
	},
	helpers::input,
	helpers::utils,
	abt::menus,
	menus::menu_traits,
};


pub struct UserSelect {
	users : list::List,
	decoration : textbox::Box,
	create : inputfield::InputField,
	rename : inputfield::InputField,
	delete : inputfield::InputField,
	tier : textbox::Box,
	menu : Rc<RefCell<menus::Menu>>,
}

impl menu_traits::Menu for UserSelect {
	fn init(menu_ref : Rc<RefCell<menus::Menu>>) -> Self {
		let mut user_list : list::List = list::List::new(3, 1, 5, 5);
		let deco_box : textbox::Box = textbox::Box::new(3, 1, 5, 5);
		let tier_label : textbox::Box = textbox::Box::new(15, 3, 5, 3);

		let create_if : inputfield::InputField = inputfield::InputField::new(0, 0, 0, 0, 8, " Create");
		let rename_if : inputfield::InputField = inputfield::InputField::new(0, 0, 0, 0, 8, " Rename");
		let delete_if : inputfield::InputField = inputfield::InputField::new(0, 0, 0, 0, 8, " Delete");

		user_list.items = crate::abt::fs::users::list();
		
		
		Self {
			users : user_list,
			decoration : deco_box,
			create : create_if,
			rename : rename_if,
			delete : delete_if,
			tier : tier_label,
			menu : menu_ref,
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
		self.decoration.width = input.window.width -14;
		self.decoration.height = input.window.height -2;
		self.users.width = input.window.width -14 -2;
		self.users.height = input.window.height -2 -2;
		
		utils::object(&mut self.users, input, &self.menu, menus::Menu::UserSelect,
		(4, 2), (4, 2), 0, out);

		utils::object(&mut self.create, input, &self.menu, menus::Menu::UserSelect,
		(-10, 10), (-10, 10), 1, out);

		utils::object(&mut self.tier, input, &self.menu, menus::Menu::Main,
		(-7, 3), (-7, 3), 1, out);
	}
}
