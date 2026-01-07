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
	abt::menus,
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

impl MainMenu {
	pub fn init(menu_ref : Rc<RefCell<menus::Menu>>) -> Self {
		let maps_label : label::Label = label::Label::new(12, "Maps");
		let mods_label : label::Label = label::Label::new(12, "Mods");
		let wiki_label : label::Label = label::Label::new(12, "Wiki");
		let launch_label : label::Label = label::Label::new(12, "Launch");
		let speedruns_label : label::Label = label::Label::new(12, "Speedruns");
		let challenges_label : label::Label = label::Label::new(12, "Challenges");
		let tier_label : textbox::Box = textbox::Box::new(15, 3, 7, 7);
		
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

	pub fn init_draw(&mut self, input : &input::InputHandler, out : &mut Stdout) {
		write!(out, "\x1b[2J").unwrap();

		object(&mut self.maps, input, &self.menu, menus::Menu::UserSelect,
		(2, 2), (3, 2), out);

		object(&mut self.mods, input, &self.menu, menus::Menu::UserSelect,
		(2, 4), (3, 4), out);
		
		object(&mut self.wiki, input, &self.menu, menus::Menu::UserSelect,
		(2, 6), (3, 6), out);
		
		object(&mut self.launch, input, &self.menu, menus::Menu::UserSelect,
		(2, 8), (3, 8), out);

		object(&mut self.speedruns, input, &self.menu, menus::Menu::UserSelect,
		(2, 10), (3, 10), out);

		object(&mut self.challenges, input, &self.menu, menus::Menu::UserSelect,
		(2, 12), (3, 12), out);

		object(&mut self.tier, input, &self.menu, menus::Menu::Out,
		(15, 3), (15, 3), out);

		traits::UserInterface::draw(&self.maps, out);
		traits::UserInterface::draw(&self.mods, out);
		traits::UserInterface::draw(&self.wiki, out);
		traits::UserInterface::draw(&self.launch, out);
		traits::UserInterface::draw(&self.speedruns, out);
		traits::UserInterface::draw(&self.challenges, out);
		traits::UserInterface::draw(&self.tier, out);
	}

	pub fn tick(&mut self, input : &input::InputHandler, out : &mut Stdout) {

		object(&mut self.maps, input, &self.menu, menus::Menu::UserSelect,
		(2, 2), (3, 2), out);

		object(&mut self.mods, input, &self.menu, menus::Menu::UserSelect,
		(2, 4), (3, 4), out);
		
		object(&mut self.wiki, input, &self.menu, menus::Menu::UserSelect,
		(2, 6), (3, 6), out);
		
		object(&mut self.launch, input, &self.menu, menus::Menu::UserSelect,
		(2, 8), (3, 8), out);

		object(&mut self.speedruns, input, &self.menu, menus::Menu::UserSelect,
		(2, 10), (3, 10), out);

		object(&mut self.challenges, input, &self.menu, menus::Menu::UserSelect,
		(2, 12), (3, 12), out);

		object(&mut self.tier, input, &self.menu, menus::Menu::Out,
		(15, 3), (15, 3), out);
	}
}

fn object(
	object : &mut impl traits::UserInterface, input : &input::InputHandler,
	menu_reference : &Rc<RefCell<menus::Menu>>, menu_clicked : menus::Menu,
	norm : (u16, u16), hover : (u16, u16), 
	out : &mut Stdout
) {
	let prev_state : bool = object.is_hovered();
	object.set_position(norm.0, norm.1);
	traits::UserInterface::update(object, input);
	let redraw_requested : bool =
		object.is_hovered() != prev_state;

	if redraw_requested {

		if prev_state {
			object.set_position(hover.0, hover.1);
		}
		else {
			object.set_position(norm.0, norm.1);
		}
		
		traits::UserInterface::clear(object, out);
	}
	
	if object.is_hovered() {
		object.set_position(hover.0, hover.1);
		object.set_color(true);
		if input.mouse.lclick {
			*menu_reference.borrow_mut() = menu_clicked;
		}
	}
	else {
		object.set_color(false);
		object.set_position(norm.0, norm.1);
	}
	
	if redraw_requested {
		traits::UserInterface::draw(object, out);
	}
}
