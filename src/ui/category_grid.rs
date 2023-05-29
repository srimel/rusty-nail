use std::collections::{HashMap, HashSet};

use gtk::{prelude::*, Button, FlowBox, ScrolledWindow};

pub fn build_category_grid(data_map: HashMap<String, Vec<String>>) -> ScrolledWindow {
    let scroll_window = ScrolledWindow::new();
    scroll_window.set_size_request(600, -1);

    let flowbox = FlowBox::new();
    flowbox.set_homogeneous(true);
    flowbox.set_valign(gtk::Align::Start);
    flowbox.set_selection_mode(gtk::SelectionMode::None);
    flowbox.set_max_children_per_line(3);

    // Create category buttons from unique values in the "Category" column
    let categories = data_map.get("Category").unwrap();
    let unique_categories: HashSet<_> = categories.iter().cloned().collect();

    let css_classes = [
        "btn-red",
        "btn-blue",
        "btn-yellow",
        "btn-green",
        "btn-cyan",
        "btn-pink",
        "btn-magenta",
        "btn-marine",
        "btn-purple",
        "btn-blueviolet",
    ];

    let mut css_class_iter = css_classes.iter().cycle();

    // Create category buttons
    for category in unique_categories {
        let button = Button::with_label(&category);

        // Assign CSS class to button
        let css_class = css_class_iter.next().unwrap();
        button.add_css_class(css_class);
        button.add_css_class("btn");

        // Connect a callback function to handle button click event
        button.connect_clicked(move |_| {
            // Handle category button click event
            println!("Category button clicked: {}", category);
        });

        flowbox.append(&button);
    }

    scroll_window.set_child(Some(&flowbox));

    scroll_window
}
