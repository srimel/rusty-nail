mod data_processing;
mod patron;
mod patrons;
mod tests;
mod ui;

use std::collections::HashMap;
use std::env;

use data_processing::read_csv_file;
use gtk::gdk::Display;
use gtk::{
    prelude::*, style_context_add_provider_for_display, Application, ApplicationWindow, Box,
    CssProvider, Orientation,
};

/// Builds the UI for the application.
/// Calls `build_header_bar()` to build the header bar.
/// Calls `build_category_grid()` to build the category grid.
/// Calls `build_transaction_panel()` to build the transaction panel.
fn build_ui(application: &Application, data_map: HashMap<String, Vec<String>>) {
    let window = ApplicationWindow::new(application);
    window.set_cursor_from_name(Some("default"));
    window.set_default_size(800, 600);
    window.set_title(Some("Rusty Nail POS"));
    window.add_css_class("main-window");

    let content_box = Box::new(Orientation::Vertical, 0);
    content_box.append(&ui::build_header_bar(&window));

    let main_box = Box::new(Orientation::Horizontal, 0);
    main_box.set_size_request(-1, 500);
    main_box.append(&ui::build_category_grid(data_map));
    main_box.append(&ui::build_transaction_panel(&window));

    content_box.append(&main_box);

    window.set_child(Some(&content_box));
    window.present();
}

/// Creates a `CssProvider` and loads the CSS file.
fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("style.css"));
    style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

/// Main function for the application.
/// Calls `build_ui()` to build the UI.
/// Calls `read_csv_file()` to read the CSV file.
/// Calls `load_css()` to load the CSS file via a `CssProvider`.
fn main() {
    let application = Application::builder()
        .application_id("com.RustGroup.RustyNail")
        .build();

    application.connect_startup(|_| load_css());

    application.connect_activate(|app| {
        let current_dir = env::current_dir().expect("Failed to get current directory");
        println!("Current working directory: {:?}", current_dir);
        match read_csv_file("./menus/menu1.csv") {
            Ok(data_map) => {
                println!("{:?}", data_map);
                build_ui(app, data_map);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    });

    application.run();
}
