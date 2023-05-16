use gtk::{prelude::*, Box, Button, Label, ListBox, ListBoxRow, Orientation};

/// Builds the right panel of the application. This includes the list of items and the total
/// amount owed with a checkout button.
///
/// TODO: This will probably need another box containing the holder of the current bar tab.
pub fn build_transaction_panel() -> Box {
    let transaction_box = Box::new(Orientation::Vertical, 0);
    transaction_box.append(&build_item_list());
    transaction_box.append(&build_amount_owed_box());

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

    let checkout_button = Button::with_label("Close Out");
    total_amount_box.append(&checkout_button);

    total_amount_box
}
