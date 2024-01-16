use libadwaita::OverlaySplitView;
use gtk::ToggleButton;

pub fn sidebar_actions() -> (OverlaySplitView, ToggleButton) {
	let splitview = OverlaySplitView::builder()
		.build();
	let sidebar_toggle = ToggleButton::builder()

		.build();

	return (splitview, sidebar_toggle);
}
