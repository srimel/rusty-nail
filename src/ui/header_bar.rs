use gtk::{prelude::*, Button, HeaderBar, Label};

/// Builds the header bar at the top of the application.
pub fn build_header_bar() -> HeaderBar {
    let header_bar = HeaderBar::new();
    header_bar.add_css_class("header-bar");

    let title_label = Label::new(Some("Rusty Nail POS"));
    header_bar.set_title_widget(Some(&title_label));

    // define header buttons
    // let header_button1 = Button::with_label("Load Menu");
    // header_button1.add_css_class("btn");
    // header_button1.set_tooltip_text(Some("Load a menu from a CSV file"));
    // header_button1.connect_clicked(|_| {
    //     println!("Load menu button clicked");
    // });
    let header_button2 = Button::with_label("Select Patron");
    header_button2.add_css_class("btn");
    header_button2.connect_clicked(|_| {
        println!("Select patron button clicked");
    });
    let header_button3 = Button::with_label("New Tab");
    header_button3.add_css_class("btn");
    header_button3.connect_clicked(|_| {
        println!("New tab clicked");
    });

    // add buttons to header bar
    // header_bar.pack_start(&header_button1);
    header_bar.pack_start(&header_button2);
    header_bar.pack_start(&header_button3);

    header_bar
}
