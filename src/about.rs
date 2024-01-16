use gtk::gio::{ActionEntry, ActionMap};

pub fn about_actions() -> ActionEntry<ActionMap> {
	let action_about = ActionEntry::builder("about")
		.build();
	
	return action_about;
}