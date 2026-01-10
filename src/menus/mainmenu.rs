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
		textbox,
		traits
	},
	helpers::input,
	helpers::utils,
	abt::menus,
	abt::data,
	menus::menu_traits,
};


pub struct MainMenu {
	maps : label::Label,
	mods : label::Label,
	wiki : label::Label,
	launch : label::Label,
	speedruns : label::Label,
	challenges : label::Label,
	tier : textbox::Box,
	menu : Rc<RefCell<menus::Menu>>
}

impl menu_traits::Menu for MainMenu {
	fn init(menu_ref : Rc<RefCell<menus::Menu>>, _data_ref : Rc<RefCell<data::Data>>) -> Self {
		let maps_label : label::Label = label::Label::new(12, "Maps");
		let mods_label : label::Label = label::Label::new(12, "Mods");
		let wiki_label : label::Label = label::Label::new(12, "Wiki");
		let launch_label : label::Label = label::Label::new(12, "Launch");
		let speedruns_label : label::Label = label::Label::new(12, "Speedruns");
		let challenges_label : label::Label = label::Label::new(12, "Challenges");
		let tier_label : textbox::Box = textbox::Box::new(15, 3, 5, 3);
		
		Self {
			maps : maps_label,
			mods : mods_label,
			wiki : wiki_label,
			launch : launch_label,
			speedruns : speedruns_label,
			challenges : challenges_label,
			tier : tier_label,
			menu : menu_ref
		}
	}

	fn redraw(&mut self, input : &input::InputHandler, out : &mut Stdout) {
		write!(out, "\x1b[2J").unwrap();

		self.update(input, out);

		traits::UserInterface::draw(&self.maps, out);
		traits::UserInterface::draw(&self.mods, out);
		traits::UserInterface::draw(&self.wiki, out);
		traits::UserInterface::draw(&self.launch, out);
		traits::UserInterface::draw(&self.speedruns, out);
		traits::UserInterface::draw(&self.challenges, out);
		traits::UserInterface::draw(&self.tier, out);
	}

	fn tick(&mut self, input : &input::InputHandler, out : &mut Stdout) {

		if input.window.request_full_redraw {
			self.redraw(input, out);
		}

		self.update(input, out);
	}
}

impl MainMenu {
	fn update(&mut self, input : &input::InputHandler, out : &mut Stdout) {
		utils::object(&mut self.maps, input, &self.menu, menus::Menu::UserSelect,
		(2, 2), (3, 2), 0, out);

		utils::object(&mut self.mods, input, &self.menu, menus::Menu::UserSelect,
		(2, 4), (3, 4), 0, out);
		
		utils::object(&mut self.wiki, input, &self.menu, menus::Menu::UserSelect,
		(2, 6), (3, 6), 0, out);
		
		utils::object(&mut self.launch, input, &self.menu, menus::Menu::UserSelect,
		(2, 8), (3, 8), 0, out);

		utils::object(&mut self.speedruns, input, &self.menu, menus::Menu::UserSelect,
		(2, 10), (3, 10), 0, out);

		utils::object(&mut self.challenges, input, &self.menu, menus::Menu::UserSelect,
		(2, 12), (3, 12), 0, out);

		utils::object(&mut self.tier, input, &self.menu, menus::Menu::Out,
		(-7, 3), (-7, 3), 1, out);
	}
}
