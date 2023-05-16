use gtk::{prelude::*, Button, FlowBox, ScrolledWindow};

pub fn build_category_grid() -> ScrolledWindow {
    let scroll_window = ScrolledWindow::new();
    scroll_window.set_size_request(600, -1);

    let flowbox = FlowBox::new();
    flowbox.set_homogeneous(true);
    flowbox.set_valign(gtk::Align::Start);
    flowbox.set_selection_mode(gtk::SelectionMode::None);
    flowbox.set_max_children_per_line(3);

    // Placeholder buttons
    for i in 1..=10 {
        let button_label = format!("Category {}", i);
        let button = Button::with_label(&button_label);
        flowbox.append(&button);
    }
    scroll_window.set_child(Some(&flowbox));

    scroll_window
}
