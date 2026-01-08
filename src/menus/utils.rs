use std::{
	io::Stdout,
	rc::Rc,
	cell::RefCell,
};
use crate::{
	interface::traits,
	helpers::input,
	abt::menus,
}; 


fn object(
	object : &mut impl traits::UserInterface, input : &input::InputHandler,
	menu_reference : &Rc<RefCell<menus::Menu>>, menu_clicked : menus::Menu,
	norm : (i16, i16), hover : (i16, i16), anchor : u8,
	out : &mut Stdout
) {
	let prev_state : bool = object.is_hovered();
	object.set_position(
		norm.0, norm.1,
		anchor, (input.window.width, input.window.height)
	);
	traits::UserInterface::update(object, input);
	let redraw_requested : bool =
		object.is_hovered() != prev_state;

	if redraw_requested {
		if prev_state {
			object.set_position(
				hover.0, hover.1,
				anchor, (input.window.width, input.window.height)
			);
		}
		else {
			object.set_position(
				norm.0, norm.1,
				anchor, (input.window.width, input.window.height)
			);
		}
		
		traits::UserInterface::clear(object, out);
	}
	
	if object.is_hovered() {
		object.set_position(
			hover.0, hover.1,
			anchor, (input.window.width, input.window.height)
		);
		object.set_color(true);
		if input.mouse.lclick {
			*menu_reference.borrow_mut() = menu_clicked;
		}
	}
	else {
		object.set_color(false);
		object.set_position(
			norm.0, norm.1,
			anchor, (input.window.width, input.window.height)
		);
	}
	
	if redraw_requested {
		traits::UserInterface::draw(object, out);
	}
}
