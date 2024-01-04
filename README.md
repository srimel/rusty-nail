# Rusty Nail POS System

### Created by Stuart Rimel, 2023

<br>

## Overview
Rusty Nail is a GTK4 GUI application created for Intro to Rust class final project. This application mocks a point of sales system experience catering
to the needs of bars and restaurants. This application provides users with a visually intuitive interface for managing orders, tabs, and transactions
within a hopitality setting. 

<br>

## Features
- **Order Management:** GUI implemented with GTK4 provides easy-to-use interface allowing users to take orders from patrons.
- **Dynamic Updates:** The GUI dynamically updates in real-time based on user interactions, ensuring a seamless and responsive experience.
- **Tab Creation:** Users can create new tabs for patrons, facilitating efficient organization of orders and transactions.
- **Patron Selection:** The application allows users to create and select among multiple patrons in the system to modify their tabs.
- **Creating Menu:** Users can easily add a menu or modify their current one by modifying the menu csv file in the menu directory.
- **Grid-Based System:** The main visual component is a grid of buttons for users to dynamically navigate their menus and select items to add to a tab.
- **Checkout Process:** When a patron is ready to close their tab, the checkout button generates a mock receipt containing essential details such as transaction ID, patron name, total amount, and credit card details for seamless payment processing.


<br>

## How to Install and Run

### MacOS, Linux
1. Install GTK4 using package manager ([homebrew](https://brew.sh/) for macOS)
> Ex. `brew install gtk4` for macOS or `sudo pacman -Syu gtk4` for Arch  
> Note: for linux use the appropriate package manager for your distro.
2. From top level project directory run `cargo run`.
### Windows
1. Install GTK4 by following [this guide](https://www.gtk.org/docs/installations/windows) from gtk.org
2. From top level project directory run `cargo run`.

<br>

## Example Screenshots:

Main Screen:

![main screen](https://github.com/srimel/rusty-nail/blob/main/screenshots/main_screen.png)

Create New Tab:

![new patron](https://github.com/srimel/rusty-nail/blob/main/screenshots/new_patron.png)

Patron Order:

![patron order](https://github.com/srimel/rusty-nail/blob/main/screenshots/patron_order.png)

Select Patron:

![select patron](https://github.com/srimel/rusty-nail/blob/main/screenshots/select_patron.png)

Checkout Patron:

![checkout patron](https://github.com/srimel/rusty-nail/blob/main/screenshots/checkout_patron.png)

Mock Receipt as text file:

![mock receipt](https://github.com/srimel/rusty-nail/blob/main/screenshots/receipt.png)

<br>

