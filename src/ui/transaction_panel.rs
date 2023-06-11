use crate::current_patron::get_current_patron;
use crate::patrons::PATRONS;
use gtk::{prelude::*, Box, Button, Label, ListBox, ListBoxRow, Orientation};

// These global mutable references are used to access and update widgets dynamically
// based on user interaction with the app.
static mut CURRENT_PATRON_LABEL: Option<Label> = None;
static mut ITEM_LIST: Option<ListBox> = None;
static mut AMOUNT_OWED_LABEL: Option<Label> = None;

/// Builds the right panel of the application. This includes the list of items and the total
/// amount owed with a checkout button.
pub fn build_transaction_panel() -> Box {
    let transaction_box = Box::new(Orientation::Vertical, 0);
    transaction_box.append(&build_tab_owner_box());
    transaction_box.append(&build_item_list());
    transaction_box.append(&build_amount_owed_box());
    transaction_box.add_css_class("transaction-box");

    transaction_box
}

/// Builds the list of items that the patron has ordered.
/// Assignes the `ListBox` to a global mutable reference so that it can be accessed from other
/// modules.
fn build_item_list() -> ListBox {
    let item_list: ListBox = ListBox::new();
    item_list.set_selection_mode(gtk::SelectionMode::None);
    item_list.set_size_request(200, 300);
    item_list.add_css_class("items-list");

    unsafe {
        ITEM_LIST = Some(item_list.clone());
    }

    item_list
}

/// Builds the box that displays the total amount owed and the checkout button.
/// Assignes the `Label` to a global mutable reference so that it can be accessed from other
/// modules.
fn build_amount_owed_box() -> Box {
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

    // TODO: Add functionality to checkout button
    checkout_button.connect_clicked(move |_| {
        println!("Checkout button clicked");
        let patrons = PATRONS.lock().unwrap();
        println!("Patrons: {:?}", *patrons);
        let curr_patrons = get_current_patron();
        println!("Current patron: {:?}", curr_patrons);
    });

    total_amount_box
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

pub fn get_current_patron_label() -> &'static Option<Label> {
    // Unsafe block to access the global mutable reference
    unsafe { &CURRENT_PATRON_LABEL }
}

pub fn get_current_patron_label_text() -> String {
    let current_patron_label = get_current_patron_label();
    let current_patron_label_text = current_patron_label.as_ref().unwrap().text();
    current_patron_label_text.to_string()
}

/// Updates the list of items in the transaction panel based on the current patron.
/// This function is called when a new patron is selected or when a new tab is created.
pub fn update_item_list() {
    let mut patrons = PATRONS.lock().unwrap();
    let curr_patron = patrons.iter_mut().find(|p| p.name == get_current_patron_label_text());
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
            amount_owed_label.as_ref().unwrap().set_text(&format!("Total Amount: ${:.2}", total));
        }
        None => {
            println!("update_list_item: Could not find patron");
        }
    }
}

fn get_amount_owed_label() -> &'static Option<Label> {
    // Unsafe block to access the global mutable reference
    unsafe { &AMOUNT_OWED_LABEL }
}   

pub fn get_item_list() -> &'static Option<ListBox> {
    // Unsafe block to access the global mutable reference
    unsafe { &ITEM_LIST }
}
