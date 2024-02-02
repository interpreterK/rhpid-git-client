use libadwaita::{prelude::*, glib, Application, ApplicationWindow, HeaderBar, OverlaySplitView, ToolbarView};
use gtk::{gio::{ActionEntry, ActionMap}, Box as GtkBox, Orientation, ToggleButton};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn window_header() -> (ToggleButton, HeaderBar, GtkBox) {
	let sidebar_toggle = ToggleButton::builder()
		.active(true)
		.tooltip_text("Toggle Locations Sidebar")
		.icon_name("view-sidebar-start-symbolic")
		.build();

	let header = HeaderBar::new();
	header.pack_start(&sidebar_toggle);
	let headerbox = GtkBox::new(Orientation::Vertical, 0);
	headerbox.append(&header);

	return (sidebar_toggle, header, headerbox);
}

fn main_window(application: &Application) {
	application.connect_activate(|app| {
		let (sidebar_toggle, _header, headerbox) = window_header();

		let splitview = OverlaySplitView::builder()
			.content(&headerbox)
			.build();

		let action_about: ActionEntry<ActionMap> = ActionEntry::builder("about").build();

		let toolbar_view = ToolbarView::builder()
			.content(&splitview)
			.build();

		let window = ApplicationWindow::builder()
			.application(app)
			.title(format!("rhpid Git Client {}", VERSION))
			.default_width(1200)
			.default_height(700)
			.content(&toolbar_view)
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