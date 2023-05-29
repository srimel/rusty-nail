use gtk::subclass::application_window;
use gtk::{prelude::*, Box, Button, Label, ListBox, ListBoxRow, Orientation};
use crate::patrons::PATRONS;
use crate::current_patron::{get_current_patron};

static mut CURRENT_PATRON_LABEL: Option<Label> = None;

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

fn build_item_list() -> ListBox {
    let item_list = ListBox::new();
    item_list.set_selection_mode(gtk::SelectionMode::None);
    item_list.set_size_request(200, 300);
    item_list.add_css_class("items-list");

    // Placeholder items
    for i in 1..=5 {
        let item_label = format!("Item {}: $5.00", i);
        let row = ListBoxRow::new();
        let label = Label::new(Some(&item_label));
        row.set_child(Some(&label));
        item_list.append(&row);
    }

    item_list
}

fn build_amount_owed_box() -> Box {
    let total_amount_box = Box::new(Orientation::Vertical, 0);
    let total_amount_label = Label::new(Some("Total Amount: $25.00"));

    total_amount_box.append(&total_amount_label);
    total_amount_box.add_css_class("total-amount-box");

    let checkout_button = Button::with_label("Close Out");
    total_amount_box.append(&checkout_button);
    checkout_button.add_css_class("btn");
    checkout_button.add_css_class("checkout-btn");

    checkout_button.connect_clicked(move |_| {
        println!("Checkout button clicked");
        let patrons = PATRONS.lock().unwrap();
        println!("Patrons: {:?}", *patrons);
        let curr_patrons = get_current_patron();
        println!("Current patron: {:?}", curr_patrons);
    });

    total_amount_box
}

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
    unsafe {
        &CURRENT_PATRON_LABEL
    }
}
