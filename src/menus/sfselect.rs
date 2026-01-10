use std::io::Write;
use std::io::Stdout;
use std::rc::Rc;
use std::cell::RefCell;

use crate::interface::label;
use crate::interface::list;
use crate::interface::textbox;
use crate::interface::traits;
use crate::interface::inputfield;
use crate::interface::checkbox;
use crate::helpers::input;
use crate::helpers::utils;
use crate::abt::menus;
use crate::abt::data;
use crate::menus::menu_traits;


pub struct SaveSelect {
	savefile_list : list::List,
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
	lobby_based : checkbox::CheckBox,
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

impl menu_traits::Menu for SaveSelect {
	fn init(menu_ref : Rc<RefCell<menus::Menu>>, data_ref : Rc<RefCell<data::Data>>) -> Self {
		let mut savelist : list::List = list::List::new(3, 1, 5, 5);
		if let Some(user) = data_ref.borrow().user.clone() {
			update_savelist(&mut savelist, &user);
		}

		Self {
			savefile_list : savelist,
			decoration : textbox::Box::new(3, 1, 5, 5),
			submenu : None,
			
			create_button : label::Label::new(8, " Create"),
			create_submenu : Create {
				decoration : textbox::Box::new(10, 10, 25, 9),
				message : label::Label::new(19, " Create a new user"),
				input : inputfield::InputField::new(0, 0, 15),
				lobby_based : checkbox::CheckBox::new(0, 0),
				message_fail : label::Label::new(26, ""),
				confirm : label::Label::new(9, " Confirm"),
				cancel : label::Label::new(8, " Cancel"),
			},

			rename_button : label::Label::new(8, " Rename"),
			rename_submenu : Rename {
				decoration : textbox::Box::new(10, 10, 25, 9),
				message : label::Label::new(19, " Rename the user"),
				input : inputfield::InputField::new(0, 0, 15),
				message_fail : label::Label::new(26, ""),
				confirm : label::Label::new(9, " Confirm"),
				cancel : label::Label::new(8, " Cancel"),
			},

			delete_button : label::Label::new(8, " Delete"),
			delete_submenu : Delete {
				decoration : textbox::Box::new(10, 10, 25, 9),
				message : label::Label::new(16, " Are you sure?"),
				confirm : label::Label::new(9, " Confirm"),
				cancel : label::Label::new(8, " Cancel"),
				message_fail : label::Label::new(26, ""),
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

		traits::UserInterface::draw(&self.savefile_list, out);
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
			Some(1) => {
				traits::UserInterface::draw(&self.rename_submenu.decoration, out);
				traits::UserInterface::draw(&self.rename_submenu.message, out);
				traits::UserInterface::draw(&self.rename_submenu.input, out);
				traits::UserInterface::draw(&self.rename_submenu.confirm, out);
				traits::UserInterface::draw(&self.rename_submenu.cancel, out);
			},
			Some(2) => {
				traits::UserInterface::draw(&self.delete_submenu.decoration, out);
				traits::UserInterface::draw(&self.delete_submenu.message, out);
				traits::UserInterface::draw(&self.delete_submenu.confirm, out);
				traits::UserInterface::draw(&self.delete_submenu.cancel, out);
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

impl SaveSelect {
	fn update(&mut self, input : &input::InputHandler, out : &mut Stdout) {
		self.decoration.width = input.window.width -14;
		self.decoration.height = input.window.height -2;
		self.savefile_list.width = input.window.width -14 -2;
		self.savefile_list.height = input.window.height -2 -2;
		self.create_submenu.message_fail.size = input.window.width -8;

		let submenu_prev = self.submenu;

		match self.submenu {
			None => {
				let selected_prev : bool = self.savefile_list.selected.is_some();
				
				utils::object(&mut self.savefile_list, input, &self.menu, menus::Menu::SavefileSelect,
				(4, 2), (4, 2), 0, out);

				utils::object(&mut self.create_button, input, &self.menu, menus::Menu::SavefileSelect,
				(-10, 10), (-10, 10), 1, out);
				
				utils::object(&mut self.rename_button, input, &self.menu, menus::Menu::SavefileSelect,
				(-10, 12), (-10, 12), 1, out);

				utils::object(&mut self.delete_button, input, &self.menu, menus::Menu::SavefileSelect,
				(-10, 14), (-10, 14), 1, out);

				utils::object(&mut self.open_button, input, &self.menu, menus::Menu::LobbySelect,
				(-10, 16), (-10, 16), 1, out);
					
				if self.savefile_list.selected.is_some() {
					if self.savefile_list.selected.is_some() != selected_prev {
						traits::UserInterface::draw(&self.rename_button, out);
						traits::UserInterface::draw(&self.delete_button, out);
						traits::UserInterface::draw(&self.open_button, out);
					}
				}
				else if self.savefile_list.selected.is_some() != selected_prev {
					traits::UserInterface::clear(&self.rename_button, out);
					traits::UserInterface::clear(&self.delete_button, out);
					traits::UserInterface::clear(&self.open_button, out);
				}

				if self.create_button.clicked {
					self.submenu = Some(0);
				}
				if self.rename_button.clicked {
					self.submenu = Some(1);
				}
				if self.delete_button.clicked {
					self.submenu = Some(2);
				}
				if let Some(selected) = self.savefile_list.selected && self.open_button.clicked {
					self.data.borrow_mut().user = Some(self.savefile_list.items[selected as usize].clone());
				}

				utils::object(&mut self.tier, input, &self.menu, menus::Menu::Main,
				(-7, 3), (-7, 3), 1, out);
			},
			
			Some(0) => {
				utils::object(&mut self.create_submenu.decoration, input, &self.menu, menus::Menu::SavefileSelect,
				(0, 0), (0, 0), 4, out);
				
				utils::object(&mut self.create_submenu.message, input, &self.menu, menus::Menu::SavefileSelect,
				(0, -2), (0, -2), 4, out);

				utils::object(&mut self.create_submenu.input, input, &self.menu, menus::Menu::SavefileSelect,
				(0, 0), (0, 0), 4, out);

				utils::object(&mut self.create_submenu.confirm, input, &self.menu, menus::Menu::SavefileSelect,
				(-4, 2), (-4, 2), 4, out);

				utils::object(&mut self.create_submenu.cancel, input, &self.menu, menus::Menu::SavefileSelect,
				(5, 2), (5, 2), 4, out);

				utils::object(&mut self.create_submenu.message_fail, input, &self.menu, menus::Menu::SavefileSelect,
				(4, input.window.height as i16-1), (4, input.window.height as i16-1), 0, out);
				
				if self.create_submenu.confirm.clicked {
					if let Some(user) = self.data.borrow().user.clone() &&
						let Some(selected) = self.savefile_list.selected &&
						let Err(e) = crate::abt::fs::sf::create(
						&user, &self.savefile_list.items[selected as usize], self.create_submenu.lobby_based.checked)
					{
						self.create_submenu.message_fail.text = e.to_string();
					}
					self.message_timer = 100;
					self.create_submenu.input.reset();
					if let Some(user) = self.data.borrow().user.clone() {
						update_savelist(&mut self.savefile_list, &user);
					}
					self.submenu = None;
				}
				
				if self.create_submenu.cancel.clicked {
					self.create_submenu.input.reset();
					self.submenu = None;
				}
			},

			Some(1) => {
				utils::object(&mut self.rename_submenu.decoration, input, &self.menu, menus::Menu::UserSelect,
				(0, 0), (0, 0), 4, out);
				
				utils::object(&mut self.rename_submenu.message, input, &self.menu, menus::Menu::UserSelect,
				(0, -2), (0, -2), 4, out);

				utils::object(&mut self.rename_submenu.input, input, &self.menu, menus::Menu::UserSelect,
				(0, 0), (0, 0), 4, out);

				utils::object(&mut self.rename_submenu.confirm, input, &self.menu, menus::Menu::UserSelect,
				(-4, 2), (-4, 2), 4, out);

				utils::object(&mut self.rename_submenu.cancel, input, &self.menu, menus::Menu::UserSelect,
				(5, 2), (5, 2), 4, out);

				utils::object(&mut self.rename_submenu.message_fail, input, &self.menu, menus::Menu::UserSelect,
				(4, input.window.height as i16-1), (4, input.window.height as i16-1), 0, out);
				
				if self.rename_submenu.confirm.clicked {
					if let Some(user) = self.data.borrow().user.clone() &&
						let Some(selected) = self.savefile_list.selected &&
						let Err(e) = crate::abt::fs::sf::rename(
						&user, &self.savefile_list.items[selected as usize], &self.rename_submenu.input.output)
					{
						self.rename_submenu.message_fail.text = e.to_string();
					}
					self.message_timer = 100;
					self.rename_submenu.input.reset();
					if let Some(user) = self.data.borrow().user.clone() {
						update_savelist(&mut self.savefile_list, &user);
					}
					self.submenu = None;
				}
				
				if self.rename_submenu.cancel.clicked {
					self.rename_submenu.input.reset();
					self.submenu = None;
				}
			},

			Some(2) => {
				utils::object(&mut self.delete_submenu.decoration, input, &self.menu, menus::Menu::UserSelect,
				(0, 0), (0, 0), 4, out);
				
				utils::object(&mut self.delete_submenu.message, input, &self.menu, menus::Menu::UserSelect,
				(0, -1), (0, -1), 4, out);

				utils::object(&mut self.delete_submenu.confirm, input, &self.menu, menus::Menu::UserSelect,
				(-4, 1), (-4, 1), 4, out);

				utils::object(&mut self.delete_submenu.cancel, input, &self.menu, menus::Menu::UserSelect,
				(5, 1), (5, 1), 4, out);

				utils::object(&mut self.delete_submenu.message_fail, input, &self.menu, menus::Menu::UserSelect,
				(4, input.window.height as i16-1), (4, input.window.height as i16-1), 0, out);
				
				if self.delete_submenu.confirm.clicked {
					if let Some(user) = self.data.borrow().user.clone() &&
						let Some(selected) = self.savefile_list.selected &&
						let Err(e) = crate::abt::fs::sf::delete(
						&user, &self.savefile_list.items[selected as usize])
					{
						self.delete_submenu.message_fail.text = e.to_string();
					}
					self.message_timer = 100;
					if let Some(user) = self.data.borrow().user.clone() {
						update_savelist(&mut self.savefile_list, &user);
					}
					self.submenu = None;
				}
				
				if self.delete_submenu.cancel.clicked {
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

fn update_savelist(list : &mut list::List, user : &str) {
	list.items = crate::abt::fs::sf::list(user);
}
