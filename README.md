# Rusty Nail POS System

### Created by Stuart Rimel

<br>

## Overview
---
Rusty Nail is a GTK4 GUI application that serves as a point of sales (POS) system specifically for operating in bars and restaurants. This application will provide a visual interface for the user to take orders for patrons and add to their tab. This program will also allow users to remove items from a patron's tab as well as create and select among many patrons in the system. When a patrons is "closing out" their tab, the checkout button will generate a mock receipt containing the transaction id, name, total, and credit card details for payment processing. The GUI will dynamically update itself based on user interactions with the program. The main visual component is a grid of buttons system such that on clicking will generate another grid of buttons to select specific items to add to the current patron's tab. Different modal dialogs will present to the user when they create a new tab, select a different patron, and close out the currently selected patron. 

<br>

## How to Install and Run
---
### MacOS, Linux
1. Install GTK4 using package manager ([homebrew](https://brew.sh/) for macOS)
>> Ex. `brew install gtk4` for macOS or `sudo pacman -Syu gtk4` for Arch  
>> Note: for linux use the appropriate package manager for your distro.
2. From top level project directory run `cargo run`.
### Windows
1. Install GTK4 by following [this guide](https://www.gtk.org/docs/installations/windows) from gtk.org
2. From top level project directory run `cargo run`.

<br>

## Example Screenshots:
---
Main Screen:

![main screen](https://gitlab.cecs.pdx.edu/rust-group-project/rusty_nail/-/raw/main/screenshots/main_screen.png)

Create New Tab:

![new patron](https://gitlab.cecs.pdx.edu/rust-group-project/rusty_nail/-/raw/main/screenshots/new_patron.png)

Patron Order:

![patron order](https://gitlab.cecs.pdx.edu/rust-group-project/rusty_nail/-/raw/main/screenshots/patron_order.png)

Select Patron:

![select patron](https://gitlab.cecs.pdx.edu/rust-group-project/rusty_nail/-/raw/main/screenshots/select_patron.png)

Checkout Patron:

![checkout patron](https://gitlab.cecs.pdx.edu/rust-group-project/rusty_nail/-/raw/main/screenshots/checkout_patron.png)

Mock Receipt as text file:

![mock receipt](https://gitlab.cecs.pdx.edu/rust-group-project/rusty_nail/-/raw/main/screenshots/receipt.png)

<br>

## What Worked, What Did Not
---

This project was challenging as I'm new to rust and also new to GTK development. I really struggled with sending property changed signals the proper way, so ended up using unsafe and unidomatic methods to track application state with global mutables. For example, I had to use a vector of patrons wrapped in a mutex to keep track accross the different widgets that need that information. Another issue, is I would of liked a looser coupling between the UI and the underlying logic. Ideally, I would of designed this application using the MVC design pattern, but I ran out of time to refactor and might revisit that at another time. Also, since this appliation was very UI heavy I found it challenging creating unit testing for the components. Since the underlying logic is too tightly coupled to the UI components, I couldn't really design great unit testing for this project.

