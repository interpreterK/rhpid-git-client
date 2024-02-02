use libadwaita::{glib, prelude::*, AboutWindow, Application, ApplicationWindow, HeaderBar, OverlaySplitView, ToolbarView};
use gtk::{Box as GtkBox, Orientation, ToggleButton};

const VERSION: 	&str = env!("CARGO_PKG_VERSION");
const APP_ID:   &str = "com.unittensor.gitwaita";
const APP_NAME: &str = "GitWaita";
const DEFAULT_WIDTH:  i32 = 1200;
const DEFAULT_HEIGHT: i32 = 700;

// TODO:
// libadwaita has no support yet for Windows, if Windows is detected switch over to pure GTK 4
// enum Platform {
// 	Linux,
// 	Windows
// }

fn window_header() -> (ToggleButton, HeaderBar, GtkBox) {
	let sidebar_toggle = ToggleButton::builder()
		.active(true)
		.tooltip_text("Toggle Locations Sidebar")
		.icon_name("view-sidebar-start-symbolic")
		.build();

	let header = HeaderBar::new();
	let headerbox = GtkBox::new(Orientation::Vertical, 0);
	header.pack_start(&sidebar_toggle); //Place toggling the sidebar button on the left side of the window frame
	headerbox.append(&header);

	return (sidebar_toggle, header, headerbox);
}

fn locations_sidebar(headerbox: GtkBox) -> (OverlaySplitView, ToolbarView) {
	let splitview = OverlaySplitView::builder()
		.content(&headerbox)
		.build();

	let toolbar_view = ToolbarView::builder()
		.content(&splitview)
		.build();

	return (splitview, toolbar_view);
}

fn about() {
	let about_window = AboutWindow::builder()
		.transient_for(transient_for)
		.build();
}

fn main_window(application: &Application) {
	application.connect_activate(|app| {
		let (_sidebar_toggle, _header, headerbox) = window_header();
		let (_splitview, toolbar_view) = locations_sidebar(headerbox);

		let window = ApplicationWindow::builder()
			.application(app)
			.title(APP_NAME)
			.default_width(DEFAULT_WIDTH)
			.default_height(DEFAULT_HEIGHT)
			.content(&toolbar_view)
			.build();
		window.present();
	});
}

fn main() -> glib::ExitCode {
	let application = Application::builder()
		.application_id(APP_ID)
		.build();
	main_window(&application);

	println!("Using version {}", VERSION);
	application.run()
}