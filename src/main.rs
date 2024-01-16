use libadwaita::{prelude::*, ToolbarView};
use libadwaita::{glib, OverlaySplitView, Application, ApplicationWindow};
use gtk::{Box, Orientation};

const DEFAULT_WINDOW_WIDTH: i32 = 1200;
const DEFAULT_WINDOW_HEIGHT: i32 = 700;
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn sidebar() {

}

fn main_window(application: &Application) {
	application.connect_activate(|app| {
		let splitview = OverlaySplitView::builder()
			.build();

        let headerbar = Box::new(Orientation::Vertical, 0);
        headerbar.append(&ToolbarView::new());
        headerbar.append(&splitview);

		let window = ApplicationWindow::builder()
			.application(app)
			.title(format!("rhpid Git Client {}", VERSION))
			.default_width(DEFAULT_WINDOW_WIDTH)
			.default_height(DEFAULT_WINDOW_HEIGHT)
			.content(&headerbar)
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