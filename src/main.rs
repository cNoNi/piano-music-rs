extern crate gtk;

use gtk::prelude::*;

fn build_keyboard() -> gtk::Box {
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);

    // Create a grid of buttons to represent the piano keys
    let grid = gtk::Grid::new();
    for (i, &note) in ["C", "D", "E", "F", "G", "A", "B"].iter().enumerate() {
        let white_key = gtk::Button::new_with_label(note);
        grid.attach(&white_key, i as i32, 0, 1, 1);
        let black_key = gtk::Button::new_with_label("");
        black_key.get_style_context().add_class("black-key");
        if note != "E" && note != "B" {
            grid.attach(&black_key, i as i32, 1, 1, 1);
        }
    }

    vbox.add(&grid);

    // Add a sustain pedal at the bottom of the keyboard
    let sustain_pedal = gtk::ToggleButton::new_with_label("Sustain");
    vbox.add(&sustain_pedal);

    vbox
}

fn main() {
    gtk::init().expect("GTK initialization failed");

    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Rust Piano");
    window.set_default_size(400, 200);

    let keyboard = build_keyboard();
    window.add(&keyboard);

    window.show_all();

    window.connect_deleteevent(|, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
window.connect_button_pressevent(move |, event| {
    println!("Mouse button {} was pressed at coordinates ({}, {})", event.get_button(), event.get_x(), event.get_y());
    Inhibit(false)
});
let sustain_pedal = gtk::ToggleButton::new_with_label("Sustain");
sustain_pedal.connect_toggled(move |button| {
    if button.get_active() {
        println!("Sustain pedal engaged!");
    } else {
        println!("Sustain pedal released!");
    }
});
