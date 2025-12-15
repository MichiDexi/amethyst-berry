use std::{
	time::{
		Duration,
		Instant,
	},
	thread::{
		sleep
	},
	io::{
		stdout,
		Write,
		Stdout,
	},
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
	mapcreation : label::Label,
	achievements : label::Label,
	challenges : label::Label,
	tier : textbox::Box,
	selection : Selector
}

enum Selector {
	Maps,
	Mods,
	Wiki,
	Launch,
	Speedruns,
	MapCreation,
	Achievements,
	Challenges,
	Tier,
}

impl MainMenu {
	pub fn init(out : &mut Stdout) -> Self {
		let maps_label : label::Label = label::Label::new(2, 3, 12, "Maps");
		let mods_label : label::Label = label::Label::new(2, 5, 12, "Mods");
		let wiki_label : label::Label = label::Label::new(2, 7, 12, "Wiki");
		let launch_label : label::Label = label::Label::new(2, 9, 12, "Launch");
		let speedruns_label : label::Label = label::Label::new(2, 11, 12, "Speedruns");
		let mapcreation_label : label::Label = label::Label::new(2, 13, 12, "Create map");
		let achievements_label : label::Label = label::Label::new(2, 15, 12, "Achievement");
		let challenges_label : label::Label = label::Label::new(2, 17, 12, "Challenges");
		let tier_label : textbox::Box = textbox::Box::new(15, 3, 7, 7);
		let selector : Selector = Selector::Maps;
		
		let obj = Self {
			maps : maps_label,
			mods : mods_label,
			wiki : wiki_label,
			launch : launch_label,
			speedruns : speedruns_label,
			mapcreation : mapcreation_label,
			achievements : achievements_label,
			challenges : challenges_label,
			tier : tier_label,
			selection : selector,
		};

		obj.init_draw(out);

		obj
	}

	fn init_draw(&self, out : &mut Stdout) {
		write!(out, "\x1b[2J").unwrap();
		traits::UserInterface::draw(&self.maps, out);
		traits::UserInterface::draw(&self.mods, out);
		traits::UserInterface::draw(&self.wiki, out);
		traits::UserInterface::draw(&self.launch, out);
		traits::UserInterface::draw(&self.speedruns, out);
		traits::UserInterface::draw(&self.mapcreation, out);
		traits::UserInterface::draw(&self.achievements, out);
		traits::UserInterface::draw(&self.challenges, out);
		traits::UserInterface::draw(&self.tier, out);
	}

	pub fn tick(&mut self, input : &input::InputHandler, out : &mut Stdout, menu : &mut menus::Menu) {

		// Label vector
		// Needed to make "Label loop" work
		let mut labels : Vec<&mut label::Label> =
		vec!(
			&mut self.maps, &mut self.mods,
			&mut self.wiki, &mut self.launch,
			&mut self.speedruns,
			&mut self.mapcreation,
			&mut self.achievements,
			&mut self.challenges
		);

		// Label loop
		for label in labels.iter_mut() {
			let prev_state : bool = label.hovered; // Before updating to current input
			traits::UserInterface::update(*label, input);
			let redraw_requested : bool = label.hovered != prev_state; // If input has changed, you should redraw

			if redraw_requested {
				traits::UserInterface::clear(*label, out); // Clear previous label
			}
			if label.hovered {
				label.x = 3; // Move the label if hovered
			}
			else {
				label.x = 2; // Put the label back if not
			}
			if redraw_requested {
				traits::UserInterface::draw(*label, out); // Add new label
			}
		}

		let prev_state : bool = self.tier.hovered; // Before updating to current input
		traits::UserInterface::update(&mut self.tier, input);
		let redraw_requested : bool = self.tier.hovered != prev_state; // If input has changed, you should redraw

		if redraw_requested {
			traits::UserInterface::clear(&self.tier, out);
		}
		self.tier.color.color_enabled = self.tier.hovered;
		if redraw_requested {
			traits::UserInterface::draw(&self.tier, out);
		}

		if self.tier.hovered && input.mouse.lclick {
			*menu = menus::Menu::Out;
		}
	}
}
