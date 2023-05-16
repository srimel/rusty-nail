use gtk::{prelude::*, Button, HeaderBar, Label};

/// Builds the header bar at the top of the application.
pub fn build_header_bar() -> HeaderBar {
    let header_bar = HeaderBar::new();
    header_bar.add_css_class("header-bar");

    let title_label = Label::new(Some("Rusty Nail POS"));
    header_bar.set_title_widget(Some(&title_label));

    let header_button1 = Button::with_label("Button 1");
    header_button1.add_css_class("btn");
    let header_button2 = Button::with_label("Button 2");
    header_button2.add_css_class("btn");    
    header_bar.pack_start(&header_button1);
    header_bar.pack_start(&header_button2);

    header_bar
}
