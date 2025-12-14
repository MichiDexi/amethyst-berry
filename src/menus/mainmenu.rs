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
	},
	helpers::input,
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
		let maps_label : label::Label = label::Label::new(2, 3, 10, "Maps");
		let mods_label : label::Label = label::Label::new(2, 5, 10, "Mods");
		let wiki_label : label::Label = label::Label::new(2, 7, 10, "Wiki");
		let launch_label : label::Label = label::Label::new(2, 9, 10, "Launch");
		let speedruns_label : label::Label = label::Label::new(2, 11, 10, "Speedruns");
		let mapcreation_label : label::Label = label::Label::new(2, 13, 10, "Create map");
		let achievements_label : label::Label = label::Label::new(2, 15, 10, "Achievement");
		let challenges_label : label::Label = label::Label::new(2, 17, 10, "Challenges");
		let tier_label : textbox::Box = textbox::Box::new(15, 3, 7, 7, 1);
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

	pub fn init_draw(&self, out : &mut Stdout) {
		self.maps.draw(out);
		self.mods.draw(out);
		self.wiki.draw(out);
		self.launch.draw(out);
		self.speedruns.draw(out);
		self.mapcreation.draw(out);
		self.achievements.draw(out);
		self.challenges.draw(out);
		self.tier.draw(out);
	}

	pub fn tick(&mut self, input : &input::InputHandler, out : &mut Stdout) {
		
	}
}
