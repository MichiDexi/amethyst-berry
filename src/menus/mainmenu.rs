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
	labels : Vec<label::Label>,
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
			labels : vec!(maps_label,
			mods_label,
			wiki_label,
			launch_label,
			speedruns_label,
			mapcreation_label,
			achievements_label,
			challenges_label),
			tier : tier_label,
			selection : selector,
		};

		obj.init_draw(out);

		obj
	}

	fn init_draw(&self, out : &mut Stdout) {
		write!(out, "\x1b[2J").unwrap();
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


		self.maps.update(input);
		if self.maps.hovered {
			self.maps.clear(out);
			self.maps.x = 3;
			self.maps.draw(out);
		}
		else {
			self.maps.clear(out);
			self.maps.x = 2;
			self.maps.draw(out);
		}

		self.mods.update(input);
		if self.mods.hovered {
			self.mods.clear(out);
			self.mods.x = 3;
			self.mods.draw(out);
		}
		else {
			self.mods.clear(out);
			self.mods.x = 2;
			self.mods.draw(out);
		}

		self.wiki.update(input);
		if self.wiki.hovered {
			self.wiki.clear(out);
			self.wiki.x = 3;
			self.wiki.draw(out);
		}
		else {
			self.wiki.clear(out);
			self.wiki.x = 2;
			self.wiki.draw(out);
		}

		self.launch.update(input);
		if self.launch.hovered {
			self.launch.clear(out);
			self.launch.x = 3;
			self.launch.draw(out);
		}
		else {
			self.launch.clear(out);
			self.launch.x = 2;
			self.launch.draw(out);
		}

		self.speedruns.update(input);
		if self.speedruns.hovered {
			self.speedruns.clear(out);
			self.speedruns.x = 3;
			self.speedruns.draw(out);
		}
		else {
			self.speedruns.clear(out);
			self.speedruns.x = 2;
			self.speedruns.draw(out);
		}

		self.mapcreation.update(input);
		if self.mapcreation.hovered {
			self.mapcreation.clear(out);
			self.mapcreation.x = 3;
			self.mapcreation.draw(out);
		}
		else {
			self.mapcreation.clear(out);
			self.mapcreation.x = 2;
			self.mapcreation.draw(out);
		}

		self.achievements.update(input);
		if self.achievements.hovered {
			self.achievements.clear(out);
			self.achievements.x = 3;
			self.achievements.draw(out);
		}
		else {
			self.achievements.clear(out);
			self.achievements.x = 2;
			self.achievements.draw(out);
		}

		self.challenges.update(input);
		if self.challenges.hovered {
			self.challenges.clear(out);
			self.challenges.x = 3;
			self.challenges.draw(out);
		}
		else {
			self.challenges.clear(out);
			self.challenges.x = 2;
			self.challenges.draw(out);
		}

		self.tier.update(input);
		if self.tier.hovered {
			self.tier.clear(out);
			self.tier.y = 3;
			self.tier.draw(out);
		}
		else {
			self.tier.clear(out);
			self.tier.y = 2;
			self.tier.draw(out);
		}
		
		for label in self.labels.iter_mut() {
		    elem.update(input);
		    if elem.hovered() {
		        elem.draw_hover(out);
		    } else {
		        elem.draw_normal(out);
		    }
		}
	}
}
