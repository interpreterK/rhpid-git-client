use libadwaita::{prelude::*, HeaderBar, glib, Application, ApplicationWindow};
use gtk::{Box, Orientation};

use crate::sidebar::sidebar as git_sidebar_tree;

const DEFAULT_WINDOW_WIDTH: i32 = 1200;
const DEFAULT_WINDOW_HEIGHT: i32 = 700;
const VERSION: &str = env!("CARGO_PKG_VERSION");

mod sidebar;

fn main_window(application: &Application) {
	application.connect_activate(|app| {
		let sidebar_tree = git_sidebar_tree();

		let headerbar = HeaderBar::builder()
			.build();
		let sidebar_button = Box::builder()
			.tooltip_text("hi")
			.build();
		sidebar_button.append(&headerbar);

        let headerbox = Box::new(Orientation::Vertical, 0);
        headerbox.append(&headerbar);

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