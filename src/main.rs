mod ui;

use gtk::gdk::Display;
use gtk::{
    prelude::*, style_context_add_provider_for_display, Application, ApplicationWindow, Box,
    CssProvider, Orientation,
};

fn build_ui(application: &Application) {
    let window = ApplicationWindow::new(application);
    window.set_default_size(800, 600);
    window.set_title(Some("Rusty Nail POS"));
    window.add_css_class("main-window");

    let content_box = Box::new(Orientation::Vertical, 0);
    content_box.append(&ui::build_header_bar());

    let main_box = Box::new(Orientation::Horizontal, 0);
    main_box.set_size_request(-1, 500);
    main_box.append(&ui::build_category_grid());
    main_box.append(&ui::build_transaction_panel());

    content_box.append(&main_box);

    window.set_child(Some(&content_box));
    window.present();
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("style.css"));
    style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn main() {
    let application = Application::builder()
        .application_id("com.RustGroup.RustyNail")
        .build();

    application.connect_startup(|_| load_css());

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run();
}
