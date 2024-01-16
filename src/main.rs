use libadwaita::{prelude::*, HeaderBar, glib, Application, ApplicationWindow};
use gtk::{Box, Orientation};

use crate::sidebar::sidebar_actions;
use crate::about::about_actions;

const DEFAULT_WINDOW_WIDTH: i32 = 1200;
const DEFAULT_WINDOW_HEIGHT: i32 = 700;
const VERSION: &str = env!("CARGO_PKG_VERSION");

mod sidebar;
mod about;

fn main_window(application: &Application) {
	application.connect_activate(|app| {
		//modules
		let (splitview, sidebar_toggle) = sidebar_actions();
		let action_about = about_actions();

        let header = HeaderBar::new();
        header.pack_start(&sidebar_toggle);
        
        let headerbox = Box::new(Orientation::Vertical, 0);
        headerbox.append(&header);

		let window = ApplicationWindow::builder()
			.application(app)
			.title(format!("rhpid Git Client {}", VERSION))
			.default_width(DEFAULT_WINDOW_WIDTH)
			.default_height(DEFAULT_WINDOW_HEIGHT)
			.content(&headerbox)
			.build();
		window.present();
	});
}

fn main() -> glib::ExitCode {
	let application = Application::builder()
		.application_id("com.rhpidfyre.gtk4gitclient")
		.build();

	main_window(&application);
	application.run()
}