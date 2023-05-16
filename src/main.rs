use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk::{Box, Button, FlowBox, FlowBoxChild, Label, Orientation, ScrolledWindow};
use gtk::{ListBox, ListBoxRow, HeaderBar};

fn build_ui(application: &Application) {
    let window = ApplicationWindow::new(application);
    window.set_default_size(800, 600);
    window.set_title(Some("Rusty Nail POS"));

    let main_box = Box::new(Orientation::Horizontal, 0);
    main_box.set_size_request(-1, 500);

    // Header Bar
    let header_bar = HeaderBar::new();
    // header_bar.set_show_close_button(false);

    let title_label = Label::new(Some("Rusty Nail POS"));
    header_bar.set_title_widget(Some(&title_label));

    let header_button1 = Button::with_label("Button 1");
    let header_button2 = Button::with_label("Button 2");
    header_bar.pack_start(&header_button1);
    header_bar.pack_start(&header_button2);

    // Create a box to hold the header bar and main content
    let content_box = Box::new(Orientation::Vertical, 0);
    content_box.append(&header_bar);

    // Left Pane Button Grid
    let left_pane = ScrolledWindow::new();
    left_pane.set_size_request(600, -1);

    let flowbox = FlowBox::new();
    flowbox.set_homogeneous(true);
    flowbox.set_valign(gtk::Align::Start);
    flowbox.set_selection_mode(gtk::SelectionMode::None);
    flowbox.set_max_children_per_line(3);

    // Add placeholder buttons to the left pane flowbox
    for i in 1..=10 {
        let button_label = format!("Category {}", i);
        let button = Button::with_label(&button_label);
        flowbox.append(&button);
    }

    left_pane.set_child(Some(&flowbox));

    let right_pane = ListBox::new();
    right_pane.set_selection_mode(gtk::SelectionMode::None);

    // Add placeholder items to the right pane listbox
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

    content_box.append(&main_box);

    window.set_child(Some(&content_box));

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
