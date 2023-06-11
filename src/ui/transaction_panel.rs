use crate::patrons::PATRONS;
#[allow(deprecated)]
use gtk::{
    prelude::*, ApplicationWindow, Box, Button, Dialog, Label, ListBox, ListBoxRow, Orientation,
    ResponseType,
};

// These global mutable references are used to access and update widgets dynamically
// based on user interaction with the app.
static mut CURRENT_PATRON_LABEL: Option<Label> = None;
static mut ITEM_LIST: Option<ListBox> = None;
static mut AMOUNT_OWED_LABEL: Option<Label> = None;

/// Builds the right panel of the application. This includes the list of items and the total
/// amount owed with a checkout button.
pub fn build_transaction_panel(window: &ApplicationWindow) -> Box {
    let transaction_box = Box::new(Orientation::Vertical, 0);
    transaction_box.append(&build_tab_owner_box());
    transaction_box.append(&build_item_list());
    transaction_box.append(&build_remove_item_box());
    transaction_box.append(&build_amount_owed_box(window));
    transaction_box.add_css_class("transaction-box");

    transaction_box
}

/// Builds the list of items that the patron has ordered.
/// Assignes the `ListBox` to a global mutable reference so that it can be accessed from other
/// modules.
fn build_item_list() -> ListBox {
    let item_list: ListBox = ListBox::new();
    item_list.set_selection_mode(gtk::SelectionMode::Single);
    item_list.set_size_request(200, 300);
    item_list.add_css_class("items-list");

    unsafe {
        ITEM_LIST = Some(item_list.clone());
    }

    item_list
}

/// Builds the box that contains the remove item button.
fn build_remove_item_box() -> Box {
    let remove_item_box = Box::new(Orientation::Vertical, 0);
    let remove_item_button = Button::with_label("Remove Item");
    remove_item_button.add_css_class("btn");
    remove_item_button.add_css_class("remove-item-btn");
    remove_item_button.connect_clicked(|_| {
        println!("Remove item button clicked");
        let item_list_box = get_item_list().as_ref().unwrap();
        if let Some(selected_row) = item_list_box.selected_row() {
            let selected_row_label = selected_row.child().unwrap().downcast::<Label>().unwrap();
            let selected_row_string = selected_row_label.text().to_string();
            let split_string: Vec<&str> = selected_row_string.split(": $").collect();
            let item_name_to_remove = split_string[0];
            let current_patron_name = get_current_patron_label_text();
            let mut patrons = PATRONS.lock().unwrap();
            let curr_patron = patrons.iter_mut().find(|p| p.name == current_patron_name);
            match curr_patron {
                Some(p) => {
                    println!("Found Patron: {:?}", p);
                    let item_index = p
                        .tab
                        .iter()
                        .position(|(item, _)| item == item_name_to_remove);
                    match item_index {
                        Some(i) => {
                            p.tab.remove(i);
                        }
                        None => {
                            println!("Item not found in patron's tab");
                        }
                    }
                }
                None => {
                    println!("Could not find patron");
                }
            }
            println!("Item to remove: {}", item_name_to_remove);
            drop(patrons);
            update_item_list();
        } else {
            println!("Could not find selected row");
        }
    });
    remove_item_box.append(&remove_item_button);
    remove_item_box.add_css_class("remove-item-box");

    remove_item_box
}

/// Builds the box that displays the total amount owed and the checkout button.
/// Assignes the `Label` to a global mutable reference so that it can be accessed from other
/// modules.
fn build_amount_owed_box(window: &ApplicationWindow) -> Box {
    let total_amount_box = Box::new(Orientation::Vertical, 0);
    let total_amount_label = Label::new(Some("Total Amount: $0.00"));

    unsafe {
        AMOUNT_OWED_LABEL = Some(total_amount_label.clone());
    }

    total_amount_box.append(&total_amount_label);
    total_amount_box.add_css_class("total-amount-box");

    let checkout_button = Button::with_label("Close Out");
    total_amount_box.append(&checkout_button);
    checkout_button.add_css_class("btn");
    checkout_button.add_css_class("checkout-btn");

    let cloned_window = window.clone();
    checkout_button.connect_clicked(move |_| {
        println!("Checkout button clicked");
        // if no patron is selected or total amount is 0, do nothing
        let total_amount_text = get_amount_owed_label().as_ref().unwrap().text();
        let split_string: Vec<&str> = total_amount_text.split(": $").collect();
        let amount = split_string[1].parse::<f64>().unwrap();
        if get_current_patron_label_text().is_empty() || amount == 0.0 {
            println!("No patron selected or total amount is 0");
            return;
        }
        start_checkout_dialog(&cloned_window);
    });

    total_amount_box
}

/// Starts the dialog that prompts the user to enter their credit card information and generates a receipt if they press the checkout button.
#[allow(deprecated)]
fn start_checkout_dialog(window: &ApplicationWindow) {
    let dialog = Dialog::new();
    dialog.add_css_class("dialog");
    dialog.set_title(Some("Checkout"));
    dialog.set_modal(true);
    dialog.set_transient_for(Some(window));
    dialog
        .add_button("Cancel", ResponseType::Cancel)
        .add_css_class("btn");
    dialog
        .add_button("Checkout", ResponseType::Accept)
        .add_css_class("btn");

    let content_area = dialog.content_area();
    content_area.add_css_class("dialog-content-area");
    let vbox = Box::new(Orientation::Vertical, 0);
    content_area.append(&vbox);

    let patron_name_label_text =
        String::from("Checkout Patron: ") + &get_current_patron_label_text();
    let patron_name_label = Label::new(Some(&patron_name_label_text));
    patron_name_label.add_css_class("patron-name-label");
    vbox.append(&patron_name_label);

    let amount_owed_label = get_amount_owed_label().as_ref().unwrap();
    let amount_owed_label_text = amount_owed_label.text().to_string();
    let checkout_amount_label = Label::new(Some(&amount_owed_label_text));
    checkout_amount_label.add_css_class("checkout-amount-label");
    vbox.append(&checkout_amount_label);

    let card_number_label = Label::new(Some("Card Number:"));
    card_number_label.add_css_class("card-number-label");
    vbox.append(&card_number_label);
    let card_number_entry = gtk::Entry::new();
    card_number_entry.add_css_class("card-number-entry");
    vbox.append(&card_number_entry);

    let card_expiration_label = Label::new(Some("Card Expiration:"));
    card_expiration_label.add_css_class("card-expiration-label");
    vbox.append(&card_expiration_label);
    let card_expiration_entry = gtk::Entry::new();
    card_expiration_entry.add_css_class("card-expiration-entry");
    vbox.append(&card_expiration_entry);

    let card_cvv_label = Label::new(Some("Card CVV:"));
    card_cvv_label.add_css_class("card-cvv-label");
    vbox.append(&card_cvv_label);
    let card_cvv_entry = gtk::Entry::new();
    card_cvv_entry.add_css_class("card-cvv-entry");
    vbox.append(&card_cvv_entry);

    dialog.connect_response(move |dialog, response| {
        if response == ResponseType::Accept {
            let current_patron_name = get_current_patron_label_text();
            let mut patrons = PATRONS.lock().unwrap();
            let posn = patrons.iter().position(|p| p.name == current_patron_name);
            match posn {
                Some(i) => {
                    patrons.remove(i);
                }
                None => {
                    println!("Could not find patron");
                }
            }
            drop(patrons);

            // Generate a mock reciept as an external text file
            let amount = amount_owed_label_text.split(": $").collect::<Vec<&str>>()[1].to_string();
            generate_receipt(
                current_patron_name,
                amount,
                card_number_entry.text().to_string(),
                card_expiration_entry.text().to_string(),
                card_cvv_entry.text().to_string(),
            );

            if let Some(label) = get_current_patron_label() {
                label.set_text("");
            }
            if let Some(item_list) = get_item_list() {
                item_list.unselect_all();
                while let Some(row) = item_list.last_child() {
                    item_list.remove(&row);
                }
            }
            // Update the AMOUNT_OWED_LABEL with default value
            let amount_owed_label = get_amount_owed_label();
            amount_owed_label
                .as_ref()
                .unwrap()
                .set_text("Total Amount: $0.00");
        }
        dialog.close();
    });
    dialog.present();
}

/// Generates a mock receipt as an external text file.
/// The receipt is stored in the receipts directory.
/// The receipt file name is the number of receipts + 1.
fn generate_receipt(
    patron_name: String,
    amount_owed: String,
    card_number: String,
    card_expiration: String,
    card_cvv: String,
) {
    use std::fs;
    use std::fs::File;
    use std::io::prelude::*;

    fs::create_dir_all("receipts").expect("Could not create receipts directory");

    let receipt_count = fs::read_dir("receipts")
        .expect("Could not read receipts directory")
        .count();

    let receipt_file_name = format!("receipts/receipt{}.txt", receipt_count + 1);
    let mut file = File::create(receipt_file_name).expect("Could not create file");
    let receipt = format!(
        "Name: {}\nAmount Charge: ${}\nCard Number: {}\nCard Expiration: {}\nCard CVV: {}",
        patron_name, amount_owed, card_number, card_expiration, card_cvv
    );
    file.write_all(receipt.as_bytes())
        .expect("Could not write to file");
}

/// Builds the box that displays the current patron's name.
/// Assignes the `Label` to a global mutable reference so that it can be accessed from other
/// modules.
fn build_tab_owner_box() -> Box {
    let tab_owner_box = Box::new(Orientation::Vertical, 0);
    let patron_name_label = Label::new(Some("Patron Name:"));

    let current_patron_label = Label::new(Some(""));
    current_patron_label.set_widget_name("current-patron-label");
    unsafe {
        CURRENT_PATRON_LABEL = Some(current_patron_label.clone());
    }

    tab_owner_box.append(&patron_name_label);
    tab_owner_box.append(&current_patron_label);
    tab_owner_box.add_css_class("tab-owner-box");

    tab_owner_box
}

/// Updates the current patron label with the name of the selected patron.
pub fn get_current_patron_label() -> &'static Option<Label> {
    // Unsafe block to access the global mutable reference
    unsafe { &CURRENT_PATRON_LABEL }
}

/// Returns the text of the current patron label.
pub fn get_current_patron_label_text() -> String {
    let current_patron_label = get_current_patron_label();
    let current_patron_label_text = current_patron_label.as_ref().unwrap().text();
    current_patron_label_text.to_string()
}

/// Updates the list of items in the transaction panel based on the current patron.
/// This function is called when a new patron is selected or when a new tab is created.
pub fn update_item_list() {
    let mut patrons = PATRONS.lock().unwrap();
    let curr_patron = patrons
        .iter_mut()
        .find(|p| p.name == get_current_patron_label_text());
    match curr_patron {
        Some(p) => {
            let item_list = get_item_list();
            // delete all rows in the list
            item_list.as_ref().unwrap().unselect_all();
            let list_box = item_list.as_ref().unwrap();
            while let Some(row) = list_box.last_child() {
                list_box.remove(&row);
            }
            // add all items in the patron's tab
            let mut total = 0.0f64;
            for (item, price) in p.tab.iter() {
                total += price;
                let item_label = format!("{}: ${}", item, price);
                let row = ListBoxRow::new();
                let label = Label::new(Some(&item_label));
                row.set_child(Some(&label));
                item_list.as_ref().unwrap().append(&row);
            }
            // update the AMOUNT_OWED_LABEL with total
            let amount_owed_label = get_amount_owed_label();
            amount_owed_label
                .as_ref()
                .unwrap()
                .set_text(&format!("Total Amount: ${:.2}", total));
        }
        None => {
            println!("update_list_item: Could not find patron");
        }
    }
}

/// Returns the label that displays the total amount owed.
fn get_amount_owed_label() -> &'static Option<Label> {
    // Unsafe block to access the global mutable reference
    unsafe { &AMOUNT_OWED_LABEL }
}

/// Returns the list of items in the transaction panel.
fn get_item_list() -> &'static Option<ListBox> {
    // Unsafe block to access the global mutable reference
    unsafe { &ITEM_LIST }
}
