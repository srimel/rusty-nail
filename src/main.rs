use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk::{Box, Button, FlowBox, FlowBoxChild, Label, Orientation, ScrolledWindow};
use gtk::{ListBox, ListBoxRow};

fn build_ui(application: &Application) {
    let window = ApplicationWindow::new(application);
    window.set_default_size(800, 600);
    window.set_title(Some("Rusty Nail POS"));

    // Create the left pane with a scrollable FlowBox
    let main_box = Box::new(Orientation::Horizontal, 0);
    window.set_child(Some(&main_box));

    let left_pane = ScrolledWindow::new();
    left_pane.set_size_request(600, -1);

    let flowbox = FlowBox::new();
    flowbox.set_homogeneous(true);
    flowbox.set_valign(gtk::Align::Start);
    flowbox.set_selection_mode(gtk::SelectionMode::None);

    // Add placeholder buttons to the left pane flowbox
    for i in 1..=10 {
        let button_label = format!("Category {}", i);
        let button = Button::with_label(&button_label);
        flowbox.append(&button);
    }

    flowbox.set_max_children_per_line(3);

    left_pane.set_child(Some(&flowbox));

    let right_pane = ListBox::new();
    for i in 1..=5 {
        let item_label = format!("Item {}", i);
        let row = ListBoxRow::new();
        let label = Label::new(Some(&item_label));
        row.set_child(Some(&label));
        right_pane.append(&row);
    }
    right_pane.set_size_request(200, -1);

    main_box.append(&left_pane);
    main_box.append(&right_pane);

    window.present();
}

fn main() {
    let application = Application::builder()
        .application_id("com.RustGroup.RustyNail")
        .build();

    application.connect_startup(|app| {
        build_ui(app);
    });

    application.run();
}
