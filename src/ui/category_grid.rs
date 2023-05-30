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

        let cloned_data_map = data_map.clone();
        let cloned_category = category.clone();
        let scroll_window_clone = scroll_window.clone();

        let flowbox_clone_for_back_button = flowbox.clone();


        // Connect a callback function to handle button click event
        button.connect_clicked(move |_| {
            // Here we will create a new FlowBox for the items
            let item_flowbox = FlowBox::new();
            item_flowbox.set_homogeneous(true);
            item_flowbox.set_valign(gtk::Align::Start);
            item_flowbox.set_selection_mode(gtk::SelectionMode::None);
            item_flowbox.set_max_children_per_line(3);

            println!("Cloned Category: {}", cloned_category);

            if let (Some(categories), Some(items)) =
                (cloned_data_map.get("Category"), cloned_data_map.get("Item"))
            {
                let mut item_css_class_iter = css_classes.iter().cycle();

                // Filter items that belong to the cloned_category
                let category_items: Vec<_> = items
                    .iter()
                    .enumerate()
                    .filter_map(|(index, item)| {
                        if categories
                            .get(index)
                            .map_or(false, |c| c == &cloned_category)
                        {
                            Some(item)
                        } else {
                            None
                        }
                    })
                    .collect();

                println!("Items: {:?}", category_items);

                for item in category_items {
                    let item_button = Button::with_label(item);

                    // Assign CSS class to item button
                    let item_css_class = item_css_class_iter.next().unwrap();
                    item_button.add_css_class(item_css_class);
                    item_button.add_css_class("btn");

                    item_flowbox.append(&item_button);
                }

                // Add a back button
                let back_button = Button::with_label("Back");

                // Assign CSS class to back button
                back_button.add_css_class("btn");
                item_flowbox.append(&back_button);

                // Handle back button click
                let flowbox_clone = flowbox_clone_for_back_button.clone();
                let scroll_window_clone_back = scroll_window_clone.clone();
                back_button.connect_clicked(move |_| {
                    scroll_window_clone_back.set_child(Some(&flowbox_clone));
                });
            }

            // Now replace the category flowbox with the item flowbox in the scroll window
            scroll_window_clone.set_child(Some(&item_flowbox));
        });

        flowbox.append(&button);
    }

    scroll_window.set_child(Some(&flowbox));

    scroll_window
}
