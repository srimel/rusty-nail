use crate::patron::Patron;
use crate::patrons::PATRONS;
use crate::ui::transaction_panel::{get_current_patron_label, update_item_list};

#[allow(deprecated)]
use gtk::{
    prelude::*, ApplicationWindow, Box, Button, Dialog, Entry, HeaderBar, Label, ListBox,
    Orientation, ResponseType,
};

/// Builds the header bar at the top of the application.
/// Connects the "Select Patron" button to `create_select_patron()`.
/// Connects the "New Tab" button to `create_new_tab()`.
pub fn build_header_bar(window: &ApplicationWindow) -> HeaderBar {
    let header_bar = HeaderBar::new();
    header_bar.add_css_class("header-bar");

    let title_label = Label::new(Some("Rusty Nail POS"));
    header_bar.set_title_widget(Some(&title_label));

    let select_patron_button = Button::with_label("Select Patron");
    select_patron_button.add_css_class("btn");
    let another_cloned_window = window.clone();
    select_patron_button.connect_clicked(move |_| {
        println!("Select patron button clicked");
        create_select_patron(&another_cloned_window);
    });
    let new_tab_button = Button::with_label("New Tab");
    new_tab_button.add_css_class("btn");
    let cloned_window = window.clone();
    new_tab_button.connect_clicked(move |_| {
        println!("New tab clicked");
        create_new_tab(&cloned_window);
    });

    // add buttons to header bar
    header_bar.pack_start(&select_patron_button);
    header_bar.pack_start(&new_tab_button);

    header_bar
}

/// Creates a dialog box that allows the user to create a new patron.
/// The new patron is then set as the current patron.
/// Calls `update_item_list()` to update the item list for the new patron, which in this case will be empty.
#[allow(deprecated)]
fn create_new_tab(window: &ApplicationWindow) {
    // Create a new dialog box
    let dialog = Dialog::new();
    dialog.add_css_class("dialog");
    dialog.set_title(Some("New Tab"));
    dialog.set_modal(true);
    dialog.set_transient_for(Some(window));
    dialog
        .add_button("Cancel", ResponseType::Cancel)
        .add_css_class("btn");
    dialog
        .add_button("Create", ResponseType::Accept)
        .add_css_class("btn");

    // Create a vertical box to hold the dialog's contents
    let content_area = dialog.content_area();
    content_area.add_css_class("dialog-content-area");
    let vbox = Box::new(Orientation::Vertical, 10);
    content_area.append(&vbox);

    // Add a label and an entry for the patron name
    let label = Label::new(Some("Enter the patron name:"));
    let entry = Entry::new();
    vbox.append(&label);
    vbox.append(&entry);

    // Connect the dialog's response signal
    dialog.connect_response(move |dialog, response| {
        if response == ResponseType::Accept {
            let patron_name = entry.text().to_string();
            if patron_name.is_empty() {
                return;
            }
            println!("Patron name: {}", patron_name);
            let mut patrons = PATRONS.lock().unwrap();
            patrons.push(Patron {
                name: patron_name.to_string(),
                tab: Vec::new(),
            });
            drop(patrons);
            if let Some(label) = get_current_patron_label() {
                label.set_text(&patron_name);
                update_item_list();
            }
        }
        dialog.close();
    });

    // Show the dialog
    dialog.present();
}

/// Creates a dialog box that allows the user to select a patron.
/// The selected patron is then set as the current patron.
/// Calls `update_item_list()` to update the item list for the selected patron.
#[allow(deprecated)]
fn create_select_patron(window: &ApplicationWindow) {
    // Create a new dialog box
    let dialog = Dialog::new();
    dialog.add_css_class("dialog");
    dialog.set_title(Some("Select A Patron"));
    dialog.set_modal(true);
    dialog.set_transient_for(Some(window));
    dialog
        .add_button("Cancel", ResponseType::Cancel)
        .add_css_class("btn");
    dialog
        .add_button("Select", ResponseType::Accept)
        .add_css_class("btn");

    let content_area = dialog.content_area();
    content_area.add_css_class("dialog-content-area");
    let patron_list_box = ListBox::new();
    patron_list_box.set_selection_mode(gtk::SelectionMode::Single);
    let patrons = PATRONS.lock().unwrap();
    for patron in patrons.iter() {
        let patron_label = Label::new(Some(&patron.name));
        patron_list_box.append(&patron_label);
    }
    content_area.append(&patron_list_box);

    // Connect the dialog's response signal
    dialog.connect_response(move |dialog, response| {
        if response == ResponseType::Accept {
            let selected_patron = patron_list_box.selected_row();
            if let Some(selected_patron) = selected_patron {
                let selected_patron_label = selected_patron
                    .child()
                    .unwrap()
                    .downcast::<Label>()
                    .unwrap();
                let selected_patron_name = selected_patron_label.text().to_string();
                let current_patron_label = get_current_patron_label().as_ref().unwrap();
                current_patron_label.set_text(&selected_patron_name);
                update_item_list();
            }
        }
        dialog.close();
    });

    // Show the dialog
    dialog.present();
}
