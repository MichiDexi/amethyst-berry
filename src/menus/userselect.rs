use std::io::Write;
use std::io::Stdout;
use std::rc::Rc;
use std::cell::RefCell;

use crate::interface::label;
use crate::interface::list;
use crate::interface::textbox;
use crate::interface::traits;
use crate::interface::inputfield;
use crate::helpers::input;
use crate::helpers::utils;
use crate::abt::menus;
use crate::abt::data;
use crate::menus::menu_traits;


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
	data : Rc<RefCell<data::Data>>,
	message_timer : u8,
}

pub struct Create {
	decoration : textbox::Box,
	message : label::Label,
	input : inputfield::InputField,
	message_fail : label::Label,
	confirm : label::Label,
	cancel : label::Label,
}

pub struct Rename {
	decoration : textbox::Box,
	message : label::Label,
	input : inputfield::InputField,
	message_fail : label::Label,
	confirm : label::Label,
	cancel : label::Label,
}

pub struct Delete {
	decoration : textbox::Box,
	message : label::Label,
	confirm : label::Label,
	cancel : label::Label,
	message_fail : label::Label,
}

impl menu_traits::Menu for UserSelect {
	fn init(menu_ref : Rc<RefCell<menus::Menu>>, data_ref : Rc<RefCell<data::Data>>) -> Self {
		let mut user_list : list::List = list::List::new(3, 1, 5, 5);
		update_userlist(&mut user_list);

		Self {
			users : user_list,
			decoration : textbox::Box::new(3, 1, 5, 5),
			submenu : None,
			
			create_button : label::Label::new(8, " Create"),
			create_submenu : Create {
				decoration : textbox::Box::new(10, 10, 25, 9),
				message : label::Label::new(19, " Create a new user"),
				input : inputfield::InputField::new(0, 0, 15),
				message_fail : label::Label::new(26, ""),
				confirm : label::Label::new(9, " Confirm"),
				cancel : label::Label::new(8, " Cancel"),
			},

			rename_button : label::Label::new(8, " Rename"),
			rename_submenu : Rename {
				decoration : textbox::Box::new(3, 1, 5, 5),
				message : label::Label::new(19, " Rename a the user"),
				input : inputfield::InputField::new(0, 0, 15),
				message_fail : label::Label::new(26, "Couldn't rename the user"),
				confirm : label::Label::new(9, " Confirm"),
				cancel : label::Label::new(8, " Cancel"),
			},

			delete_button : label::Label::new(8, " Delete"),
			delete_submenu : Delete {
				decoration : textbox::Box::new(3, 1, 5, 5),
				message : label::Label::new(16, " Are you sure?"),
				confirm : label::Label::new(9, " Confirm"),
				cancel : label::Label::new(8, " Cancel"),
				message_fail : label::Label::new(26, "Couldn't rename the user"),
			},
			
			open_button : label::Label::new(8, "  Open"),
			
			tier : textbox::Box::new(15, 3, 5, 3),
			menu : menu_ref,
			data : data_ref,
			message_timer : 0,
		}
	}

	fn redraw(&mut self, input : &input::InputHandler, out : &mut Stdout) {
		write!(out, "\x1b[2J").unwrap();

		self.update(input, out);

		traits::UserInterface::draw(&self.users, out);
		traits::UserInterface::draw(&self.decoration, out);
		traits::UserInterface::draw(&self.create_button, out);
		traits::UserInterface::draw(&self.tier, out);

		match self.submenu {
			Some(0) => {
				traits::UserInterface::draw(&self.create_submenu.decoration, out);
				traits::UserInterface::draw(&self.create_submenu.message, out);
				traits::UserInterface::draw(&self.create_submenu.input, out);
				traits::UserInterface::draw(&self.create_submenu.confirm, out);
				traits::UserInterface::draw(&self.create_submenu.cancel, out);
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

		if self.message_timer != 0 {
			self.message_timer -= 1;
			if !self.create_submenu.message_fail.text.is_empty() {
				traits::UserInterface::draw(&self.create_submenu.message_fail, out);
			}
			else if !self.rename_submenu.message_fail.text.is_empty() {
				traits::UserInterface::draw(&self.rename_submenu.message_fail, out);
			}
			else if !self.delete_submenu.message_fail.text.is_empty() {
				traits::UserInterface::draw(&self.delete_submenu.message_fail, out);
			}
			else {
				self.message_timer = 0;
			}
		}
		else {
			traits::UserInterface::clear(&self.create_submenu.message_fail, out);
			traits::UserInterface::clear(&self.rename_submenu.message_fail, out);
			traits::UserInterface::clear(&self.delete_submenu.message_fail, out);
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
		self.create_submenu.message_fail.size = input.window.width -8;

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
				utils::object(&mut self.create_submenu.decoration, input, &self.menu, menus::Menu::UserSelect,
				(0, 0), (0, 0), 4, out);
				
				utils::object(&mut self.create_submenu.message, input, &self.menu, menus::Menu::UserSelect,
				(0, -2), (0, -2), 4, out);

				utils::object(&mut self.create_submenu.input, input, &self.menu, menus::Menu::UserSelect,
				(0, 0), (0, 0), 4, out);

				utils::object(&mut self.create_submenu.confirm, input, &self.menu, menus::Menu::UserSelect,
				(-4, 2), (-4, 2), 4, out);

				utils::object(&mut self.create_submenu.cancel, input, &self.menu, menus::Menu::UserSelect,
				(5, 2), (5, 2), 4, out);

				utils::object(&mut self.create_submenu.message_fail, input, &self.menu, menus::Menu::UserSelect,
				(5, input.window.height as i16-2), (5, input.window.height as i16-2), 4, out);
				
				if self.create_submenu.confirm.clicked {
					if let Err(v) = crate::abt::fs::users::create(&self.create_submenu.input.output) {
						self.create_submenu.message_fail.text = v.to_string();
					}
					self.message_timer = 100;
					self.create_submenu.input.reset();
					update_userlist(&mut self.users);
					self.submenu = None;
				}
				
				if self.create_submenu.cancel.clicked {
					self.create_submenu.input.reset();
					self.submenu = None;
				}
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

fn update_userlist(list : &mut list::List) {
	list.items = crate::abt::fs::users::list();
}
