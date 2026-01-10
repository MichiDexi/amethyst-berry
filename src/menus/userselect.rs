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
		label,
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
	submenu : Option<u8>,
	
	create_button : label::Label,
	create_submenu : Create,
	rename_button : label::Label,
	rename_submenu : Rename,
	delete_button : label::Label,
	delete_submenu : Delete,
	open_button : label::Label,

	tier : textbox::Box,
	menu : Rc<RefCell<menus::Menu>>,
}

pub struct Create {
	decoration : textbox::Box,
	message : label::Label,
	input : inputfield::InputField,
	message_fail : label::Label
}

pub struct Rename {
	decoration : textbox::Box,
	message : label::Label,
	input : inputfield::InputField,
	message_fail : label::Label
}

pub struct Delete {
	decoration : textbox::Box,
	message : label::Label,
	yes_option : label::Label,
	no_option : label::Label,
	message_fail : label::Label
}

impl menu_traits::Menu for UserSelect {
	fn init(menu_ref : Rc<RefCell<menus::Menu>>) -> Self {
		let mut user_list : list::List = list::List::new(3, 1, 5, 5);
		user_list.items = crate::abt::fs::users::list();
		
		
		Self {
			users : user_list,
			decoration : textbox::Box::new(3, 1, 5, 5),
			submenu : None,
			
			create_button : label::Label::new(8, " Create"),
			create_submenu : Create {
				decoration : textbox::Box::new(3, 1, 5, 5),
				message : label::Label::new(19, " Create a new user"),
				input : inputfield::InputField::new(0, 0, 15),
				message_fail : label::Label::new(26, "Couldn't create new user"),
			},

			rename_button : label::Label::new(8, " Rename"),
			rename_submenu : Rename {
				decoration : textbox::Box::new(3, 1, 5, 5),
				message : label::Label::new(19, " Rename a the user"),
				input : inputfield::InputField::new(0, 0, 15),
				message_fail : label::Label::new(26, "Couldn't rename the user"),
			},

			delete_button : label::Label::new(8, " Delete"),
			delete_submenu : Delete {
				decoration : textbox::Box::new(3, 1, 5, 5),
				message : label::Label::new(16, " Are you sure?"),
				yes_option : label::Label::new(5, " Yes"),
				no_option : label::Label::new(5, " No"),
				message_fail : label::Label::new(26, "Couldn't rename the user"),
			},
			
			open_button : label::Label::new(8, "  Open"),
			
			tier : textbox::Box::new(15, 3, 5, 3),
			menu : menu_ref,
		}
	}

	fn redraw(&mut self, input : &input::InputHandler, out : &mut Stdout) {
		write!(out, "\x1b[2J").unwrap();

		self.update(input, out);

		traits::UserInterface::draw(&self.users, out);
		traits::UserInterface::draw(&self.decoration, out);
		traits::UserInterface::draw(&self.create_button, out);
		traits::UserInterface::draw(&self.tier, out);
		traits::UserInterface::draw(&self.create_submenu.message_fail, out);

		match self.submenu {
			Some(0) => {
				traits::UserInterface::draw(&self.create_submenu.decoration, out);
				traits::UserInterface::draw(&self.create_submenu.message, out);
				traits::UserInterface::draw(&self.create_submenu.input, out);
			},
			Some(_) => {
				self.submenu = None;
			}
			None => { }
		}
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

		let submenu_prev = self.submenu;

		match self.submenu {
			None => {
				let selected_prev : bool = self.users.selected.is_some();
				
				utils::object(&mut self.users, input, &self.menu, menus::Menu::UserSelect,
				(4, 2), (4, 2), 0, out);

				utils::object(&mut self.create_button, input, &self.menu, menus::Menu::UserSelect,
				(-10, 10), (-10, 10), 1, out);
				if self.create_button.clicked {
					self.submenu = Some(0);
				}

				if self.users.selected.is_some() != selected_prev {
					if self.users.selected.is_some() {

						utils::object(&mut self.rename_button, input, &self.menu, menus::Menu::UserSelect,
						(-10, 12), (-10, 12), 1, out);

						utils::object(&mut self.delete_button, input, &self.menu, menus::Menu::UserSelect,
						(-10, 14), (-10, 14), 1, out);

						utils::object(&mut self.open_button, input, &self.menu, menus::Menu::UserSelect,
						(-10, 16), (-10, 16), 1, out);

						traits::UserInterface::draw(&self.rename_button, out);
						traits::UserInterface::draw(&self.delete_button, out);
						traits::UserInterface::draw(&self.open_button, out);
					}
					else {
						traits::UserInterface::clear(&self.rename_button, out);
						traits::UserInterface::clear(&self.delete_button, out);
						traits::UserInterface::clear(&self.open_button, out);
					}
				}

				utils::object(&mut self.tier, input, &self.menu, menus::Menu::Main,
				(-7, 3), (-7, 3), 1, out);
			},
			Some(0) => {
				utils::object(&mut self.create_submenu.input, input, &self.menu, menus::Menu::UserSelect,
				(-10, 12), (-10, 12), 1, out);
			},
			_ => {
				self.submenu = None;
			}
		}

		if self.submenu != submenu_prev {
			menu_traits::Menu::redraw(self, input, out);
		}
	}
}
